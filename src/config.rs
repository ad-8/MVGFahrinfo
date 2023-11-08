use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// index of a station in `stations.json` // TODO fn thats prints all station names with index
    pub fav_station_idx: Option<usize>,
    /// list of directions to highlight departures to those directions
    pub fav_directions: Option<Vec<String>>,
    /// make the display of seconds since last refresh optional (for performance reasons)
    pub display_seconds: Option<bool>,
    /// in seconds
    pub refresh_rate: Option<usize>,
}

impl Config {
    /// If a `config.toml` exists, tries to parse the file,
    /// otherwise initializes all fields with [None].
    pub fn parse() -> Self {
        match std::fs::read_to_string("config.toml") { 
            Ok(s) => {
                // TODO handle error properly
                let c: Config = toml::from_str(&s).expect("failed to parse config.toml");
                c 
            }
            Err(_) => Config {
                fav_station_idx: None,
                fav_directions: None,
                display_seconds: None,
                refresh_rate: None,
            },
        }
    }
}
