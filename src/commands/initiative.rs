use crate::initiative::InitiativeSession;
use crate::{Context, Error};

#[poise::command(
    slash_command,
    subcommands("new_session", "draw", "hold", "enemy", "round", "end"),
    check = "crate::commands::ensure_allowed_channel"
)]
pub async fn initiative(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, subcommands("enemy_draw", "enemy_hold"))]
pub async fn enemy(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "new", check = "crate::commands::ensure_admin")]
pub async fn new_session(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;

    if ctx
        .data()
        .db
        .get_active_initiative_session(guild_id)?
        .is_some()
    {
        ctx.say(
            "⚠️ C'è già una sessione di iniziativa attiva. Usa `/initiative end` prima di iniziarne una nuova.",
        )
        .await?;
        return Ok(());
    }

    let session = InitiativeSession::new();
    ctx.data()
        .db
        .save_active_initiative_session(guild_id, &session)?;

    ctx.say(
        "🃏 **Iniziativa avviata**\nRound corrente: **1**\nIl mazzo da poker è stato preparato con 54 carte, inclusi i Jokers.",
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn draw(ctx: Context<'_>) -> Result<(), Error> {
    crate::commands::send_ephemeral_reply(
        ctx,
        "⚠️ `/initiative draw` non è ancora disponibile in questo commit.",
    )
    .await
}

#[poise::command(slash_command)]
pub async fn hold(ctx: Context<'_>) -> Result<(), Error> {
    crate::commands::send_ephemeral_reply(
        ctx,
        "⚠️ `/initiative hold` non è ancora disponibile in questo commit.",
    )
    .await
}

#[poise::command(
    slash_command,
    rename = "draw",
    check = "crate::commands::ensure_admin"
)]
pub async fn enemy_draw(ctx: Context<'_>) -> Result<(), Error> {
    crate::commands::send_ephemeral_reply(
        ctx,
        "⚠️ `/initiative enemy draw` non è ancora disponibile in questo commit.",
    )
    .await
}

#[poise::command(
    slash_command,
    rename = "hold",
    check = "crate::commands::ensure_admin"
)]
pub async fn enemy_hold(ctx: Context<'_>) -> Result<(), Error> {
    crate::commands::send_ephemeral_reply(
        ctx,
        "⚠️ `/initiative enemy hold` non è ancora disponibile in questo commit.",
    )
    .await
}

#[poise::command(slash_command, check = "crate::commands::ensure_admin")]
pub async fn round(ctx: Context<'_>) -> Result<(), Error> {
    crate::commands::send_ephemeral_reply(
        ctx,
        "⚠️ `/initiative round` non è ancora disponibile in questo commit.",
    )
    .await
}

#[poise::command(slash_command, check = "crate::commands::ensure_admin")]
pub async fn end(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    if ctx.data().db.end_initiative_session(guild_id)? {
        ctx.say("🛑 **Iniziativa terminata**\nLa sessione di combattimento attiva è stata chiusa.")
            .await?;
    } else {
        ctx.say("ℹ️ Non c'è nessuna sessione di iniziativa attiva da chiudere.")
            .await?;
    }

    Ok(())
}
