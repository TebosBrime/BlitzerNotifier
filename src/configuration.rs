use crate::model::LocationBox;
use config::Config;
use std::sync::OnceLock;
use tokio::sync::RwLock;

static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

fn config() -> &'static RwLock<Config> {
    CONFIG.get_or_init(|| RwLock::new(init_config()))
}

fn init_config() -> Config {
    Config::builder()
        .add_source(config::File::with_name("Settings"))
        .add_source(config::Environment::with_prefix("BLITZER"))
        .build()
        .unwrap()
}

pub async fn get_location_box() -> LocationBox {
    let lat_min = get_float("locations.first.lat").await;
    let lng_min = get_float("locations.first.lng").await;
    let lat_max = get_float("locations.second.lat").await;
    let lng_max = get_float("locations.second.lng").await;

    LocationBox {
        lat_min,
        lng_min,
        lat_max,
        lng_max,
    }
}

pub async fn get_mysql_connection_uri() -> String {
    format!(
        "mysql://{}:{}@{}:{}/{}",
        get_string("database.username").await,
        get_string("database.password").await,
        get_string("database.host").await,
        get_int("database.port").await,
        get_string("database.database").await,
    )
}

pub async fn get_telegram_bot_configuration() -> TelegramBotConfiguration {
    TelegramBotConfiguration {
        token: get_string("telegram.token").await,
        chat_id : get_string("telegram.chat_id").await,
    }
}

pub struct TelegramBotConfiguration {
    pub token: String,
    pub chat_id: String,
}

async fn get_float(key: &str) -> f64 {
    let config = config().read().await;

    config
        .get_float(key)
        .unwrap_or_else(|_| panic!("Configuration should have a float on {key}"))
}

async fn get_string(key: &str) -> String {
    let config = config().read().await;

    config
        .get_string(key)
        .unwrap_or_else(|_| panic!("Configuration should have a string on {key}"))
}
async fn get_int(key: &str) -> i64 {
    let config = config().read().await;

    config
        .get_int(key)
        .unwrap_or_else(|_| panic!("Configuration should have a int on {key}"))
}
