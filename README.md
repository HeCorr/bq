# Battlemetrics Query

This is an experimental CLI application written in Rust ([the language](https://www.rust-lang.org/)) for querying the [Battlemetrics](https://www.battlemetrics.com/) API.

```
$ bq server 2788421 info --full
Rustafied.com - EU Medium III:
  Game: Rust
  Status: Online
  Players: 81/125
  Country: NL
  IP/Port: 195.60.166.148:28015
  Address: eumedium3.rustafied.com
  Private: No
  Rank: #11
  Details: 
    PVE: No
    Map: Rustafied Custom Map
    Official: Yes
    ...

$ bq server 2788421 player-list
Rustafied.com - EU Medium III:
  Online players (117):
    Grub#1
    Butcher
    BOSS
    Heart
    AK47
    Crosshairx user
    Timmy
    Sciencist
    ...
```

Fully supported game servers:
- [x] Minecraft
- [x] Rust
- [ ] 7 Days to Die
- [ ] ARK: Survival Ascended
- [ ] ARK: Survival Evolved
- [ ] ArmA 2
- [ ] ArmA 3
- [ ] Atlas
- [ ] Battalion 1944
- [ ] BattleBit Remastered
- [ ] Beyond the Wire
- [ ] Conan Exiles
- [ ] Counter-Strike
- [ ] Counter-Strike: Source
- [ ] Dark and Light
- [ ] DayZ
- [ ] Enshrouded
- [ ] Garry's Mod
- [ ] Hell Let Loose
- [ ] Insurgency
- [ ] Insurgency: Sandstorm
- [ ] MORDHAU
- [ ] Myth of Empires
- [ ] Palworld
- [ ] PixARK
- [ ] Project Zomboid
- [ ] Rend
- [ ] Rising Storm 2: Vietnam
- [ ] SCUM
- [ ] Squad
- [ ] Squad 44
- [ ] Team Fortress 2
- [ ] The Front
- [ ] Unturned
- [ ] V Rising
- [ ] Valheim

(Unmarked games can still be queried but lack detailed info)
