use crate::{Context, Error};

/// Comandi SWADE del bot.
#[poise::command(
    slash_command,
    subcommands(
        "crate::commands::rolls::trait_roll",
        "crate::commands::rolls::extra",
        "crate::commands::rolls::damage",
        "crate::commands::benny::benny",
        "crate::commands::initiative::initiative",
        "help_cmd"
    ),
    check = "crate::commands::ensure_allowed_channel"
)]
pub async fn swade(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Mostra il riepilogo rapido dei comandi.
#[poise::command(slash_command, rename = "help")]
pub async fn help_cmd(ctx: Context<'_>) -> Result<(), Error> {
    let content = format!(
        r#"**SWADE Discord Bot**
Solo nel canale `#{}`.

`/swade help`
Aiuto rapido.

**Tiri**
`/swade trait die:d0|d4|d6|d8|d10|d12 mod:int tn:int name:text`
PG/Wild Card. `d0` = unskilled. Es: `/swade trait die:d8 name:Spellcasting`

`/swade extra die:d0|d4|d6|d8|d10|d12 mod:int tn:int name:text`
Extra. `d0` = unskilled. Es: `/swade extra die:d6 name:Goblin Fighting`

`/swade damage attr_die:dX weapon_die:dY toughness:int mod:int ap:int name:text`
Danno. Es: `/swade damage attr_die:d8 weapon_die:d6 toughness:7 name:Sword`

**Bennies**
`/swade benny list`
`/swade benny give user:@user amount:int reason:text` `admin`
`/swade benny spend user:@user amount:int reason:text` `admin`
`/swade benny reset amount:int` `admin`

**Initiative**
`/swade initiative new` `admin`
`/swade initiative draw`
`/swade initiative hold`
`/swade initiative enemy-draw names:"Goblin 1; Troll"` `admin`
`/swade initiative enemy-hold names:"Goblin 1; Troll"` `admin`
`/swade initiative round` `admin`
`/swade initiative end` `admin`

**Note**
`TN` = Target Number, il valore da raggiungere o superare
`TN` base di solito `4`
`mod` = modificatore
`ap` = armor piercing
`d0` = unskilled, usa `d4` e `-2`
`hold` richiede una draw prima
`Wild Die` = d6 extra dei PG/Wild Card
`Raise` = 1 ogni `4` punti pieni sopra il bersaglio
Formula trait: `miglior dado + mod`
Formula raises: `floor((totale - bersaglio) / 4)`, solo se `totale >= bersaglio`
`Critical Failure` = doppio `1` su Trait Die + Wild Die
Joker player: +1 Benny solo a chi ha pescato nel round
"#,
        ctx.data().allowed_channel_name
    );

    ctx.send(
        poise::CreateReply::default()
            .content(content)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
