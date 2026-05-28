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
/initiative new
/initiative draw
/initiative hold
/initiative enemy draw names:string
/initiative enemy hold names:string
/initiative round
/initiative end
/swade help
```

`/swade help` risponde in modo personale/ephemeral, quindi non sporca il canale.
Il bot esegue i comandi solo nel canale `#swade-bot` oppure nel nome configurato tramite `SWADEDSBOT_ALLOWED_CHANNEL`.
I comandi operativi supportano anche `comment` opzionale, che viene riportato nel messaggio Discord.

Permessi Bennies:

- `/benny give`: solo admin o utenti con `Manage Server`;
- `/benny spend`: solo admin o utenti con `Manage Server`;
- `/benny reset`: solo admin o utenti con `Manage Server`;
- `/benny list`: disponibile a tutti.

Flusso iniziativa:

- admin: `/initiative new`
- player: `/initiative draw`
- player: `/initiative hold` opzionale, ma solo dopo una draw
- admin: `/initiative enemy draw names:"Goblin 1; Goblin 2; Troll"`
- admin: `/initiative enemy hold names:"Goblin 1; Troll"`
- admin: `/initiative round`
- admin: `/initiative end`

L'iniziativa usa carte da poker, non dadi. I nomi enemy sono separati da `;`.
`/initiative enemy hold` è stretto: non crea nomi mancanti e li segnala come possibile refuso.
Qualsiasi Joker rimescola il mazzo dopo la risoluzione del round. Se un player pesca almeno un Joker, il bot assegna +1 Benny solo ai player che hanno pescato nel round corrente.

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

## Deploy

Esempio minimale su VPS Linux con `systemd`.

1. Crea l'utente di servizio, la directory applicativa e la directory dati usata dal servizio:

```bash
sudo useradd --system --create-home --home-dir /home/swadebot swadebot
sudo mkdir -p /opt/swadedsbot
sudo mkdir -p /home/swadebot/.swadedsbot
sudo chown -R swadebot:swadebot /opt/swadedsbot /home/swadebot
```

2. Compila il binario in release e copia i file necessari:

```bash
cargo build --release
sudo cp target/release/swadedsbot /opt/swadedsbot/
sudo cp .env.example /opt/swadedsbot/.env
sudo editor /opt/swadedsbot/.env
sudo chown -R swadebot:swadebot /opt/swadedsbot /home/swadebot
sudo chmod 755 /opt/swadedsbot /home/swadebot /home/swadebot/.swadedsbot
```

Nel file `.env` del servizio imposta almeno:

```bash
DISCORD_TOKEN=...
SWADEDSBOT_DATA_DIR=/home/swadebot/.swadedsbot
```

3. Crea il file di unità `systemd` in `/etc/systemd/system/swadedsbot.service`:

```ini
[Unit]
Description=SWADE Discord Bot
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=swadebot
Group=swadebot
WorkingDirectory=/opt/swadedsbot
EnvironmentFile=/opt/swadedsbot/.env
ExecStart=/opt/swadedsbot/swadedsbot
Restart=always
RestartSec=5
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=full
ProtectHome=false
ReadWritePaths=/home/swadebot/.swadedsbot /opt/swadedsbot

[Install]
WantedBy=multi-user.target
```

4. Ricarica `systemd`, abilita il servizio e controlla i log:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now swadedsbot
sudo journalctl -u swadedsbot -f
```

Se il servizio non parte, controlla prima che esistano davvero:

- `/opt/swadedsbot/swadedsbot`
- `/home/swadebot/.swadedsbot`

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
