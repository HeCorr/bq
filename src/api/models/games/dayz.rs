use super::GameServerDetails;
use crate::api::util::{bool_fmt, BoolType};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DayZServerDetails {
    pub version: String,
    pub password: bool,
    pub official: bool,
    pub time: String,
    pub third_person: bool,
    pub modded: bool,
    #[serde(rename = "modIds")]
    pub mod_ids: Vec<u32>,
    #[serde(rename = "modNames")]
    pub mod_names: Vec<String>,
    #[serde(rename = "serverSteamId")]
    pub server_steam_id: String,
}

impl GameServerDetails for DayZServerDetails {
    fn fmt(&self) -> String {
        format!(
            "
    Version: {}
    Password Protected: {}
    Official Server: {}
    Time: {}
    Third Person: {}
    Modded: {}
    Mods ({}): {}\
        ",
            self.version,
            bool_fmt(self.password, BoolType::YesNo),
            bool_fmt(self.official, BoolType::YesNo),
            self.time,
            bool_fmt(self.third_person, BoolType::AllowedDisabled),
            bool_fmt(self.modded, BoolType::YesNo),
            self.mod_names.len(),
            self.mod_names
                .is_empty()
                .then(|| "<None>".to_owned())
                .unwrap_or_else(|| format!("\n      {}", self.mod_names.join("\n      "))),
        )
    }
}
