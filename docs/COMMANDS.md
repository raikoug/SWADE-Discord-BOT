# Commands Spec

## Regola generale

- Comandi e sigle in inglese.
- Output in italiano.
- `/swade help` sempre ephemeral/personale.
- Il bot esegue i comandi solo nel canale `#swade-bot` per default.
- In DM o fuori canale consentito il comando viene rifiutato con risposta ephemeral.
- I dati persistenti del bot restano sotto `~/.swadedsbot/` salvo override di configurazione.

## `/swade trait`

Per PG e Wild Cards.

```text
/swade trait die:d4|d6|d8|d10|d12 mod:int=0 tn:int=4 name:string
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

## `/swade extra`

Per Extras.

```text
/swade extra die:d4|d6|d8|d10|d12 mod:int=0 tn:int=4 name:string
```

Regole:

- tira solo Trait Die;
- il dado esplode;
- niente Wild Die.

## `/swade damage`

```text
/swade damage dice:string toughness:int mod:int=0 ap:int=0 name:string
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

## `/swade benny give`

```text
/swade benny give user:@user amount:int=1 reason:string
```

Disponibile solo ad admin o utenti con permessi `Manage Server`.

## `/swade benny spend`

```text
/swade benny spend user:@user amount:int=1 reason:string
```

Non permette di andare sotto zero.
Disponibile solo ad admin o utenti con permessi `Manage Server`.

## `/swade benny list`

```text
/swade benny list
```

Mostra Bennies tracciati nel server.
Disponibile a tutti gli utenti del server.

## `/swade benny reset`

```text
/swade benny reset amount:int=3
```

Aggiorna solo player già tracciati.
Disponibile solo ad admin o utenti con permessi `Manage Server`.

## `/swade help`

Risposta ephemeral/personale in italiano.

## `/swade initiative`

L'iniziativa usa carte da poker, non dadi.

### Flusso consigliato

```text
/swade initiative new
/swade initiative draw
/swade initiative hold
/swade initiative enemy-draw names:"Goblin 1; Goblin 2; Troll"
/swade initiative enemy-hold names:"Goblin 1; Troll"
/swade initiative round
/swade initiative end
```

### `/swade initiative new`

Comando admin-only.

```text
/swade initiative new
```

- avvia una sessione attiva
- parte da round 1
- prepara un mazzo da poker completo con 2 Jokers
- se una sessione è già attiva, chiede di usare prima `/swade initiative end`

### `/swade initiative draw`

Comando player.

```text
/swade initiative draw
```

- pesca una carta per l'utente Discord nel round corrente
- impedisce una seconda draw nello stesso round
- usa il display name Discord nell'output

### `/swade initiative hold`

Comando player.

```text
/swade initiative hold
```

- funziona solo se il player ha già pescato nel round corrente
- marca la draw attuale come Hold
- non crea una nuova draw

### `/swade initiative enemy-draw`

Comando admin-only.

```text
/swade initiative enemy-draw names:string
```

Regole:

- `names` è una stringa singola con nomi separati da `;`
- gli spazi interni ai nomi sono preservati
- i nomi vuoti sono ignorati
- se non resta nessun nome valido, il comando viene rifiutato
- i nomi già presenti nel round vengono saltati e riportati
- i duplicati nello stesso comando vengono riportati

### `/swade initiative enemy-hold`

Comando admin-only.

```text
/swade initiative enemy-hold names:string
```

Regole:

- `names` usa lo stesso parsing separato da `;`
- il comportamento è stretto
- mette in Hold solo nemici già presenti nel round corrente
- i nomi mancanti vengono riportati come possibile refuso
- non crea nemici mancanti
- non usa fuzzy match

### `/swade initiative round`

Comando admin-only.

```text
/swade initiative round
```

- richiede almeno una draw nel round corrente
- ordina per SWADE: Joker, poi A, K, Q, J, 10..2
- a parità di rango ordina per seme: ♠, ♥, ♦, ♣
- evidenzia le draw in Hold
- se compare qualunque Joker, il mazzo viene rimescolato dopo il round
- se non compare Joker, le carte pescate restano scartate
- se almeno un player pesca un Joker, +1 Benny solo ai player che hanno pescato nel round corrente

### `/swade initiative end`

Comando admin-only.

```text
/swade initiative end
```

- termina la sessione attiva
- chiude il combattimento corrente senza richiedere cleanup manuale
