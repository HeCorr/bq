mod api;

use api::{query_server_info, query_server_players};
use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("bq")
        .about("Battlemetrics Query")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("server")
                .visible_alias("s")
                .about("Server operations")
                .arg(arg!(<ID> "The server ID"))
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("player-list")
                        .visible_alias("pl")
                        .about("List players connected to server"),
                )
                .subcommand(
                    Command::new("info")
                        .visible_alias("i")
                        .about("Print server info"),
                ),
        )
}
fn main() {
    let cli = cli();
    let matches = cli.clone().get_matches();

    match matches.subcommand() {
        Some(("server", server_matches)) => match server_matches.subcommand() {
            Some(("player-list", _)) => {
                let srv_id = server_matches
                    .get_one::<String>("ID")
                    .expect("required")
                    .parse::<u32>()
                    .unwrap();
                println!("Fetching server info for {}...", srv_id);
                let players = query_server_players(srv_id);
                match players {
                    Ok(players) => {
                        println!("Online players ({}):", players.len());
                        players.iter().for_each(|p| println!("{p}"));
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                };
            }
            Some(("info", _)) => {
                let srv_id = server_matches
                    .get_one::<String>("ID")
                    .expect("required")
                    .parse::<u32>()
                    .unwrap();
                println!("Fetching server info for {}...", srv_id);
                let info = query_server_info(srv_id);
                match info {
                    Ok(info) => println!("{info}"),
                    Err(e) => println!("Error: {:?}", e),
                };
            }
            Some((&_, _)) => todo!(),
            None => todo!(),
        },
        Some((&_, _)) => todo!(),
        None => todo!(),
    };
}
