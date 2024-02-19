use crate::api::util::bool_fmt;

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
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub players: u16,
    pub max_players: u16,
    pub rank: u16,
    pub status: ServerStatus,
    pub official: bool,
    pub map: String,
    pub pve: bool,
    pub description: String,
    pub modded: bool,
    pub private: bool,
    pub country: String,
    pub online_players: Option<Vec<Player>>,
}



impl Server {
    pub fn fmt(&self, full: bool) -> String {
        if full {
            format!(
                "{}:\n  Status: {:?}\n  Players: {}/{}\n  Country: {}\n  IP/Port: {}:{}\n  Rank: #{}\n  PVE: {}\n  Modded: {}\n  Official: {}\n  Description: {}",
                self.name, 
                self.status, 
                self.players, self.max_players,
                self.country,
                self.ip, self.port,
                self.rank,
                bool_fmt(self.pve),
                bool_fmt(self.modded),
                bool_fmt(self.official),
                self.description,
            )
        } else {
            format!(
                "{}:\n  Status: {:?}\n  Players: {}/{}\n  IP/Port: {}:{}",
                self.name, self.status, self.players, self.max_players, self.ip, self.port
            )
        }
    }
}
