use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use postgres::{Connection, Error, TlsMode};
mod postgres_helpers;

const CMD_CREATE: &str = "create";
const CMD_ADD: &str = "add";
const CMD_LIST: &str = "list";
fn main() -> Result<(), Error> {
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
        .get_matches();
    let addr = matches
        .value_of("database")
        .unwrap_or("postgres://postgres:perrito1@localhost:5432");

    let conn = Connection::connect(addr, TlsMode::None)?;

    match matches.subcommand() {
        (CMD_CREATE, _) => {
            postgres_helpers::create_table(&conn)?;
        }
        (CMD_ADD, Some(matches)) => {
            let name = matches.value_of("NAME").unwrap();
            let email = matches.value_of("EMAIL").unwrap();
            postgres_helpers::create_user(&conn, name, email)?;
        }
        (CMD_LIST, _) => {
            let list = postgres_helpers::list_users(&conn)?;
            for (name, email) in list {
                println!("Name:{:5}    Email:{:5}", name, email);
            }
        }
        _ => {
            matches.usage();
        }
    }
    Ok(())
}
