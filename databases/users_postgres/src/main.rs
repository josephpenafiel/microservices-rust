use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use postgres::{Connection, Error};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use rayon::prelude::*;
mod postgres_helpers;

const CMD_CREATE: &str = "create";
const CMD_ADD: &str = "add";
const CMD_LIST: &str = "list";
const CMD_IMPORT: &str = "import";

fn main() -> Result<(), failure::Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("db")
                .value_name("ADDR")
                .help("Sets an address of db connection")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name(CMD_CREATE).about("create users table"))
        .subcommand(
            SubCommand::with_name(CMD_ADD)
                .about("add user to the table")
                .arg(
                    Arg::with_name("NAME")
                        .help("Sets the name of a user")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("EMAIL")
                        .help("Sets the email of a user")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(SubCommand::with_name(CMD_LIST).about("print list users"))
        .subcommand(SubCommand::with_name(CMD_IMPORT).about("import users from .csv file"))
        .get_matches();
    let addr = matches
        .value_of("database")
        .unwrap_or("postgres://postgres:perrito1@localhost:5432");
    let manager = PostgresConnectionManager::new(addr, TlsMode::None)?;
    let pool = r2d2::Pool::new(manager)?;
    let conn = pool.get()?;

    match matches.subcommand() {
        (CMD_CREATE, _) => {
            postgres_helpers::create_table(&conn)?;
        }
        (CMD_ADD, Some(matches)) => {
            let name = matches.value_of("NAME").unwrap().to_owned();
            let email = matches.value_of("EMAIL").unwrap().to_owned();
            let user = postgres_helpers::User { name, email };
            postgres_helpers::create_user(&conn, &user)?;
        }
        (CMD_LIST, _) => {
            let list = postgres_helpers::list_users(&conn)?;
            for user in list {
                println!("Name:{:5}    Email:{:5}", user.name, user.email);
            }
        }
        (CMD_IMPORT, _) => {
            let mut rdr = csv::Reader::from_reader(std::io::stdin());
            let mut users = Vec::new();
            for user in rdr.deserialize() {
                users.push(user?);
            }

            users
                .par_iter() // rayon prelude
                .map(|user| -> Result<(), failure::Error> {
                    let conn = pool.get()?;
                    postgres_helpers::create_user(&conn, &user)?;
                    Ok(())
                })
                .for_each(drop);
        }
        _ => {
            matches.usage();
        }
    }
    Ok(())
}
