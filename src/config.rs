use anyhow::{anyhow, Context, Result};
use std::env;
use std::path::PathBuf;

pub const ALLOWED_CHANNEL_NAME: &str = "swade-bot";

#[derive(Debug, Clone)]
pub struct Config {
    pub discord_token: String,
    pub guild_id: Option<u64>,
    pub data_dir: PathBuf,
    pub allowed_channel_name: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let discord_token = env::var("DISCORD_TOKEN")
            .context("DISCORD_TOKEN is required. Create a .env file from .env.example")?;

        let guild_id = match env::var("GUILD_ID") {
            Ok(value) if !value.trim().is_empty() => Some(
                value
                    .trim()
                    .parse::<u64>()
                    .context("GUILD_ID must be a Discord snowflake integer")?,
            ),
            _ => None,
        };

        let data_dir = match env::var("SWADEDSBOT_DATA_DIR") {
            Ok(value) if !value.trim().is_empty() => PathBuf::from(value),
            _ => default_data_dir()?,
        };

        let allowed_channel_name = match env::var("SWADEDSBOT_ALLOWED_CHANNEL") {
            Ok(value) if !value.trim().is_empty() => value.trim().to_string(),
            _ => ALLOWED_CHANNEL_NAME.to_string(),
        };

        Ok(Self {
            discord_token,
            guild_id,
            data_dir,
            allowed_channel_name,
        })
    }

    pub fn ensure_data_dir(&self) -> Result<()> {
        std::fs::create_dir_all(&self.data_dir)
            .with_context(|| format!("cannot create data dir {}", self.data_dir.display()))
    }

    pub fn database_path(&self) -> PathBuf {
        self.data_dir.join("swadedsbot.sqlite")
    }
}

fn default_data_dir() -> Result<PathBuf> {
    let home = env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("USERPROFILE").map(PathBuf::from))
        .ok_or_else(|| anyhow!("cannot determine home directory; set SWADEDSBOT_DATA_DIR"))?;

    Ok(home.join(".swadedsbot"))
}
