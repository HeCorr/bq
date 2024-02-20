use super::GameServerDetails;
use crate::api::util::bool_fmt;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PalworldWorld {
    pub days_l: u32,
    pub description_s: String,
    pub worldguid_s: String,
    pub create_time_l: u64,
    pub busesstats_b: bool,
    pub builduniqueid_l: u16,
    pub banticheatprotected_b: bool,
}

impl PalworldWorld {
    fn fmt(&self) -> String {
        format!(
            "\
    Days: {}
    Anti-Cheat: {}
    Description: {}
    World GUID: {}
    Creation Time: {}
        ",
            self.days_l,
            bool_fmt(self.banticheatprotected_b),
            self.description_s,
            self.worldguid_s,
            self.create_time_l,
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct PalworldServerDetails {
    pub map: String,
    pub version: String,
    pub official: bool,
    pub password: bool,
    pub palworld: PalworldWorld,
}

impl GameServerDetails for PalworldServerDetails {
    fn fmt(&self) -> String {
        format!(
            "
    Map: {}
    Version: {}
    Official: {}
    Password: {}
    {}\
        ",
            self.map,
            self.version,
            bool_fmt(self.official),
            bool_fmt(self.password),
            self.palworld.fmt(),
        )
    }
}
