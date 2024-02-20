#[derive(Debug)]
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

pub trait GameServerDetails {
    fn fmt(&self) -> String;
}
