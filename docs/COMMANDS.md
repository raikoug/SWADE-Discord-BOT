# Commands Spec

## Regola generale

- Comandi e sigle in inglese.
- Output in italiano.
- `/swade help` sempre ephemeral/personale.
- Il bot esegue i comandi solo nel canale `#swade-bot` per default.
- In DM o fuori canale consentito il comando viene rifiutato con risposta ephemeral.
- I dati persistenti del bot restano sotto `~/.swadedsbot/` salvo override di configurazione.
- I comandi operativi possono accettare un `comment` opzionale riportato nell'output.

## `/trait`

Per PG e Wild Cards.

```text
/trait die:d4|d6|d8|d10|d12 mod:int=0 tn:int=4 name:string comment:string
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
/extra die:d4|d6|d8|d10|d12 mod:int=0 tn:int=4 name:string comment:string
```

Regole:

- tira solo Trait Die;
- il dado esplode;
- niente Wild Die.

## `/damage`

```text
/damage dice:string toughness:int mod:int=0 ap:int=0 name:string comment:string
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

```text
/benny list comment:string
```

Mostra Bennies tracciati nel server.
Disponibile a tutti gli utenti del server.

## `/benny reset`

```text
/benny reset amount:int=3 comment:string
```

Aggiorna solo player già tracciati.
Disponibile solo ad admin o utenti con permessi `Manage Server`.

## `/swade help`

Risposta ephemeral/personale in italiano.

## `/initiative`

L'iniziativa usa carte da poker, non dadi.

### Flusso consigliato

```text
/initiative new
/initiative draw
/initiative hold
/initiative enemy draw names:"Goblin 1; Goblin 2; Troll"
/initiative enemy hold names:"Goblin 1; Troll"
/initiative round
/initiative end
```

### `/initiative new`

Comando admin-only.

```text
/initiative new comment:string
```

- avvia una sessione attiva
- parte da round 1
- prepara un mazzo da poker completo con 2 Jokers
- se una sessione è già attiva, chiede di usare prima `/initiative end`

### `/initiative draw`

Comando player.

```text
/initiative draw comment:string
```

- pesca una carta per l'utente Discord nel round corrente
- impedisce una seconda draw nello stesso round
- usa il display name Discord nell'output

### `/initiative hold`

Comando player.

```text
/initiative hold comment:string
```

- funziona solo se il player ha già pescato nel round corrente
- marca la draw attuale come Hold
- non crea una nuova draw

### `/initiative enemy draw`

Comando admin-only.

```text
/initiative enemy draw names:string comment:string
```

Regole:

- `names` è una stringa singola con nomi separati da `;`
- gli spazi interni ai nomi sono preservati
- i nomi vuoti sono ignorati
- se non resta nessun nome valido, il comando viene rifiutato
- i nomi già presenti nel round vengono saltati e riportati
- i duplicati nello stesso comando vengono riportati

### `/initiative enemy hold`

Comando admin-only.

```text
/initiative enemy hold names:string comment:string
```

Regole:

- `names` usa lo stesso parsing separato da `;`
- il comportamento è stretto
- mette in Hold solo nemici già presenti nel round corrente
- i nomi mancanti vengono riportati come possibile refuso
- non crea nemici mancanti
- non usa fuzzy match

### `/initiative round`

Comando admin-only.

```text
/initiative round comment:string
```

- richiede almeno una draw nel round corrente
- ordina per SWADE: Joker, poi A, K, Q, J, 10..2
- a parità di rango ordina per seme: ♠, ♥, ♦, ♣
- evidenzia le draw in Hold
- se compare qualunque Joker, il mazzo viene rimescolato dopo il round
- se non compare Joker, le carte pescate restano scartate
- se almeno un player pesca un Joker, +1 Benny solo ai player che hanno pescato nel round corrente

### `/initiative end`

Comando admin-only.

```text
/initiative end comment:string
```

- termina la sessione attiva
- chiude il combattimento corrente senza richiedere cleanup manuale
