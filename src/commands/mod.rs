pub mod benny;
pub mod help;
pub mod rolls;

use crate::{Data, Error};

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![
        rolls::trait_roll(),
        rolls::extra(),
        rolls::damage(),
        benny::benny(),
        help::help_cmd(),
    ]
}

pub async fn ensure_allowed_channel(ctx: crate::Context<'_>) -> Result<bool, Error> {
    let allowed_channel_name = ctx.data().allowed_channel_name.clone();

    let Some(channel) = ctx.guild_channel().await else {
        send_ephemeral_reply(
            ctx,
            &format!(
                "🎲 Questo bot funziona solo in un server Discord, nel canale #{}.",
                allowed_channel_name
            ),
        )
        .await?;
        return Ok(false);
    };

    if channel.name != allowed_channel_name {
        send_ephemeral_reply(
            ctx,
            &format!(
                "🎲 Questo bot funziona solo nel canale #{}.",
                ctx.data().allowed_channel_name
            ),
        )
        .await?;
        return Ok(false);
    }

    Ok(true)
}

pub async fn ensure_benny_admin(ctx: crate::Context<'_>) -> Result<bool, Error> {
    let Some(channel) = ctx.guild_channel().await else {
        send_benny_admin_denied(ctx).await?;
        return Ok(false);
    };
    let Some(member) = ctx.author_member().await else {
        send_benny_admin_denied(ctx).await?;
        return Ok(false);
    };
    let Some(guild) = ctx.partial_guild().await else {
        send_benny_admin_denied(ctx).await?;
        return Ok(false);
    };

    let permissions = guild.user_permissions_in(&channel, member.as_ref());
    if permissions.administrator() || permissions.manage_guild() {
        return Ok(true);
    }

    send_benny_admin_denied(ctx).await?;
    Ok(false)
}

async fn send_benny_admin_denied(ctx: crate::Context<'_>) -> Result<(), Error> {
    send_ephemeral_reply(
        ctx,
        "❌ Comando riservato agli admin del server.\n\nSolo utenti con permessi Administrator o Manage Server possono modificare i Bennies.",
    )
    .await
}

async fn send_ephemeral_reply(ctx: crate::Context<'_>, content: &str) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default()
            .content(content)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
