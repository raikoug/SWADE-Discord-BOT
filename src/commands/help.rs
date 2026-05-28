use crate::{Context, Error};

#[poise::command(
    slash_command,
    subcommands("help_cmd"),
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
`/trait die:dX mod:int tn:int name:text comment:text`
PG/Wild Card. Es: `/trait die:d8 name:Spellcasting`

`/extra die:dX mod:int tn:int name:text comment:text`
Extra. Es: `/extra die:d6 name:Goblin Fighting`

`/damage dice:XdY toughness:int mod:int ap:int name:text comment:text`
Danno. Es: `/damage dice:2d6 toughness:7 name:Sword`

**Bennies**
`/benny list comment:text`
`/benny give user:@user amount:int reason:text` `admin`
`/benny spend user:@user amount:int reason:text` `admin`
`/benny reset amount:int comment:text` `admin`

**Initiative**
`/initiative new comment:text` `admin`
`/initiative draw comment:text`
`/initiative hold comment:text`
`/initiative enemy draw names:"Goblin 1; Troll" comment:text` `admin`
`/initiative enemy hold names:"Goblin 1; Troll" comment:text` `admin`
`/initiative round comment:text` `admin`
`/initiative end comment:text` `admin`

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
