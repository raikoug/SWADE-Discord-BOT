use crate::dice::{parse_damage_notation, roll_damage, roll_trait, Die};
use crate::formatting::{comment_line, format_damage_roll, format_trait_roll};
use crate::{Context, Error};

#[poise::command(
    slash_command,
    rename = "trait",
    check = "crate::commands::ensure_allowed_channel"
)]
pub async fn trait_roll(
    ctx: Context<'_>,
    #[description = "Dado del Trait o della Skill"] die: Die,
    #[description = "Modificatore situazionale"]
    #[rename = "mod"]
    r#mod: Option<i32>,
    #[description = "Target Number, default 4"] tn: Option<i32>,
    #[description = "Nome del tiro, esempio Spellcasting"] name: Option<String>,
    #[description = "Commento opzionale"] comment: Option<String>,
) -> Result<(), Error> {
    let modifier = r#mod.unwrap_or(0);
    let tn = tn.unwrap_or(4);
    let roll_name = name.unwrap_or_else(|| "Trait Roll".to_string());
    let actor = ctx.author().display_name().to_string();

    let roll = roll_trait(die, true, modifier, tn);
    let message = format!(
        "{}{}",
        format_trait_roll(&actor, &roll_name, &roll),
        comment_line(comment.as_deref())
    );

    save_history(&ctx, "trait", &message)?;
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(slash_command, check = "crate::commands::ensure_allowed_channel")]
pub async fn extra(
    ctx: Context<'_>,
    #[description = "Dado del Trait o della Skill"] die: Die,
    #[description = "Modificatore situazionale"]
    #[rename = "mod"]
    r#mod: Option<i32>,
    #[description = "Target Number, default 4"] tn: Option<i32>,
    #[description = "Nome del tiro, esempio Goblin Fighting"] name: Option<String>,
    #[description = "Commento opzionale"] comment: Option<String>,
) -> Result<(), Error> {
    let modifier = r#mod.unwrap_or(0);
    let tn = tn.unwrap_or(4);
    let roll_name = name.unwrap_or_else(|| "Extra Roll".to_string());
    let actor = ctx.author().display_name().to_string();

    let roll = roll_trait(die, false, modifier, tn);
    let message = format!(
        "{}{}",
        format_trait_roll(&actor, &roll_name, &roll),
        comment_line(comment.as_deref())
    );

    save_history(&ctx, "extra", &message)?;
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(slash_command, check = "crate::commands::ensure_allowed_channel")]
pub async fn damage(
    ctx: Context<'_>,
    #[description = "Dadi danno, esempio 2d6, 3d8, d12"] dice: String,
    #[description = "Toughness del bersaglio"] toughness: i32,
    #[description = "Modificatore al danno"]
    #[rename = "mod"]
    r#mod: Option<i32>,
    #[description = "Armor Piercing"] ap: Option<i32>,
    #[description = "Nome del danno, esempio Sword o Fireball"] name: Option<String>,
    #[description = "Commento opzionale"] comment: Option<String>,
) -> Result<(), Error> {
    let modifier = r#mod.unwrap_or(0);
    let armor_piercing = ap.unwrap_or(0).max(0);
    let damage_name = name.unwrap_or_else(|| "Damage".to_string());
    let notation = parse_damage_notation(&dice)?;
    let roll = roll_damage(notation, modifier, toughness, armor_piercing);
    let actor = ctx.author().display_name().to_string();

    let message = format!(
        "{}{}",
        format_damage_roll(&actor, &damage_name, &roll),
        comment_line(comment.as_deref())
    );

    save_history(&ctx, "damage", &message)?;
    ctx.say(message).await?;
    Ok(())
}

fn save_history(ctx: &Context<'_>, command_name: &str, summary: &str) -> anyhow::Result<()> {
    ctx.data().db.insert_roll_history(
        ctx.guild_id().map(|id| id.get()),
        ctx.channel_id().get(),
        ctx.author().id.get(),
        command_name,
        summary,
    )
}
