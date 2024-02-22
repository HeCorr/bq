use super::GameServerDetails;
use crate::api::util::{bool_fmt, tf2_gamemode_fmt, BoolType};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TF2Rules {
    pub coop: String,
    pub deathmatch: String,
    pub decalfrequency: String,
    pub tf_medieval: String,
    pub tf_playergib: String,
    pub tf_weapon_criticals: String,
    pub tf_weapon_criticals_melee: String,
}

impl TF2Rules {
    fn fmt(&self) -> String {
        format!(
            "\
    Co-Op: {}
    Deathmatch: {}
    Decal Frequency: {}
    Medieval: {}
    Player Gib: {}
    Melee Criticals: {}
    Weapon Criticals: {}\
        ",
            self.coop,
            self.deathmatch,
            self.decalfrequency,
            self.tf_medieval,
            self.tf_playergib,
            self.tf_weapon_criticals_melee,
            self.tf_weapon_criticals,
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct TF2ServerDetails {
    pub map: String,
    pub password: bool,
    pub tags: String,
    pub numbots: u16,
    #[serde(rename = "gameMode")] // whyyyy
    pub gamemode: Option<String>,
    pub rules: TF2Rules,
}

impl GameServerDetails for TF2ServerDetails {
    fn fmt(&self) -> String {
        format!(
            "
    Map: {}
    Password: {}
    Tags: {}
    Num. Bots: {}
    Gamemode: {}
    {}\
        ",
            self.map,
            bool_fmt(self.password, BoolType::YesNo),
            self.tags,
            self.numbots,
            self.gamemode
                .clone()
                .map(|x| tf2_gamemode_fmt(&x))
                .unwrap_or("<Unknown>".into()),
            self.rules.fmt(),
        )
    }
}
