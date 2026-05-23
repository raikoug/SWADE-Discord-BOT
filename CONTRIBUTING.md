# Contributing

## Flusso consigliato

```bash
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Stile codice

- Logica SWADE in `src/dice.rs`.
- Output utente in `src/formatting.rs`.
- Handler Discord in `src/commands/`.
- Preferisci funzioni piccole e testabili.

## Documentazione

Aggiorna questi file quando cambi comportamento:

- `README.md`
- `docs/COMMANDS.md`
- `docs/SWADE_RULES.md`
- `/help` in `src/commands/help.rs`
