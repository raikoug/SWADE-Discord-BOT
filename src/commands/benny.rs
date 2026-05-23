use crate::{Context, Error};
use anyhow::anyhow;
use poise::serenity_prelude as serenity;

#[poise::command(
    slash_command,
    subcommands("give", "spend", "list", "reset"),
    check = "crate::commands::ensure_allowed_channel"
)]
pub async fn benny(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "give",
    check = "crate::commands::ensure_benny_admin"
)]
pub async fn give(
    ctx: Context<'_>,
    #[description = "Player che riceve i Bennies"] user: serenity::User,
    #[description = "Quantità, default 1"] amount: Option<i64>,
    #[description = "Motivo, opzionale"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = require_guild_id(&ctx)?;
    let amount = amount.unwrap_or(1);
    let new_total = ctx.data().db.add_bennies(guild_id, user.id.get(), amount)?;

    let reason_line = reason
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("\n**Motivo:** {value}"))
        .unwrap_or_default();

    ctx.say(format!(
        "🎟️ **Benny assegnato**\n**User:** {}\n**Amount:** +{}\n**Totale:** {}{}",
        user.name, amount, new_total, reason_line
    ))
    .await?;

    Ok(())
}

#[poise::command(
    slash_command,
    rename = "spend",
    check = "crate::commands::ensure_benny_admin"
)]
pub async fn spend(
    ctx: Context<'_>,
    #[description = "Player che spende i Bennies"] user: serenity::User,
    #[description = "Quantità, default 1"] amount: Option<i64>,
    #[description = "Motivo, opzionale"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = require_guild_id(&ctx)?;
    let amount = amount.unwrap_or(1);
    let new_total = ctx
        .data()
        .db
        .spend_bennies(guild_id, user.id.get(), amount)?;

    let reason_line = reason
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("\n**Motivo:** {value}"))
        .unwrap_or_default();

    ctx.say(format!(
        "🎟️ **Benny speso**\n**User:** {}\n**Amount:** -{}\n**Totale:** {}{}",
        user.name, amount, new_total, reason_line
    ))
    .await?;

    Ok(())
}

#[poise::command(slash_command, rename = "list")]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = require_guild_id(&ctx)?;
    let rows = ctx.data().db.list_bennies(guild_id)?;

    if rows.is_empty() {
        ctx.say("🎟️ **Bennies**\nNessun player tracciato. Usa `/benny give` per aggiungerne uno.")
            .await?;
        return Ok(());
    }

    let mut lines = vec!["🎟️ **Bennies**".to_string(), String::new()];
    for row in rows {
        lines.push(format!("<@{}>: {}", row.user_id, row.amount));
    }

    ctx.say(lines.join("\n")).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "reset",
    check = "crate::commands::ensure_benny_admin"
)]
pub async fn reset(
    ctx: Context<'_>,
    #[description = "Valore a inizio sessione, default 3"] amount: Option<i64>,
) -> Result<(), Error> {
    let guild_id = require_guild_id(&ctx)?;
    let amount = amount.unwrap_or(3);
    let changed = ctx.data().db.reset_bennies(guild_id, amount)?;

    ctx.say(format!(
        "🔄 **Bennies resettati**\nPlayer aggiornati: {}\nNuovo valore: {}",
        changed, amount
    ))
    .await?;

    Ok(())
}

fn require_guild_id(ctx: &Context<'_>) -> anyhow::Result<u64> {
    ctx.guild_id()
        .map(|id| id.get())
        .ok_or_else(|| anyhow!("questo comando richiede un server Discord, non un DM"))
}
