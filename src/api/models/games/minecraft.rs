use super::GameServerDetails;
use crate::api::util::bool_fmt;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MinecraftVersion {
    pub name: String,
    pub protocol: u16,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftServerDetails {
    pub minecraft_clean_description: String,
    pub minecraft_version: MinecraftVersion,
    pub minecraft_modded: bool,
    pub minecraft_hash: String,
    pub minecraft_version_name: String,
}

impl GameServerDetails for MinecraftServerDetails {
    fn fmt(&self) -> String {
        format!(
            "
    Version: {} ({} [Protocol {}])
    Modded: {}
    Hash: {}
    Description: \n{}\
        ",
            self.minecraft_version_name,
            self.minecraft_version.name,
            self.minecraft_version.protocol,
            bool_fmt(self.minecraft_modded),
            self.minecraft_hash,
            self.minecraft_clean_description,
        )
    }
}
