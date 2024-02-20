pub mod minecraft;
pub mod rust;
pub mod tf2;

pub trait GameServerDetails {
    fn fmt(&self) -> String;
}
