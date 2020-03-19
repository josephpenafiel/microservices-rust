pub use chrono::Utc;
pub use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
pub use failure::{format_err, Error};
pub use rusoto_core::Region;
pub use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, QueryInput, UpdateItemInput};
pub use std::collections::HashMap;

#[derive(Debug)]
pub struct Location {
    pub user_id: String,
    pub timestamp: String,
    pub longitude: String,
    pub latitude: String,
}

impl Location {
    pub fn from_map(map: HashMap<String, AttributeValue>) -> Result<Location, Error> {
        let user_id = map
            .get("Uid")
            .ok_or_else(|| format_err!("No Uid in record!"))
            .and_then(attr_to_string)?;
        let timestamp = map
            .get("TimeStamp")
            .ok_or_else(|| format_err!("No timeStamp in record!"))
            .and_then(attr_to_string)?;
        let latitude = map
            .get("Latitude")
            .ok_or_else(|| format_err!("No Latitude in record"))
            .and_then(attr_to_string)?;
        let longitude = map
            .get("Longitude")
            .ok_or_else(|| format_err!("No Longitude in record"))
            .and_then(attr_to_string)?;
        let location = Location {
            user_id,
            timestamp,
            longitude,
            latitude,
        };

        Ok(location)
    }
}

fn attr_to_string(attr: &AttributeValue) -> Result<String, Error> {
    if let Some(value) = &attr.s {
        Ok(value.to_owned())
    } else {
        Err(format_err!("No string value"))
    }
}

pub async fn add_location(conn: &DynamoDbClient, location: Location) -> Result<(), Error> {
    let mut key = HashMap::new();
    key.insert("Uid".into(), s_attr(location.user_id));
    key.insert("TimeStamp".into(), s_attr(location.timestamp));
    let expr = format!("SET Latitude = :y, Longitude = :x");
    let mut values = HashMap::new();
    values.insert(":y".into(), s_attr(location.latitude));
    values.insert(":x".into(), s_attr(location.longitude));
    let update = UpdateItemInput {
        table_name: "Locations".into(),
        key,
        update_expression: Some(expr),
        expression_attribute_values: Some(values),
        ..Default::default()
    };
    conn.update_item(update)
        .await
        .map(drop)
        .map_err(Error::from)
}

pub async fn list_locations(
    conn: &DynamoDbClient,
    user_id: String,
) -> Result<Vec<Location>, Error> {
    let expr = format!("Uid = :uid");
    let mut values = HashMap::new();
    values.insert(":uid".into(), s_attr(user_id));
    let query = QueryInput {
        table_name: "Locations".into(),
        key_condition_expression: Some(expr),
        expression_attribute_values: Some(values),
        ..Default::default()
    };
    let items = conn
        .query(query)
        .await?
        .items
        .ok_or_else(|| format_err!("No Items"))?;
    let mut locations = Vec::new();
    for i in items {
        let location = Location::from_map(i)?;
        locations.push(location);
    }

    Ok(locations)
}

fn s_attr(s: String) -> AttributeValue {
    AttributeValue {
        s: Some(s),
        ..Default::default()
    }
}
