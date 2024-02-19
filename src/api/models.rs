#[derive(Debug)]
pub struct Player {
    pub name: String,
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
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\n  Status: {:?}\n  Players: {}/{}\n  IP/Port: {}:{}",
            self.name, self.status, self.players, self.max_players, self.ip, self.port
        )
    }
}
