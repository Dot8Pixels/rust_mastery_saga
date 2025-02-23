use super::config_model::{
    AdventurersSecret, Database, DotEnvyConfig, GuildCommandersSecret, Server,
};
use super::stage::Stage;
use anyhow::Result;
use dotenvy::dotenv;
use std::env;
pub fn load() -> Result<DotEnvyConfig> {
    dotenv().ok();

    let server = Server {
        port: env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        timeout: env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let database = Database {
        url: env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_adventurer_secret_env() -> Result<AdventurersSecret> {
    dotenv().ok();

    Ok(AdventurersSecret {
        secret: env::var("JWT_ADVENTURER_SECRET").expect("JWT_ADVENTURER_SECRET is invalid"),
        refresh_secret: env::var("JWT_ADVENTURER_REFRESH_SECRET")
            .expect("JWT_ADVENTURER_REFRESH_SECRET is invalid"),
    })
}

pub fn get_guild_commanders_secret_env() -> Result<GuildCommandersSecret> {
    dotenv().ok();

    Ok(GuildCommandersSecret {
        secret: env::var("JWT_GUILD_COMMANDER_SECRET")
            .expect("JWT_GUILD_COMMANDER_SECRET is invalid"),
        refresh_secret: env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET")
            .expect("JWT_GUILD_COMMANDER_REFRESH_SECRET is invalid"),
    })
}
