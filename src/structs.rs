pub mod args {
    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    #[command(
        name = "logs.tf parser",
        version,
        about, 
        long_about = None
    )]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Option<Commands>,
    }

    #[derive(Subcommand, Clone)]
    pub enum Commands {
        /// Parse a file with log ids.
        Parse {
            /// Files to parse
            files: Vec<String>,
        },
        /// Search and parse logs from logs.tf
        Search {
            /// Title text search (min. 2 characters)
            #[arg(short, long)]
            title: Option<String>,
            /// Exact name of a map (pl_upward).
            #[arg(short, long)]
            map: Option<String>,
            /// Uploader SteamID64
            #[arg(short, long)]
            uploader: Option<u64>,
            /// One or more player SteamID64 values
            #[arg(short, long, num_args = 1..)]
            players: Option<Vec<u64>>,
            /// Limit results (default 1000, maximum 10000)
            #[arg(short, long)]
            limit: Option<u32>,
            /// Offset results (default 0)
            #[arg(short, long)]
            offset: Option<u32>,
        }
    }
}

pub mod stats {
    #[derive(Default)]
    pub struct Stats {
        pub name: Vec<String>,
        pub kills: u32,
    }
}

pub mod search {
    use serde::Deserialize;
    use steamid_ng::SteamID;
    // use std::collections::HashMap;

    #[derive(Deserialize, Debug)]
    pub struct SearchResponse {
        pub results: u32,
        pub total: u32,
        pub parameters: Parameters,
        pub logs: Vec<SearchResults>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Parameters {
            pub title: Option<String>,
            /// Exact name of a map (pl_upward).
            pub map: Option<String>,
            /// Uploader SteamID64
            pub uploader: Option<SteamID>,
            /// One or more player SteamID64 values
            pub players: Option<Vec<SteamID>>,
            /// Limit results (default 1000, maximum 10000)
            pub limit: Option<u32>,
            /// Offset results (default 0)
            pub offset: Option<u32>,
    }

    #[derive(Deserialize, Debug)]
    pub struct SearchResults {
        pub id: u32,
        pub title: String,
        pub map: String,
        pub date: u32,
        pub views: u32,
        pub players: u8,
    }
}

/// Includes all data and serializations for [logs.tf](https://logs.tf/) api
pub mod log {
    // use steamid_ng::SteamID;
    use serde::Deserialize;
    use std::collections::HashMap;
    use log_internal::*;

    /// Root structure of deserialized log data from [logs.tf](https://logs.tf/)
    #[derive(Deserialize, Debug)]
    pub struct Log {
        /// Log API version
        pub version: u32,
        /// Cumulative team stats
        pub teams: Teams,
        /// Length of the match in seconds
        pub length: u32,
        /// SteamID3 -> [Player]
        pub players: HashMap<String, Player>,
        /// SteamID3 -> User Name
        pub names: HashMap<String, String>,
        /// Round data
        pub rounds: Vec<Round>,
        /// Chat logs
        pub chat: Vec<Message>,
    }

    mod log_internal {
        use serde::Deserialize;
        use std::collections::HashMap;

        #[derive(Deserialize, Debug)]
        pub struct Teams {
            /// Blue team cumulative stats
            #[serde(rename="Blue")]
            pub blue: Team, 
            /// Red team cumulative stats
            #[serde(rename="Red")]
            pub red: Team 
        }

        #[derive(Deserialize, Debug)]
        pub struct Team {
            /// Total score cumulitively by team
            pub score: u32,
            /// Total kills cumulitively by team
            pub kills: u32,
            /// Total deaths cumulitively by team
            pub deaths: u32,
            /// Total damage dealt cumulitively by team
            #[serde(rename="dmg")]
            pub damage: u32,
            /// Total uber charges used cumulitively by team
            #[serde(rename="charges")]
            pub ubers: u32,
            /// Total initial captures after round start
            pub firstcaps: u32,
            /// Total captures cumuliteively by team
            pub caps: u32
        }

        #[derive(Deserialize, Debug)]
        pub struct Player {
            /// Player's team color
            pub team: String,
            /// Class specific stats; Can index up to 9
            pub class_stats: Vec<ClassStats>,
            /// Total kills
            pub kills: u32,
            /// Total deaths
            pub deaths: u32,
            /// Total assists
            pub assists: u32,
            /// Total times killed without interference by an enemey player
            pub suicides: u32,
            /// Average kills and assists per death
            #[serde(rename="kapd")]
            pub kills_assists_per_death: String,
            /// Average kills per death
            #[serde(rename="kpd")]
            pub kills_per_death: String,
            /// Total damage dealt
            #[serde(rename="dmg")]
            pub damage: u32,
            /// No idea
            #[serde(rename="dmg_real")]
            pub damage_real_wtf: u32,
            /// Total damage taken
            #[serde(rename="dt")]
            pub damage_taken: u32,
            /// No idea
            pub lks: u32,
            /// Total direct hits with a projectile weapon on an airborne target
            #[serde(rename="as")]
            pub airshots: u32,
            /// No idea
            pub dapd: u32,
            /// No idea
            pub dapm: u32,
            /// Total number of ubers
            pub ubers: u32,
            /// Contains specific number of uses for each uber type
            pub ubertypes: UberTypes,
            /// Medic deaths while fully charged
            pub drops: u32,
            /// Total medkits collected with each kit given the following scoring:
            /// - Small  = 1
            /// - Medium = 2
            /// - Large  = 4
            pub medkits: u32,
            /// Total health gained from medkits
            #[serde(rename="medkits_hp")]
            pub health_from_medkits: u32,
            /// Spy backstabs
            pub backstabs: u32,
            /// Sniper / Ambasitor headshots
            pub headshots: u32,
            /// Engineer sentry...? (Kills? Built?)
            pub sentries: u32,
            /// No idea
            pub heal: u32,
            /// No idea
            pub cpc: u32,
            /// No idea
            pub ic: u32,
            /// Extra medic stats. Only [Some] when [ClassStats::class] is "medic"
            /// **Note**: All medic stats might be [None] even if they should be [Some] 
            /// because who the fuck knows.
            pub medicstats: Option<MedicStats>,
        }

