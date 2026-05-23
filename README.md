# SWADE Discord Bot

Bot Discord in Rust per **Savage Worlds Adventure Edition**.

Obiettivo MVP:

- connessione Discord via Gateway WebSocket;
- slash commands;
- output in italiano;
- comandi in inglese con sigle SWADE;
- SQLite sotto `~/.swadedsbot/`;
- esecuzione solo nel canale `#swade-bot` per default;
- nessuna gestione schede personaggio nel primo giro.

## Comandi inclusi

```text
/trait die:d4|d6|d8|d10|d12 mod:int=0 tn:int=4 name:string
/extra die:d4|d6|d8|d10|d12 mod:int=0 tn:int=4 name:string
/damage dice:string toughness:int mod:int=0 ap:int=0 name:string
/benny give user:@user amount:int=1 reason:string
/benny spend user:@user amount:int=1 reason:string
/benny list
/benny reset amount:int=3
/help
```

`/help` risponde in modo personale/ephemeral, quindi non sporca il canale.
Il bot esegue i comandi solo nel canale `#swade-bot` oppure nel nome configurato tramite `SWADEDSBOT_ALLOWED_CHANNEL`.

Permessi Bennies:

- `/benny give`: solo admin o utenti con `Manage Server`;
- `/benny spend`: solo admin o utenti con `Manage Server`;
- `/benny reset`: solo admin o utenti con `Manage Server`;
- `/benny list`: disponibile a tutti.

## Requisiti

- Rust stable.
- Un'applicazione Discord con bot token.
- Scope consigliati per invito: `bot` e `applications.commands`.

## Setup rapido

```bash
cp .env.example .env
$EDITOR .env
cargo run
```

Durante lo sviluppo imposta `GUILD_ID` per registrare i comandi solo in un server. La registrazione globale può richiedere più tempo a propagarsi.

## Variabili ambiente

| Variabile | Obbligatoria | Descrizione |
|---|---:|---|
| `DISCORD_TOKEN` | sì | Token del bot Discord |
| `GUILD_ID` | no | Server Discord dove registrare i comandi in sviluppo |
| `SWADEDSBOT_DATA_DIR` | no | Override directory dati, default `~/.swadedsbot/` |
| `SWADEDSBOT_ALLOWED_CHANNEL` | no | Nome canale consentito, default `swade-bot` |
| `RUST_LOG` | no | Logging, default `info` |

## Database

Default:

```text
~/.swadedsbot/swadedsbot.sqlite
```

Tabelle iniziali:

- `bennies`
- `roll_history`

## Comandi sviluppo

```bash
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run
```

## Deploy minimale con systemd

Vedi `deploy/swadedsbot.service`.

## Note SWADE implementate

- Trait Roll per PG/Wild Cards: Trait Die + Wild Die d6.
- Dadi esplosivi/Aces.
- TN default 4.
- Raises ogni 4 punti sopra TN.
- Critical Failure su doppio 1 per Wild Cards.
- Damage senza Wild Die.
- Damage contro Toughness con Shaken/Wounds.

## Non incluso nel primo giro

- Schede personaggio persistenti.
- Power Points.
- Iniziativa con carte.
- Stati Shaken/Wounds persistenti.
- Bottoni per reroll/spesa Benny.

Queste parti sono nel TODO, non nella prima carica della cavalleria.
