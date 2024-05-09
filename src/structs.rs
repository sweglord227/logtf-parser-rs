use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Log {
    /// Log API version
    pub version: u32,
    pub teams: Teams,
    /// Length of the match in seconds
    pub length: u32,
    /// SteamID3 -> [Player]
    pub players: HashMap<String, Player>,
    /// SteamID3 -> User Name
    pub names: HashMap<String, String>,
    pub rounds: Vec<Round>,
    pub chat: Vec<Message>,
}

#[derive(Deserialize, Debug)]
pub struct Teams {
    #[serde(rename="Blue")]
    pub blue: Team, 
    #[serde(rename="Red")]
    pub red: Team 
}

#[derive(Deserialize, Debug)]
pub struct Team {
    pub score: u32,
    pub kills: u32,
    pub deaths: u32,
    #[serde(rename="dmg")]
    pub damage: u32,
    pub charges: u32,
    pub firstcaps: u32,
    pub caps: u32
}

/// **Note:** `winner` is `None` when Tie or Stalemate
#[derive(Deserialize, Debug)]
pub struct Round {
    pub start_time: u32,
    pub winner: Option<String>,
    pub team: RoundTeams,
}

#[derive(Deserialize, Debug)]
pub struct Events {
    #[serde(rename="type")]
    pub which: String,
    pub time: u32,
    pub steamid: String,
    pub team: String
}

#[derive(Deserialize, Debug)]
pub struct RoundTeams { 
    #[serde(rename="Blue")]
    pub blue: RoundTeam, 
    #[serde(rename="Red")]
    pub red: RoundTeam 
}

#[derive(Deserialize, Debug)]
pub struct RoundTeam {
    pub score: u32,
    pub kills: u32,
    pub dmg: u32,
    pub ubers: u32
}

#[derive(Deserialize, Debug)]
pub struct Player {
    pub team: String,
    pub class_stats: Vec<ClassStats>,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub suicides: u32,
    #[serde(rename="kapd")]
    pub kills_assists_per_death: String,
    #[serde(rename="kpd")]
    pub kills_per_death: String,
    #[serde(rename="dmg")]
    pub damage: u32,
    #[serde(rename="dmg_real")]
    pub damage_real_wtf: u32,
    #[serde(rename="dt")]
    pub damage_taken: u32,
    pub lks: u32,
    #[serde(rename="as")]
    pub airshots: u32,
    pub dapd: u32,
    pub dapm: u32,
    pub ubers: u32,
    pub ubertypes: UberTypes,
    pub drops: u32,
    pub medkits: u32,
    #[serde(rename="medkits_hp")]
    pub health_from_medkits: u32,
    pub backstabs: u32,
    pub headshots: u32,
    pub sentries: u32,
    pub heal: u32,
    pub cpc: u32,
    pub ic: u32,
    pub medicstats: Option<MedicStats>,
}

#[derive(Deserialize, Debug)]
pub struct ClassStats {
    #[serde(rename="type")]
    pub class: String,
    pub kills: u32,
    pub assists: u32,
    pub deaths: u32,
    #[serde(rename="dmg")]
    pub damage: u32,
    pub weapon: HashMap<String, Weapon>,
    pub total_time: u32,
}

#[derive(Deserialize, Debug)]
pub struct Weapon {
    pub kills: u32,
    #[serde(rename="dmg")]
    pub damage: u32,
    #[serde(rename="avg_dmg")]
    pub average_damage: f32,
    pub shots: u32,
    pub hits: u32,
}

#[derive(Deserialize, Debug)]
pub struct UberTypes {
    pub medigun: Option<u32>,       
    pub kritskrieg: Option<u32>,
    pub vaccinator: Option<u32>,
    pub quickfix: Option<u32>,
}

/// **Note**: All medic stats might be `None` because who the fuck knows.
#[derive(Deserialize, Debug)]
pub struct MedicStats {
    pub advantages_lost: Option<u32>,
    pub biggest_advantage_lost: Option<u32>,
    pub deaths_with_95_99_uber: Option<u32>,
    pub deaths_within_20s_after_uber: Option<u32>,
    pub avg_time_before_healing: Option<f32>,
    pub avg_time_to_build: Option<f32>,
    pub avg_time_before_using: Option<f32>,
    pub avg_uber_length: Option<f32>,
}


#[derive(Deserialize, Debug)]
pub struct Message {
    pub steamid: String,
    pub name: String,
    #[serde(rename="msg")]
    pub message: String,
}
