use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub listen: String,
    pub secret: String,
    pub logging: Logging,
    pub rules: Vec<Rule>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub target: String,
    pub prefix: String,
    pub schema: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Logging {
    pub level: String,
    pub write_style: String,
}

lazy_static::lazy_static! {
    #[derive(Debug)]
    pub static ref  CONFIG: Config = {
        load_config()
    };

}

pub fn load_config() -> Config {
    // let config = std::fs::read_to_string("config.json").unwrap();
    // let config: config = serde_json::from_str(&config).unwrap();
    // config

    let settings = config::Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("./configs/config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        // .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let conf = settings.try_deserialize::<Config>().unwrap();
    debug!("conf: {:?}", conf);
    conf
}
