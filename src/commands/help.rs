use crate::{Context, Error};

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

#[poise::command(slash_command, rename = "help")]
pub async fn help_cmd(ctx: Context<'_>) -> Result<(), Error> {
    let content = format!(
        r#"**SWADE Discord Bot**
Solo nel canale `#{}`.

`/swade help`
Aiuto rapido.

**Tiri**
`/swade trait die:dX mod:int tn:int name:text comment:text`
PG/Wild Card. Es: `/swade trait die:d8 name:Spellcasting`

`/swade extra die:dX mod:int tn:int name:text comment:text`
Extra. Es: `/swade extra die:d6 name:Goblin Fighting`

`/swade damage dice:XdY toughness:int mod:int ap:int name:text comment:text`
Danno. Es: `/swade damage dice:2d6 toughness:7 name:Sword`

**Bennies**
`/swade benny list comment:text`
`/swade benny give user:@user amount:int reason:text` `admin`
`/swade benny spend user:@user amount:int reason:text` `admin`
`/swade benny reset amount:int comment:text` `admin`

**Initiative**
`/swade initiative new comment:text` `admin`
`/swade initiative draw comment:text`
`/swade initiative hold comment:text`
`/swade initiative enemy draw names:"Goblin 1; Troll" comment:text` `admin`
`/swade initiative enemy hold names:"Goblin 1; Troll" comment:text` `admin`
`/swade initiative round comment:text` `admin`
`/swade initiative end comment:text` `admin`

**Note**
`TN` = Target Number, il valore da raggiungere o superare
`TN` base di solito `4`
`mod` = modificatore
`ap` = armor piercing
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
