pub mod games;

use crate::api::util::{bool_fmt, game_fmt};
use self::games::RustServerDetails;

#[derive(Debug)]
pub struct Player {
    pub name: String,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub enum ServerStatus {
    Online,
    Offline,
    Unknown,
}

pub struct Server {
    pub game: String,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub address: Option<String>,
    pub players: u16,
    pub max_players: u16,
    pub rank: u16,
    pub status: ServerStatus,
    pub private: bool,
    pub country: String,
    pub online_players: Option<Vec<Player>>,
    pub rust_details: Option<RustServerDetails>,
}

impl Server {
    pub fn fmt(&self, full: bool) -> String {
        if full {
            format!(
                "{}:\n  Game: {}\n  Status: {:?}\n  Players: {}/{}\n  Country: {}\n  IP/Port: {}:{}\n  Address: {}\n  Private: {}\n  Rank: #{}\n  Details: {:#?}",
                self.name,
                game_fmt(&self.game),
                self.status, 
                self.players, self.max_players,
                self.country,
                self.ip, self.port,
                self.address.clone().unwrap_or("<None>".to_owned()),
                bool_fmt(self.private),
                self.rank,
                self.rust_details,
            )
        } else {
            format!(
                "{}:\n  Game: {}\n  Status: {:?}\n  Players: {}/{}\n  IP/Port: {}:{}",
                self.name, game_fmt(&self.game), self.status, self.players, self.max_players, self.ip, self.port
            )
        }
    }
}
