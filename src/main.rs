mod commands;
mod config;
mod db;
mod dice;
mod formatting;

use anyhow::Context as AnyhowContext;
use poise::serenity_prelude as serenity;
use tracing::{error, info};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone)]
pub struct Data {
    pub db: db::Database,
    pub allowed_channel_name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

    let cfg = config::Config::from_env()?;
    cfg.ensure_data_dir()?;

    let database_path = cfg.database_path();
    let db = db::Database::open(&database_path)
        .with_context(|| format!("cannot open SQLite database at {}", database_path.display()))?;

    info!(data_dir = %cfg.data_dir.display(), db = %database_path.display(), "SWADE Discord Bot starting");

    let commands = commands::all();
    let discord_token = cfg.discord_token.clone();
    let setup_cfg = cfg.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            let cfg = setup_cfg.clone();
            let db = db.clone();
            Box::pin(async move {
                info!(bot = %ready.user.name, "connected to Discord Gateway");

                if let Some(guild_id) = cfg.guild_id {
                    let guild_id = serenity::GuildId::new(guild_id);
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        guild_id,
                    )
                    .await?;
                    info!(
                        guild_id = guild_id.get(),
                        "registered slash commands in guild"
                    );
                } else {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    info!("registered slash commands globally");
                }

                Ok(Data {
                    db,
                    allowed_channel_name: cfg.allowed_channel_name.clone(),
                })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged();
    let mut client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await?;

    if let Err(err) = client.start().await {
        error!(error = %err, "Discord client ended with error");
        return Err(err.into());
    }

    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::CommandCheckFailed {
            error: Some(error),
            ctx,
            ..
        } => {
            error!(
                command = %ctx.command().qualified_name,
                error = %error,
                "command check failed"
            );
            let _ = ctx
                .send(
                    poise::CreateReply::default()
                        .content("⚠️ Impossibile verificare i requisiti del comando.")
                        .ephemeral(true),
                )
                .await;
        }
        poise::FrameworkError::CommandCheckFailed { error: None, .. } => {}
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!(command = %ctx.command().qualified_name, error = %error, "command failed");
            let _ = ctx
                .send(
                    poise::CreateReply::default()
                        .content(format!("⚠️ Errore durante il comando: `{error}`"))
                        .ephemeral(true),
                )
                .await;
        }
        other => {
            if let Err(err) = poise::builtins::on_error(other).await {
                error!(error = %err, "error while handling framework error");
            }
        }
    }
}

fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}
