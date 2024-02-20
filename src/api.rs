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
        "rust" => Some(Box::new(RustServerDetails {
            rust_type: data["data"]["attributes"]["details"]["rust_type"]
                .as_str()
                .unwrap()
                .to_owned(),
            rust_build: data["data"]["attributes"]["details"]["rust_build"]
                .as_str()
                .unwrap()
                .to_owned(),
            rust_ent_cnt_i: data["data"]["attributes"]["details"]["rust_ent_cnt_i"]
                .as_i64()
                .unwrap(),
            rust_fps: u8::try_from(
                data["data"]["attributes"]["details"]["rust_fps"]
                    .as_u64()
                    .unwrap_or(0),
            )
            .unwrap(),
            rust_fps_avg: data["data"]["attributes"]["details"]["rust_fps_avg"]
                .as_f64()
                .unwrap_or(0.0) as f32,
            rust_gc_cl: i32::try_from(
                data["data"]["attributes"]["details"]["rust_gc_cl"]
                    .as_i64()
                    .unwrap_or(0),
            )
            .unwrap(),
            rust_gc_mb: i32::try_from(
                data["data"]["attributes"]["details"]["rust_gc_mb"]
                    .as_i64()
                    .unwrap_or(0),
            )
            .unwrap(),
            rust_hash: data["data"]["attributes"]["details"]["rust_hash"]
                .as_str()
                .unwrap()
                .to_owned(),
            rust_headerimage: data["data"]["attributes"]["details"]["rust_headerimage"]
                .as_str()
                .unwrap()
                .to_owned(),
            rust_uptime: u32::try_from(
                data["data"]["attributes"]["details"]["rust_uptime"]
                    .as_u64()
                    .unwrap_or(0),
            )
            .unwrap(),
            rust_url: data["data"]["attributes"]["details"]["rust_url"]
                .as_str()
                .unwrap()
                .to_owned(),
            rust_world_seed: data["data"]["attributes"]["details"]["rust_world_seed"]
                .as_i64()
                .unwrap(),
            rust_world_size: u16::try_from(
                data["data"]["attributes"]["details"]["rust_world_size"]
                    .as_u64()
                    .unwrap_or(0),
            )
            .unwrap(),
            rust_world_levelurl: data["data"]["attributes"]["details"]["rust_world_levelurl"]
                .as_str()
                .unwrap()
                .to_owned(),
            rust_description: data["data"]["attributes"]["details"]["rust_description"]
                .as_str()
                .unwrap()
                .to_owned(),
            rust_modded: data["data"]["attributes"]["details"]["rust_modded"]
                .as_bool()
                .unwrap_or(false),
            rust_queued_players: u16::try_from(
                data["data"]["attributes"]["details"]["rust_queued_players"]
                    .as_u64()
                    .unwrap_or(0),
            )
            .unwrap(),
            rust_gamemode: data["data"]["attributes"]["details"]["rust_gamemode"]
                .as_str()
                .unwrap()
                .to_owned(),
            pve: data["data"]["attributes"]["details"]["pve"]
                .as_bool()
                .unwrap_or(false),
            map: data["data"]["attributes"]["details"]["map"]
                .as_str()
                .unwrap_or("")
                .to_owned(),
            official: data["data"]["attributes"]["details"]["official"]
                .as_bool()
                .unwrap_or(false),
        })),
        "minecraft" => {
            let det: MinecraftServerDetails =
                serde_json::from_value(data["data"]["attributes"]["details"].to_owned()).unwrap();
            Some(Box::new(det))
        }
        &_ => None,
    };

    Ok(Server {
        game,
        name: data["data"]["attributes"]["name"]
            .as_str()
            .unwrap_or("")
            .to_owned(),
        ip: data["data"]["attributes"]["ip"]
            .as_str()
            .unwrap_or("")
            .to_owned(),
        port: u16::try_from(data["data"]["attributes"]["port"].as_u64().unwrap_or(0))?,
        address: data["data"]["attributes"]["address"]
            .as_str()
            .map(|x| x.to_owned()),
        players: u32::try_from(data["data"]["attributes"]["players"].as_u64().unwrap_or(0))?,
        max_players: u32::try_from(
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
        private: data["data"]["attributes"]["private"]
            .as_bool()
            .unwrap_or(false),
        country: data["data"]["attributes"]["country"]
            .as_str()
            .unwrap_or("")
            .to_owned(),
        online_players,
        details,
    })
}
