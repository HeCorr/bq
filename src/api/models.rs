pub mod games;

use crate::api::util::{bool_fmt, game_fmt, BoolType};
use self::games::GameServerDetails;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
    Dead,
    Unknown,
}

impl Default for ServerStatus {
    fn default() -> Self {
        ServerStatus::Unknown
    }
}

#[derive(Deserialize)]
pub struct Server {
    #[serde(skip)]
    pub game: String,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub address: Option<String>,
    pub players: u32,
    #[serde(rename = "maxPlayers")]
    pub max_players: u32,
    pub rank: u32,
    #[serde(skip)]
    pub status: ServerStatus,
    pub private: bool,
    pub country: String,
    #[serde(skip)]
    pub online_players: Option<Vec<Player>>,
    #[serde(skip)]
    pub details: Option<Box<dyn GameServerDetails>>,
}

impl Server {
    pub fn fmt(&self, full: bool) -> String {
        if full {
            format!(
                "{}:\n  Game: {}\n  Status: {:?}\n  Players: {}/{}\n  Country: {}\n  IP/Port: {}:{}\n  Address: {}\n  Private: {}\n  Rank: #{}\n  Details: {}",
                self.name,
                game_fmt(&self.game),
                self.status, 
                self.players, self.max_players,
                self.country,
                self.ip, self.port,
                self.address.clone().unwrap_or("<None>".to_owned()),
                bool_fmt(self.private, BoolType::YesNo),
                self.rank,
                self.details.as_ref().map(|x| x.fmt()).unwrap_or("<None>".into())
            )
        } else {
            format!(
                "{}:\n  Game: {}\n  Status: {:?}\n  Players: {}/{}\n  IP/Port: {}:{}",
                self.name, game_fmt(&self.game), self.status, self.players, self.max_players, self.ip, self.port
            )
        }
    }
}
