use envconfig::Envconfig;

use crate::config::Config;

pub(crate) async fn spam_single_track() {
    tracing::info!("Spamming single track");

    let config = Config::init_from_env().unwrap();

    let client = reqwest::Client::new();
    let res = client
        .get(&config.api_url)
        .header("content-type", "application/x-www-form-urlencoded")
        .header("cookie", format!("PHPSESSID={}", &config.cookie))
        .body(format!(
        "artist%5B0%5D={}&track%5B0%5D={}&album%5B0%5D={}&albumArtist%5B0%5D=&timestamp%5B0%5D={}",
        &config.artist,
        &config.track,
        &config.album,
        chrono::Utc::now().timestamp()
    ))
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        tracing::info!("Success scrobbled track");
    }

    if res.status() == 429 {
        tracing::info!("Rate limited");
    }
}
