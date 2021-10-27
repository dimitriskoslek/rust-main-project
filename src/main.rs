use std::process::Command;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[path = "lib/email.rs"]
mod email;
#[path = "lib/error.rs"]
mod error;
#[path = "lib/init.rs"]
mod init;
#[path = "db/connect.rs"]
mod db_connect;
#[path = "lib/routes.rs"]
mod routes;
#[path = "lib/environment.rs"]
mod environment;



use rocket::*;


#[launch]
async fn rocket() -> _ {

    // get environment variables
    let environment_variables: environment::EnvVars = environment::get_environment_variables();
    dbg!(environment_variables.db_url);

    // email
    //email::send_email();

    // connect to mongo
    //db_connect::connect().await;

    // set loggers
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.) will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    /*
    let number_shaved = yak_shave::shave_all(number_of_yaks);
    info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );
    */

    /*
    tracing::subscriber::with_default(subscriber, || {
        info!("This will be logged to stdout");
    });
    info!("This will _not_ be logged to stdout");
    */

    // Bash command
    let output = Command::new("echo")
        .arg("Hello world")
        .output()
        .expect("Failed to execute command");
    assert_eq!("Hello world\n", String::from_utf8_lossy(output.stdout.as_slice()));
    println!("The command output is {}", String::from_utf8_lossy(output.stdout.as_slice()));

    // start rocket
    rocket::build()
        .mount("/", routes![routes::index])
        .mount("/hello", routes![routes::world])
        .mount("/", routes![routes::delay])
        .mount("/test", routes![routes::get_all_users])
}



//     // CLI TOOL
//     use clap::{App, Arg, SubCommand};
//     let matches = App::new("Sociality Internal Tool")
//         .version("1.0")
//         .author("Dimitris Kosmas-Lekkas <dkl1994@gmail.com>")
//         .about("Sociality tool for internal use")
//         .subcommand(SubCommand::with_name("init")
//             .about("Initialize the Tool"))
//         .subcommand(SubCommand::with_name("test")
//             .about("Test the tool's integrity"))
//         .subcommand(SubCommand::with_name("check")
//             .about("Server checks")
//             .arg(
//                 Arg::with_name("servers")
//                     .long("servers")
//                     .value_name("SERVERS")
//                     .help("Check our servers for issues")
//                     .takes_value(true)
//             )
//         )
//         .get_matches();

//     match matches.subcommand() {
//         ("check", Some(..)) => {
//             match init::check() {
//                 Ok(())  => println!("Checked!"),
//                 Err(..) => println!("The checks went awry!"),
//             }
//         }
//         ("init", Some(..)) => {
//             match init::init() {
//                 Ok(())  => println!("Tool initialized"),
//                 Err(..) => println!("Already initialized!"),
//             }
//         }
//         ("test", Some(..)) => {
//             match init::test() {
//                 Ok(())  => println!("Internal logistics scanned."),
//                 Err(..) => println!("Something in the scans went awry!"),
//             }
//         }
//         _ => println!("Command not recognized.")
//     }