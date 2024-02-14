use anyhow::{anyhow, Result};
use clap::{arg, Command};
use reqwest::StatusCode;
use std::collections::HashMap;

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

#[derive(Debug)]
struct Player {
    name: String,
}

#[derive(Debug)]
enum ServerStatus {
    Online,
    Offline,
    Unknown,
}

struct Server {
    name: String,
    ip: String,
    port: u16,
    players: u16,
    max_players: u16,
    rank: u16,
    status: ServerStatus,
    official: bool,
    map: String,
    pve: bool,
    description: String,
    modded: bool,
    private: bool,
    country: String,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\n  Status: {:?}\n  Players: {}/{}\n  IP/Port: {}:{}",
            self.name, self.status, self.players, self.max_players, self.ip, self.port
        )
    }
}

fn query_server_info(server_id: u32) -> Result<Server> {
    let url = format!("https://api.battlemetrics.com/servers/{server_id}");
    let resp = reqwest::blocking::get(url)?;

    if resp.status() == StatusCode::NOT_FOUND {
        return Err(anyhow!("Server not found."));
    }

    let data = resp.json::<HashMap<String, serde_json::Value>>()?;

    Ok(Server {
        name: data["data"]["attributes"]["name"]
            .as_str()
            .unwrap_or("")
            .to_owned(),
        ip: data["data"]["attributes"]["ip"]
            .as_str()
            .unwrap_or("")
            .to_owned(),
        port: u16::try_from(data["data"]["attributes"]["port"].as_u64().unwrap_or(0))?,
        players: u16::try_from(data["data"]["attributes"]["players"].as_u64().unwrap_or(0))?,
        max_players: u16::try_from(
            data["data"]["attributes"]["maxPlayers"]
                .as_u64()
                .unwrap_or(0),
        )?,
        rank: u16::try_from(data["data"]["attributes"]["rank"].as_u64().unwrap_or(0))?,
        status: match data["data"]["attributes"]["status"].as_str().unwrap_or("") {
            "online" => ServerStatus::Online,
            "offline" => ServerStatus::Offline,
            _ => ServerStatus::Unknown,
        },
        official: data["data"]["attributes"]["details"]["official"]
            .as_bool()
            .unwrap_or(false),
        map: data["data"]["attributes"]["details"]["map"]
            .as_str()
            .unwrap_or("")
            .to_owned(),
        pve: data["data"]["attributes"]["details"]["pve"]
            .as_bool()
            .unwrap_or(false),
        description: data["data"]["attributes"]["details"]["rust_description"]
            .as_str()
            .unwrap_or("")
            .to_owned(),
        modded: data["data"]["attributes"]["details"]["rust_modded"]
            .as_bool()
            .unwrap_or(false),
        private: data["data"]["attributes"]["private"]
            .as_bool()
            .unwrap_or(false),
        country: data["data"]["attributes"]["country"]
            .as_str()
            .unwrap_or("")
            .to_owned(),
    })
}

fn query_server_players(server_id: u32) -> Result<Vec<Player>> {
    let url = format!("https://api.battlemetrics.com/servers/{server_id}?include=player");
    let resp = reqwest::blocking::get(url)?;

    if resp.status() == StatusCode::NOT_FOUND {
        return Err(anyhow!("Server not found."));
    }

    let data = resp.json::<HashMap<String, serde_json::Value>>()?;

    let players = data["included"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|include| include["type"].as_str().unwrap() == "player")
        .map(|player| player["attributes"]["name"].as_str().unwrap().to_owned())
        .map(|name| Player { name })
        .collect();

    Ok(players)
}
