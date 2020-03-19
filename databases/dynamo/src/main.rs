// docker run -it --rm --name test-dynamodb -p 8000:8000 amazon/dynamodb-local
// aws dynamodb create-table --cli-input-json file://table.json --endpoint-url http://localhost:8000 --region custom
mod prelude;
use prelude::*;
const CMD_ADD: &str = "add";
const CMD_LIST: &str = "list";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("region")
                .long("region")
                .value_name("REGION")
                .help("Sets a region")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("endpoint")
                .long("endpoint-url")
                .value_name("URL")
                .help("Sets an endpoint url")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name(CMD_ADD)
                .about("add geo record to the table")
                .arg(
                    Arg::with_name("USER_ID")
                        .help("Sets id of a user")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("LATITUDE")
                        .help("Sets a latitude of location")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("LONGITUDE")
                        .help("Sets a longitude of location")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_LIST)
                .about("print all records for the user")
                .arg(
                    Arg::with_name("USER_ID")
                        .help("Userid to filter records")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();
    let region = matches
        .value_of("endpoint")
        .map(|e| Region::Custom {
            name: "custom".into(),
            endpoint: e.into(),
        })
        .ok_or_else(|| format_err!("Region not set"))
        .or_else(|_| matches.value_of("region").unwrap_or("us-east-1").parse())?;
    let client = DynamoDbClient::new(region);
    match matches.subcommand() {
        (CMD_ADD, Some(m)) => {
            let user_id = m.value_of("USER_ID").unwrap().to_owned();
            let timestamp = Utc::now().to_string();
            let latitude = m.value_of("LATITUDE").unwrap().to_owned();
            let longitude = m.value_of("LONGITUDE").unwrap().to_owned();
            let location = Location {
                user_id,
                timestamp,
                latitude,
                longitude,
            };
            add_location(&client, location).await?;
        }
        (CMD_LIST, Some(m)) => {
            let user_id = m.value_of("USER_ID").unwrap().to_owned();
            let locations = list_locations(&client, user_id).await?;
            for l in locations {
                println!("{:?}", l);
            }
        }
        _ => {
            matches.usage();
        }
    }
    Ok(())
}
