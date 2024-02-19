mod models;

use anyhow::{anyhow, Result};
use reqwest::StatusCode;
use std::collections::HashMap;

pub use self::models::{Player, Server, ServerStatus};

pub fn query_server_info(server_id: u32) -> Result<Server> {
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

pub fn query_server_players(server_id: u32) -> Result<Vec<Player>> {
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
