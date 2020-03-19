pub use crate::activity::Activity;
pub use bson::{bson, doc};
pub use chrono::offset::Utc;
pub use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
pub use mongodb::{
    db::{Database, ThreadedDatabase},
    Error,
};
pub use r2d2::Pool;
pub use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
pub use url::Url;

pub fn add_activity(conn: &Database, activity: Activity) -> Result<(), Error> {
    let d = doc! {
        "user_id": activity.user_id,
        "activity": activity.activity,
        "datetime": activity.datetime,
    };
    let coll = conn.collection("activities");
    coll.insert_one(d, None).map(drop)
}

pub fn list_activities(conn: &Database) -> Result<Vec<Activity>, Error> {
    conn.collection("activities")
        .find(None, None)?
        .try_fold(Vec::new(), |mut vec, doc| {
            let doc = doc?;
            let activity: Activity = bson::from_bson(bson::Bson::Document(doc))?;
            vec.push(activity);
            Ok(vec)
        })
}
