use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(
        from = "API_URL",
        default = "https://openscrobbler.com/api/v2/scrobble.php"
    )]
    pub api_url: String,

    #[envconfig(from = "COOKIE")]
    pub cookie: String,

    #[envconfig(from = "ARTIST")]
    pub artist: String,

    #[envconfig(from = "TRACK")]
    pub track: String,

    #[envconfig(from = "ALBUM")]
    pub album: String,

    #[envconfig(from = "INTERVAL", default = "1")]
    pub interval: u64,
}
