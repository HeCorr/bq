mod api;

use api::query_server_info;
use clap::{arg, Arg, ArgAction, Command};
use crossterm::{cursor::MoveLeft, execute, style::Print};
use std::io::stdout;

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
                .subcommand_required(true)
                .subcommand(
                    Command::new("player-list")
                        .visible_alias("pl")
                        .about("List players connected to server"),
                )
                .subcommand(
                    Command::new("info")
                        .visible_alias("i")
                        .about("Print server info")
                        .arg(
                            Arg::new("full")
                                .short('f')
                                .long("full")
                                .help("Display all information")
                                .action(ArgAction::SetTrue),
                        ),
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
                execute!(stdout(), Print("Fetching server info...")).unwrap();
                let info = query_server_info(srv_id, true);
                execute!(stdout(), MoveLeft(23), Print(" ".repeat(23)), MoveLeft(23)).unwrap();
                match info {
                    Ok(info) => {
                        let players = info.online_players.unwrap();
                        println!("{}:\n  Online players ({}):", info.name, players.len());
                        players.iter().for_each(|p| println!("    {p}"));
                    }
                    Err(e) => println!("Error: {:?}", e),
                };
            }
            Some(("info", info_matches)) => {
                let srv_id = server_matches
                    .get_one::<String>("ID")
                    .expect("required")
                    .parse::<u32>()
                    .unwrap();
                let full = info_matches.get_flag("full");
                execute!(stdout(), Print("Fetching server info...")).unwrap();
                let info = query_server_info(srv_id, false);
                execute!(stdout(), MoveLeft(23), Print(" ".repeat(23)), MoveLeft(23)).unwrap();
                match info {
                    Ok(info) => println!("{}", info.fmt(full)),
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
