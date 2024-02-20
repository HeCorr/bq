mod models;
mod util;

use self::models::{
    games::{minecraft::MinecraftServerDetails, rust::RustServerDetails, GameServerDetails},
    Player, Server, ServerStatus,
};
use anyhow::{anyhow, Result};
use reqwest::StatusCode;
use std::collections::HashMap;

pub fn query_server_info(server_id: u32, include_online_players: bool) -> Result<Server> {
    let url = format!(
        "https://api.battlemetrics.com/servers/{server_id}{}",
        include_online_players
            .then_some("?include=player")
            .unwrap_or("")
    );
    let resp = reqwest::blocking::get(url)?;

    if resp.status() == StatusCode::NOT_FOUND {
        return Err(anyhow!("Server not found."));
    }

    let data = resp.json::<HashMap<String, serde_json::Value>>()?;

    let online_players = include_online_players
        .then_some(
            data["included"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|include| include["type"].as_str().unwrap() == "player")
                .map(|player| player["attributes"]["name"].as_str().unwrap().to_owned())
                .map(|name| Player { name })
                .collect(),
        )
        .or(None);

    let game = data["data"]["relationships"]["game"]["data"]["id"]
        .as_str()
        .unwrap()
        .to_owned();

    let details: Option<Box<dyn GameServerDetails>> = match game.as_str() {
        "rust" => Some(Box::new(
            serde_json::from_value::<RustServerDetails>(
                data["data"]["attributes"]["details"].to_owned(),
            )
            .unwrap(),
        )),
        "minecraft" => Some(Box::new(
            serde_json::from_value::<MinecraftServerDetails>(
                data["data"]["attributes"]["details"].to_owned(),
            )
            .unwrap(),
        )),
        &_ => None,
    };

    let mut server =
        serde_json::from_value::<Server>(data["data"]["attributes"].to_owned()).unwrap();

    server.game = game;
    server.details = details;
    server.online_players = online_players;
    server.status = match data["data"]["attributes"]["status"].as_str().unwrap_or("") {
        "online" => ServerStatus::Online,
        "offline" => ServerStatus::Offline,
        _ => ServerStatus::Unknown,
    };
    Ok(server)
}
