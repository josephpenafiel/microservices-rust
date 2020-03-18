mod activity;
mod prelude;
use prelude::*;
const CMD_ADD: &str = "add";
const CMD_LIST: &str = "list";
fn main() {
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
                .about("add user to the table")
                .arg(
                    Arg::with_name("USER_ID")
                        .help("Sets the id of a user")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("ACTIVITY")
                        .help("Sets the activity of a user")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(SubCommand::with_name(CMD_LIST).about("print activities list of users"))
        .get_matches();
}
