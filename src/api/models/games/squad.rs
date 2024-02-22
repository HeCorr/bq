use super::GameServerDetails;
use crate::api::util::{bool_fmt, BoolType};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SquadServerDetails {
    pub map: String,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    pub version: String,
    #[serde(rename = "licensedServer")]
    pub licensed_server: bool,
    #[serde(rename = "licenseId")]
    pub license_id: Option<String>,
    pub password: bool,
    #[serde(rename = "squad_playerReserveCount")]
    pub player_reserve_count: Option<u16>,
    #[serde(rename = "squad_playTime")]
    pub play_time: u32,
    #[serde(rename = "squad_publicQueueLimit")]
    pub public_queue_limit: Option<u16>,
    #[serde(rename = "squad_publicQueue")]
    pub public_queue: Option<u16>,
    #[serde(rename = "squad_reservedQueue")]
    pub reserved_queue: Option<u16>,
    #[serde(rename = "squad_teamOne")]
    pub team_one: Option<String>,
    #[serde(rename = "squad_teamTwo")]
    pub team_two: Option<String>,
    #[serde(rename = "squad_nextLayer")]
    pub next_layer: Option<String>,
}

impl GameServerDetails for SquadServerDetails {
    fn fmt(&self) -> String {
        format!(
            "
    Map: {}
    Game Mode: {}
    Version: {}
    Licensed Server: {}
    License ID: {}
    Password Protected: {}\
        ",
            self.map,
            self.game_mode,
            self.version,
            bool_fmt(self.licensed_server, BoolType::YesNo),
            self.license_id.clone().unwrap_or("<None>".to_owned()),
            bool_fmt(self.password, BoolType::YesNo),
        )
    }
}
