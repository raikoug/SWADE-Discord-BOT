use crate::{Context, Error};

#[poise::command(
    slash_command,
    rename = "help",
    check = "crate::commands::ensure_allowed_channel"
)]
pub async fn help_cmd(ctx: Context<'_>) -> Result<(), Error> {
    let content = format!(
        r#"# SWADE Discord Bot

Il bot funziona solo nel canale `#{}`.

## `/trait`
Tiro per PG e Wild Cards.

Usa **Trait Die + Wild Die d6**, dadi esplosivi, tiene il migliore, applica **Mod**, confronta con **TN**, calcola i **Raises**.

Esempi:
`/trait die:d8 name:Spellcasting`
`/trait die:d10 mod:-2 name:Shooting`
`/trait die:d8 tn:6 name:Fighting`

## `/extra`
Tiro per Extras, cioè PNG minori e comparse.

Non usa Wild Die.

Esempio:
`/extra die:d6 name:Goblin Fighting`

## `/damage`
Tiro danni. Non usa Wild Die. I dadi esplodono.

Esempi:
`/damage dice:2d6 toughness:7 name:Sword`
`/damage dice:2d8 mod:2 toughness:9 name:Fireball`
`/damage dice:3d6 toughness:10 ap:2 name:Rifle`

## `/benny give`
Aggiunge Bennies a un player.
Comando riservato ad admin con permessi Administrator o Manage Server.

Esempio:
`/benny give user:@Marco amount:1 reason:Hindrance giocato bene`

## `/benny spend`
Spende Bennies per un player.
Comando riservato ad admin con permessi Administrator o Manage Server.

Esempio:
`/benny spend user:@Marco reason:Soak Roll`

## `/benny list`
Mostra i Bennies attuali.
Disponibile a tutti gli utenti del server.

## `/benny reset`
Resetta i Bennies dei player tracciati.
Comando riservato ad admin con permessi Administrator o Manage Server.

Esempio:
`/benny reset amount:3`

## Sigle rapide
- **TN**: Target Number. Default 4.
- **Mod**: modificatore situazionale.
- **AP**: Armor Piercing.
- **Wild Die**: d6 extra dei PG/Wild Cards nei Trait Roll.
- **Raise**: ogni 4 punti sopra il TN.
- **Critical Failure**: doppio 1 su Trait Die e Wild Die.
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
