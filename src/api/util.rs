/// Returns `Yes` if input is true, `No` otherwise.
pub fn bool_fmt(input: bool) -> &'static str {
    if input {
        "Yes"
    } else {
        "No"
    }
}

pub fn game_fmt(input: &String) -> String {
    match input.as_str() {
        "7dtd" => "7 Days to Die",
        "arksa" => "ARK: Survival Ascended",
        "ark" => "ARK: Survival Evolved",
        "arma2" => "ArmA 2",
        "arma3" => "ArmA 3",
        "atlas" => "Atlas",
        "battalion1944" => "Battalion 1944",
        "battlebit" => "BattleBit Remastered",
        "btw" => "Beyond the Wire",
        "conanexiles" => "Conan Exiles",
        "cs" => "Counter-Strike",
        "css" => "Counter-Strike: Source",
        "dnl" => "Dark and Light",
        "dayz" => "DayZ",
        "enshrouded" => "Enshrouded",
        "gmod" => "Garry's Mod",
        "hll" => "Hell Let Loose",
        "insurgency" => "Insurgency",
        "sandstorm" => "Insurgency: Sandstorm",
        "minecraft" => "Minecraft",
        "mordhau" => "MORDHAU",
        "moe" => "Myth of Empires",
        "palworld" => "Palworld",
        "pixark" => "PixARK",
        "zomboid" => "Project Zomboid",
        "rend" => "Rend",
        "rs2vietnam" => "Rising Storm 2: Vietnam",
        "rust" => "Rust",
        "scum" => "SCUM",
        "squad" => "Squad",
        "postscriptum" => "Squad 44",
        "tf2" => "Team Fortress 2",
        "thefront" => "The Front",
        "unturned" => "Unturned",
        "vrising" => "V Rising",
        "valheim" => "Valheim",
        &_ => input.as_str(),
    }
    .to_owned()
}
