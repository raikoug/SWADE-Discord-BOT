use crate::formatting::{
    comment_line, format_enemy_draw_result, format_enemy_hold_result, format_initiative_hold,
    format_player_initiative_draw, format_round_resolution,
};
use crate::initiative::{InitiativeError, InitiativeSession};
use crate::{Context, Error};

#[poise::command(
    slash_command,
    subcommands(
        "new_session",
        "draw",
        "hold",
        "enemy_draw",
        "enemy_hold",
        "round",
        "end"
    ),
    check = "crate::commands::ensure_allowed_channel"
)]
pub async fn initiative(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "new", check = "crate::commands::ensure_admin")]
pub async fn new_session(
    ctx: Context<'_>,
    #[description = "Commento opzionale"] comment: Option<String>,
) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;

    if ctx
        .data()
        .db
        .get_active_initiative_session(guild_id)?
        .is_some()
    {
        ctx.say(
            "⚠️ C'è già una sessione di iniziativa attiva. Usa `/swade initiative end` prima di iniziarne una nuova.",
        )
        .await?;
        return Ok(());
    }

    let session = InitiativeSession::new();
    ctx.data()
        .db
        .save_active_initiative_session(guild_id, &session)?;
    let comment_suffix = comment_line(comment.as_deref());

    ctx.say(
        format!(
            "🃏 **Iniziativa avviata**\nRound corrente: **1**\nIl mazzo da poker è stato preparato con 54 carte, inclusi i Jokers.{}",
            comment_suffix
        ),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn draw(
    ctx: Context<'_>,
    #[description = "Commento opzionale"] comment: Option<String>,
) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    let Some(mut session) = ctx.data().db.get_active_initiative_session(guild_id)? else {
        crate::commands::send_ephemeral_reply(
            ctx,
            "⚠️ Non c'è una sessione di iniziativa attiva. Usa `/swade initiative new` prima di pescare.",
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
    let comment_suffix = comment_line(comment.as_deref());
    ctx.say(format!(
        "{}{}",
        format_player_initiative_draw(&draw, session.round),
        comment_suffix
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn hold(
    ctx: Context<'_>,
    #[description = "Commento opzionale"] comment: Option<String>,
) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    let Some(mut session) = ctx.data().db.get_active_initiative_session(guild_id)? else {
        crate::commands::send_ephemeral_reply(
            ctx,
            "⚠️ Non c'è una sessione di iniziativa attiva. Usa `/swade initiative new` prima di usare Hold.",
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
    let comment_suffix = comment_line(comment.as_deref());
    ctx.say(format!(
        "{}{}",
        format_initiative_hold(&draw),
        comment_suffix
    ))
    .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "enemy-draw",
    check = "crate::commands::ensure_admin"
)]
pub async fn enemy_draw(
    ctx: Context<'_>,
    #[description = "Nomi separati da ;"] names: String,
    #[description = "Commento opzionale"] comment: Option<String>,
) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    let Some(mut session) = ctx.data().db.get_active_initiative_session(guild_id)? else {
        crate::commands::send_ephemeral_reply(
            ctx,
            "⚠️ Non c'è una sessione di iniziativa attiva. Usa `/swade initiative new` prima di pescare per i nemici.",
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
    let comment_suffix = comment_line(comment.as_deref());
    ctx.say(format!(
        "{}{}",
        format_enemy_draw_result(&result, session.round),
        comment_suffix
    ))
    .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "enemy-hold",
    check = "crate::commands::ensure_admin"
)]
pub async fn enemy_hold(
    ctx: Context<'_>,
    #[description = "Nomi separati da ;"] names: String,
    #[description = "Commento opzionale"] comment: Option<String>,
) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    let Some(mut session) = ctx.data().db.get_active_initiative_session(guild_id)? else {
        crate::commands::send_ephemeral_reply(
            ctx,
            "⚠️ Non c'è una sessione di iniziativa attiva. Usa `/swade initiative new` prima di usare `enemy hold`.",
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
    let comment_suffix = comment_line(comment.as_deref());
    ctx.say(format!(
        "{}{}",
        format_enemy_hold_result(&result),
        comment_suffix
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command, check = "crate::commands::ensure_admin")]
pub async fn round(
    ctx: Context<'_>,
    #[description = "Commento opzionale"] comment: Option<String>,
) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    let Some(mut session) = ctx.data().db.get_active_initiative_session(guild_id)? else {
        crate::commands::send_ephemeral_reply(
            ctx,
            "⚠️ Non c'è una sessione di iniziativa attiva. Usa `/swade initiative new` prima di chiudere un round.",
        )
        .await?;
        return Ok(());
    };

    let resolution = match session.resolve_round() {
        Ok(resolution) => resolution,
        Err(err) => match err.downcast_ref::<InitiativeError>() {
            Some(InitiativeError::NoDrawsThisRound) => {
                crate::commands::send_ephemeral_reply(
                    ctx,
                    "⚠️ Nessuno ha ancora pescato in questo round.",
                )
                .await?;
                return Ok(());
            }
            _ => return Err(err.into()),
        },
    };

    let mut awarded_bennies = Vec::new();
    for participant in &resolution.benny_recipients {
        if participant.user_id == 0 {
            continue;
        }

        let new_total = ctx
            .data()
            .db
            .add_bennies(guild_id, participant.user_id, 1)?;
        awarded_bennies.push((participant.display_name.clone(), new_total));
    }

    ctx.data()
        .db
        .save_active_initiative_session(guild_id, &session)?;
    let comment_suffix = comment_line(comment.as_deref());
    ctx.say(format!(
        "{}{}",
        format_round_resolution(&resolution, &awarded_bennies),
        comment_suffix
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command, check = "crate::commands::ensure_admin")]
pub async fn end(
    ctx: Context<'_>,
    #[description = "Commento opzionale"] comment: Option<String>,
) -> Result<(), Error> {
    let guild_id = crate::commands::require_guild_id(&ctx)?;
    let comment_suffix = comment_line(comment.as_deref());
    if ctx.data().db.end_initiative_session(guild_id)? {
        ctx.say(format!(
            "🛑 **Iniziativa terminata**\nLa sessione di combattimento attiva è stata chiusa.{}",
            comment_suffix
        ))
        .await?;
    } else {
        ctx.say(format!(
            "ℹ️ Non c'è nessuna sessione di iniziativa attiva da chiudere.{}",
            comment_suffix
        ))
        .await?;
    }

    Ok(())
}
