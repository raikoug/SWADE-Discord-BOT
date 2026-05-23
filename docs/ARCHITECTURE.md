# Architettura

```text
Discord Gateway WebSocket
        │
        ▼
poise / serenity
        │
        ▼
src/commands/*
        │
        ├── guard comandi      canale `#swade-bot`, permessi admin Benny
        │
        ├── src/dice.rs          logica SWADE pura
        ├── src/formatting.rs    output Discord in italiano
        └── src/db.rs            SQLite
```

## Moduli

| File | Responsabilità |
|---|---|
| `src/main.rs` | bootstrap, logging, framework Discord |
| `src/config.rs` | env vars, nome canale consentito e directory dati `~/.swadedsbot/` |
| `src/dice.rs` | dadi esplosivi, Trait Roll, Damage Roll |
| `src/formatting.rs` | messaggi in italiano |
| `src/db.rs` | SQLite, Bennies, roll history |
| `src/commands/mod.rs` | registry comandi e guard riusabili per canale/permessi |
| `src/commands/rolls.rs` | `/trait`, `/extra`, `/damage` |
| `src/commands/benny.rs` | `/benny ...`, con mutazioni limitate agli admin |
| `src/commands/help.rs` | `/help` ephemeral |

## Principio guida

I comandi devono essere sottili:

1. leggono parametri Discord;
2. chiamano logica dominio;
3. formattano output;
4. salvano storico dove utile;
5. rispondono.

La logica SWADE non deve vivere dentro gli handler Discord.
I controlli trasversali di accesso devono vivere in guard condivise, non copiati in ogni comando.