        #[derive(Deserialize, Debug)]
        pub struct ClassStats {
            // Name of class
            #[serde(rename="type")]
            pub class: String,
            /// Kills as class
            pub kills: u32,
            /// Assists as class
            pub assists: u32,
            /// Deaths as class
            pub deaths: u32,
            /// Damage as class
            #[serde(rename="dmg")]
            pub damage: u32,
            /// [Weapon] data. Weapon name -> Weapon
            /// ***TODO:*** create list of all weapon names
            pub weapon: HashMap<String, Weapon>,
            /// Time in seconds as class
            pub total_time: u32,
        }

        #[derive(Deserialize, Debug)]
        pub struct Weapon {
            /// Kills with weapon
            pub kills: u32,
            /// Damage with weapon
            #[serde(rename="dmg")]
            pub damage: u32,
            /// Average damage per shot with weapon
            #[serde(rename="avg_dmg")]
            pub average_damage: f32,
            /// Total times weapon was fired
            pub shots: u32,
            /// Total number of shots hit
            pub hits: u32,
        }

        #[derive(Deserialize, Debug)]
        pub struct UberTypes {
            /// [Medigun](https://wiki.teamfortress.com/wiki/Medi_Gun) Charges
            pub medigun: Option<u32>,       
            /// [Kritzkrieg](https://wiki.teamfortress.com/wiki/Kritzkrieg) Charges
            #[serde(rename="kritskrieg")]
            pub kritzkrieg: Option<u32>,
            /// [Vaccinator](https://wiki.teamfortress.com/wiki/Vaccinator) Charges
            pub vaccinator: Option<u32>,
            /// [Quick-Fix](https://wiki.teamfortress.com/wiki/Quick-Fix) Charges
            pub quickfix: Option<u32>,
        }

        /// **Note**: All medic stats might be [None] because who the fuck knows.
        #[derive(Deserialize, Debug)]
        pub struct MedicStats {
            /// Total "major" :shrug: uber advantages lost
            pub advantages_lost: Option<u32>,
            /// Largest advantage lost due to drop or holding in seconds
            pub biggest_advantage_lost: Option<u32>,
            /// Deaths within 95% and 99% uber
            #[serde(rename="deaths_with_95_99_uber")]
            pub deaths_near_charge: Option<u32>,
            /// Deaths within 20 seconds after using uber
            #[serde(rename="deaths_within_20s_after_uber")]
            pub deaths_after_uber: Option<u32>,
            /// Average time after spawning before healing
            pub avg_time_before_healing: Option<f32>,
            /// Average time to gain full uber charge
            pub avg_time_to_build: Option<f32>,
            /// Average time after full uber charge before using uber
            pub avg_time_before_using: Option<f32>,
            /// Average uber duration in seconds
            pub avg_uber_length: Option<f32>,
        }

        /// **Note:** `winner` is [None] when round ends in a stalemate
        #[derive(Deserialize, Debug)]
        pub struct Round {
            /// Ticks before round start
            pub start_time: u32,
            /// Round winner, either "red", "blue", or [None]
            pub winner: Option<String>,
            /// Cumulative team stats within a round
            pub team: RoundTeams,
            /// Event data
            pub events: Vec<Events>,
        }

        #[derive(Deserialize, Debug)]
        pub struct RoundTeams { 
            /// Blue team cumulative stats within a given round
            #[serde(rename="Blue")]
            pub blue: RoundTeam, 
            /// Red team cumulative stats within a given round
            #[serde(rename="Red")]
            pub red: RoundTeam 
        }

        #[derive(Deserialize, Debug)]
        pub struct RoundTeam {
            /// Total score cumulitively by team within a given round
            pub score: u32,
            /// Total kills cumulitively by team within a given round
            pub kills: u32,
            /// Total damage dealt cumulitively by team within a given round
            #[serde(rename="dmg")]
            pub damage: u32,
            /// Total uber charges used cumulitively by team
            pub ubers: u32
        }

        #[derive(Deserialize, Debug)]
        pub struct Events {
            /// Event type and main descriptor of event. Can be:
            /// - "charge"
            /// - "drop"
            /// - "medic_death"
            /// - "pointcap"
            /// - "round_win"
            #[serde(rename="type")]
            pub event: String,
            /// Type of medigun if event is "charge"
            /// TODO: Define all possible weapon types ***somewhere***
            pub medigun: Option<String>,
            /// Time at which event occurred in seconds
            pub time: u32,
            /// SteamID3 of target player
            pub steamid: Option<String>,
            /// SteamID3 of killer when event is "medic_death"
            pub killer: Option<String>,
            /// Point ID captured when event is "pointcap"
            pub point: Option<u8>,
            /// Player's team color. Can be [None] if round ends in stalemate.
            /// Will only ever be [None] when event is "round_win", so you may safely [Option::expect]
            /// if you are not checking for "round_win"
            pub team: Option<String>
        }

        #[derive(Deserialize, Debug)]
        pub struct Message {
            /// SteamID3
            pub steamid: String,
            /// Player's username at the time of log upload
            pub name: String,
            #[serde(rename="msg")]
            pub message: String,
        }
    }
}
