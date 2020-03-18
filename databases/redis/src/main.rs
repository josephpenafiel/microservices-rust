mod helpers;
mod prelude;
use helpers::*;
use prelude::*;

const SESSIONS: &str = "sessions";
const CMD_ADD: &str = "add";
const CMD_REMOVE: &str = "remove";
const CMD_LIST: &str = "list";

// docker run -it --rm --name test-redis -p 6379:6379 redis
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
        .subcommand(
            SubCommand::with_name(CMD_ADD)
                .about("add a session")
                .arg(
                    Arg::with_name("TOKEN")
                        .help("Sets the token of a user")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("UID")
                        .help("Sets the uid of a user")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_REMOVE)
                .about("remove a session")
                .arg(
                    Arg::with_name("TOKEN")
                        .help("Sets the token of a user")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(SubCommand::with_name(CMD_LIST).about("print list of sessions"))
        .get_matches();

    let addr = matches.value_of("database").unwrap_or("redis://127.0.0.1/");
    let manager = RedisConnectionManager::new(addr)?;
    let pool = r2d2::Pool::new(manager)?;
    let conn = pool.get()?;

    match matches.subcommand() {
        (CMD_ADD, Some(matches)) => {
            let token = matches.value_of("TOKEN").unwrap();
            let uid = matches.value_of("UID").unwrap();
            add_session(&conn, token, uid)?;
        }
        (CMD_REMOVE, Some(matches)) => {
            let token = matches.value_of("TOKEN").unwrap();
            remove_session(&conn, token)?;
        }
        (CMD_LIST, _) => {
            println!("LIST!");
            let sessions = list_sessions(&conn)?;
            for (token, uid) in sessions {
                println!("Token: {:20}  Uid: {:20}", token, uid);
            }
        }
        _ => {
            matches.usage();
        }
    }

    Ok(())
}
