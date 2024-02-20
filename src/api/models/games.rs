pub mod rust;

pub trait GameServerDetails {
    fn fmt(&self) -> String;
}
