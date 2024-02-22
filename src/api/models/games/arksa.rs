use super::GameServerDetails;
use crate::api::util::{bool_fmt, BoolType};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ArkSAServerDetails {
    pub version: String,
    pub pve: bool,
    pub official: bool,
    pub password: bool,
    pub time_i: u32,
    pub map: String,
    #[serde(rename = "modIds")]
    pub mod_ids: Vec<u32>,
    #[serde(rename = "modNames")]
    pub mod_names: Vec<String>,
    #[serde(rename = "modLinks")]
    pub mod_links: Vec<String>,
    pub modded: bool,
    pub crossplay: bool,
    pub arksa_console: bool,
    pub arksa_platforms: Vec<String>,
}

impl GameServerDetails for ArkSAServerDetails {
    fn fmt(&self) -> String {
        format!(
            "
    Version: {}
    PVE: {}
    Official: {}
    Password-protected: {}
    In-game Day: {}
    Map: {}
    Console: {}
    Crossplay: {}
    Platforms: {}
    Modded: {}
    Mods: {}\
        ",
            self.version,
            bool_fmt(self.pve, BoolType::YesNo),
            bool_fmt(self.official, BoolType::YesNo),
            bool_fmt(self.password, BoolType::YesNo),
            self.time_i,
            self.map,
            bool_fmt(self.arksa_console, BoolType::YesNo),
            bool_fmt(self.crossplay, BoolType::YesNo),
            self.arksa_platforms
                .is_empty()
                .then(|| "<None>".to_owned())
                .unwrap_or_else(|| self.arksa_platforms.join(", ")),
            bool_fmt(self.modded, BoolType::YesNo),
            self.mod_names
                .is_empty()
                .then(|| "<None>".to_owned())
                .unwrap_or_else(|| format!("\n      {}", self.mod_names.join("\n      "))),
        )
    }
}
