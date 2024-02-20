use super::GameServerDetails;
use crate::api::util::bool_fmt;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RustServerDetails {
    pub pve: bool,
    pub map: String,
    pub official: bool,
    pub rust_type: String,
    pub rust_build: String,
    pub rust_ent_cnt_i: i64,
    pub rust_fps: u8,
    pub rust_fps_avg: f32,
    pub rust_gc_cl: i32,
    pub rust_gc_mb: i32,
    pub rust_hash: String,
    pub rust_headerimage: String,
    pub rust_uptime: u32,
    pub rust_url: String,
    pub rust_world_seed: i64,
    pub rust_world_size: u16,
    pub rust_world_levelurl: String,
    pub rust_description: String,
    pub rust_modded: bool,
    pub rust_queued_players: u16,
    pub rust_gamemode: String,
}

impl GameServerDetails for RustServerDetails {
    fn fmt(&self) -> String {
        format!(
            "
    PVE: {}
    Map: {}
    Official: {}
    Type: {}
    Build: {}
    FPS: {}
    Avg. FPS: {}
    Hash: {}
    Uptime: {}
    URL: {}
    World Seed: {}
    World Size: {}
    World Level: {}
    Modded: {}
    Queued Players: {}
    Gamemode: {}
    Description: {}\
        ",
            bool_fmt(self.pve),
            self.map,
            bool_fmt(self.official),
            self.rust_type,
            self.rust_build,
            self.rust_fps,
            self.rust_fps_avg,
            self.rust_hash,
            self.rust_uptime,
            self.rust_url,
            self.rust_world_seed,
            self.rust_world_size,
            self.rust_world_levelurl,
            bool_fmt(self.rust_modded),
            self.rust_queued_players,
            self.rust_gamemode,
            self.rust_description.replace("\\t", "\t"),
        )
    }
}
