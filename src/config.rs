use serde::Deserialize;

const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app_refresh_rate: Option<usize>,
    /// index of a station in `stations.json` // TODO create file that lists all station names with index
    pub fav_station: Option<usize>,
    /// highlight departures to specified directions
    pub fav_directions: Option<Vec<String>>,
    /// display seconds since last refresh
    pub display_seconds: Option<bool>,
    /// in seconds
    pub display_seconds_refresh_rate: Option<usize>,
    /// only display departures of one or more transport type (BAHN, SBAHN, UBAHN, TRAM, BUS)
    pub transport: Option<Vec<String>>,
}

impl Config {
    /// If a `config.toml` exists, tries to parse the file,
    /// otherwise initializes all fields with [None].
    ///
    /// Calls [std::process::exit()] if the config file is not valid [TOML](https://toml.io/en/).
    pub fn parse() -> Self {
        match std::fs::read_to_string(CONFIG_FILE) {
            Ok(s) => {
                let c: Result<Config, toml::de::Error> = toml::from_str(&s);
                if c.is_err() {
                    eprintln!("invalid config file: `{}`", CONFIG_FILE);
                    std::process::exit(1);
                }
                c.unwrap()
            }
            Err(_) => Config {
                app_refresh_rate: None,
                fav_station: None,
                fav_directions: None,
                display_seconds: None,
                display_seconds_refresh_rate: None,
                transport: None,
            },
        }
    }
}
