pub mod arksa;
pub mod minecraft;
pub mod palworld;
pub mod rust;
pub mod tf2;

pub trait GameServerDetails {
    fn fmt(&self) -> String;
}
