# Commands Spec

## Regola generale

- Comandi e sigle in inglese.
- Output in italiano.
- `/help` sempre ephemeral/personale.
- Il bot esegue i comandi solo nel canale `#swade-bot` per default.
- In DM o fuori canale consentito il comando viene rifiutato con risposta ephemeral.
- I dati persistenti del bot restano sotto `~/.swadedsbot/` salvo override di configurazione.

## `/trait`

Per PG e Wild Cards.

```text
/trait die:d4|d6|d8|d10|d12 mod:int=0 tn:int=4 name:string
```

Regole:

- tira Trait Die;
- tira Wild Die d6;
- entrambi esplodono;
- tiene il migliore;
- applica Mod;
- confronta con TN;
- calcola Raises;
- doppio 1 = Critical Failure.

## `/extra`

Per Extras.

```text
/extra die:d4|d6|d8|d10|d12 mod:int=0 tn:int=4 name:string
```

Regole:

- tira solo Trait Die;
- il dado esplode;
- niente Wild Die.

## `/damage`

```text
/damage dice:string toughness:int mod:int=0 ap:int=0 name:string
```

Esempi `dice`:

- `2d6`
- `3d8`
- `d12`

Regole MVP:

- niente Wild Die;
- dadi esplosivi;
- `AP` viene sottratto direttamente alla Toughness passata;
- danno finale sotto Toughness effettiva = nessun effetto;
- danno finale almeno Toughness effettiva = Shaken;
- ogni Raise sopra Toughness effettiva = Wound.

## `/benny give`

```text
/benny give user:@user amount:int=1 reason:string
```

Disponibile solo ad admin o utenti con permessi `Manage Server`.

## `/benny spend`

```text
/benny spend user:@user amount:int=1 reason:string
```

Non permette di andare sotto zero.
Disponibile solo ad admin o utenti con permessi `Manage Server`.

## `/benny list`

Mostra Bennies tracciati nel server.
Disponibile a tutti gli utenti del server.

## `/benny reset`

```text
/benny reset amount:int=3
```

Aggiorna solo player già tracciati.
Disponibile solo ad admin o utenti con permessi `Manage Server`.

## `/help`

Risposta ephemeral/personale in italiano.
