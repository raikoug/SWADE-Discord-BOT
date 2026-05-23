use crate::formatting::{
    format_enemy_draw_result, format_enemy_hold_result, format_initiative_hold,
    format_player_initiative_draw,
};
use crate::initiative::{InitiativeError, InitiativeSession};
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
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    let Some(mut session) = ctx.data().db.get_active_initiative_session(guild_id)? else {
        crate::commands::send_ephemeral_reply(
            ctx,
            "⚠️ Non c'è una sessione di iniziativa attiva. Usa `/initiative new` prima di pescare.",
        )
        .await?;
        return Ok(());
    };

    let display_name = ctx.author().display_name().to_string();
    let draw = match session.draw_player(ctx.author().id.get(), &display_name) {
        Ok(draw) => draw,
        Err(err) => match err.downcast_ref::<InitiativeError>() {
            Some(InitiativeError::AlreadyDrawn) => {
                crate::commands::send_ephemeral_reply(
                    ctx,
                    "⚠️ Hai già pescato una carta in questo round.",
                )
                .await?;
                return Ok(());
            }
            _ => return Err(err.into()),
        },
    };

    ctx.data()
        .db
        .save_active_initiative_session(guild_id, &session)?;
    ctx.say(format_player_initiative_draw(&draw, session.round))
        .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn hold(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    let Some(mut session) = ctx.data().db.get_active_initiative_session(guild_id)? else {
        crate::commands::send_ephemeral_reply(
            ctx,
            "⚠️ Non c'è una sessione di iniziativa attiva. Usa `/initiative new` prima di usare Hold.",
        )
        .await?;
        return Ok(());
    };

    let draw = match session.hold_player(ctx.author().id.get()) {
        Ok(draw) => draw,
        Err(err) => match err.downcast_ref::<InitiativeError>() {
            Some(InitiativeError::HoldWithoutDraw) => {
                crate::commands::send_ephemeral_reply(
                    ctx,
                    "⚠️ Non puoi andare in Hold prima di aver pescato una carta in questo round.",
                )
                .await?;
                return Ok(());
            }
            _ => return Err(err.into()),
        },
    };

    ctx.data()
        .db
        .save_active_initiative_session(guild_id, &session)?;
    ctx.say(format_initiative_hold(&draw)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "draw",
    check = "crate::commands::ensure_admin"
)]
pub async fn enemy_draw(
    ctx: Context<'_>,
    #[description = "Nomi separati da ;"] names: String,
) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    let Some(mut session) = ctx.data().db.get_active_initiative_session(guild_id)? else {
        crate::commands::send_ephemeral_reply(
            ctx,
            "⚠️ Non c'è una sessione di iniziativa attiva. Usa `/initiative new` prima di pescare per i nemici.",
        )
        .await?;
        return Ok(());
    };

    let result = match session.draw_enemies(&names) {
        Ok(result) => result,
        Err(err) => match err.downcast_ref::<InitiativeError>() {
            Some(InitiativeError::NoValidEnemyNames) => {
                crate::commands::send_ephemeral_reply(
                    ctx,
                    "⚠️ Devi indicare almeno un nome valido. Separa i nomi con `;`.",
                )
                .await?;
                return Ok(());
            }
            Some(InitiativeError::NotEnoughCards {
                requested,
                remaining,
            }) => {
                crate::commands::send_ephemeral_reply(
                    ctx,
                    &format!(
                        "⚠️ Il mazzo non ha abbastanza carte disponibili: richieste {}, rimaste {}.",
                        requested, remaining
                    ),
                )
                .await?;
                return Ok(());
            }
            _ => return Err(err.into()),
        },
    };

    ctx.data()
        .db
        .save_active_initiative_session(guild_id, &session)?;
    ctx.say(format_enemy_draw_result(&result, session.round))
        .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "hold",
    check = "crate::commands::ensure_admin"
)]
pub async fn enemy_hold(
    ctx: Context<'_>,
    #[description = "Nomi separati da ;"] names: String,
) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    let Some(mut session) = ctx.data().db.get_active_initiative_session(guild_id)? else {
        crate::commands::send_ephemeral_reply(
            ctx,
            "⚠️ Non c'è una sessione di iniziativa attiva. Usa `/initiative new` prima di usare `enemy hold`.",
        )
        .await?;
        return Ok(());
    };

    let result = match session.hold_enemies(&names) {
        Ok(result) => result,
        Err(err) => match err.downcast_ref::<InitiativeError>() {
            Some(InitiativeError::NoValidEnemyNames) => {
                crate::commands::send_ephemeral_reply(
                    ctx,
                    "⚠️ Devi indicare almeno un nome valido. Separa i nomi con `;`.",
                )
                .await?;
                return Ok(());
            }
            _ => return Err(err.into()),
        },
    };

    ctx.data()
        .db
        .save_active_initiative_session(guild_id, &session)?;
    ctx.say(format_enemy_hold_result(&result)).await?;
    Ok(())
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
