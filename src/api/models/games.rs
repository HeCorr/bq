pub mod rust;
pub mod minecraft;

pub trait GameServerDetails {
    fn fmt(&self) -> String;
}
