# Battlemetrics Query

This is an experimental CLI application written in Rust ([the language](https://www.rust-lang.org/)) for querying the [Battlemetrics](https://www.battlemetrics.com/) API.

```
$ bq server 2788421 info --full
Rustafied.com - EU Medium III:
  Status: Online
  Players: 118/150
  Country: NL
  IP/Port: 195.60.166.148:28015
  Rank: #18
  PVE: No
  Modded: No
  Official: Yes
  Description: ...

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

It currently only supports querying basic information from Rust ([the game](https://rust.facepunch.com/)) servers.
