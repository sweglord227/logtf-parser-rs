use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Log {
    pub version: u32,
    pub teams: Teams,
    pub length: u32,
    pub players: HashMap<String, Player>,
}

#[derive(Deserialize)]
pub struct Teams {
    #[serde(rename="Blue")]
    pub blue: Team, 
    #[serde(rename="Red")]
    pub red: Team 
}

#[derive(Deserialize)]
pub struct Team {
    pub score: u32,
    pub kills: u32,
    pub deaths: u32,
    pub dmg: u32,
    pub charges: u32,
    pub firstcaps: u32,
    pub caps: u32
}

#[derive(Deserialize)]
pub struct Round {
    pub start_time: u32,
    pub winner: String,
    pub team: RoundTeams,

}

#[derive(Deserialize)]
pub struct Events {
    #[serde(rename="type")]
    pub which: String,
    pub time: u32,
    pub steamid: String,
    pub team: String
}

#[derive(Deserialize)]
pub struct RoundTeams { 
    #[serde(rename="Blue")]
    pub blue: RoundTeam, 
    #[serde(rename="Red")]
    pub red: RoundTeam 
}

#[derive(Deserialize)]
pub struct RoundTeam {
    pub score: u32,
    pub kills: u32,
    pub dmg: u32,
    pub ubers: u32
}

#[derive(Deserialize)]
pub struct Players {
    // pub steam_id: HashMap<String, Player>,
}

#[derive(Deserialize)]
pub struct Player {
    pub team: String,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub suicides: u32,
    pub kapd: String,
    pub kpd: String,
    pub dmg: u32,
    pub dmg_real: u32,
    #[serde(rename="dt")]
    pub dmg_taken: u32,
    pub lks: u32,
    #[serde(rename="as")]
    pub airshots: u32,
    pub dapd: u32,
    pub dapm: u32,
    pub ubers: u32,
    pub ubertypes: UberTypes,
    pub drops: u32,
    pub medkits: u32,
    pub medkits_hp: u32,
    pub backstabs: u32,
    pub headshots: u32,
    pub sentries: u32,
    pub heal: u32,
    pub cpc: u32,
    pub ic: u32,
    // pub medicstats: MedicStats,
}

#[derive(Deserialize)]
pub struct ClassStats {
    #[serde(rename="type")]
    pub class: String,
    pub kills: u32,
    pub assists: u32,
    pub deaths: u32,
    pub dmg: u32,
    pub weapon: HashMap<String, Weapon>,
    pub total_time: u32,
}

#[derive(Deserialize)]
pub struct Weapon {
    pub kills: u32,
    pub dmg: u32,
    pub avg_dmg: u32,
    pub shots: u32,
    pub hits: u32,
}

#[derive(Deserialize)]
pub struct UberTypes {
    pub medigun: Option<u32>,       
    pub kritskrieg: Option<u32>,
    pub vaccinator: Option<u32>,
    pub quickfix: Option<u32>,
}

#[derive(Deserialize)]
pub struct MedicStats {
    pub advantages_lost: u32,
    pub biggest_advantage_lost: u32,
    pub deaths_with_95_99_uber: u32,
    pub deaths_within_20s_after_uber: u32,
    pub avg_time_before_healing: u32,
    pub avg_time_to_build: u32,
    pub avg_time_before_using: u32,
    pub avg_uber_length: u32,
}
