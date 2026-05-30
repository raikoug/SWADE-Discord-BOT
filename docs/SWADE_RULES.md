# SWADE Rules Implementate

Questo documento riassume solo le regole implementate nel bot. Non sostituisce il manuale.

## Trait Roll

Per PG e Wild Cards:

```text
Trait Die + Wild Die d6
```

Si tiene il risultato migliore. Entrambi i dadi esplodono.

## Extra Roll

Per Extras:

```text
Trait Die
```

Niente Wild Die.

## TN

Target Number. Default: `4`.

## Raise

Ogni 4 punti sopra il TN è un Raise.

```text
raises = floor((final_total - tn) / 4)
```

Solo se `final_total >= tn`.

## Critical Failure

Per Wild Cards: doppio 1 su Trait Die e Wild Die.

Il modificatore non salva il Critical Failure.

## Damage

Il danno non usa Wild Die. I dadi danno esplodono.

MVP:

```text
effective_toughness = toughness - ap
```

Poi:

```text
damage < effective_toughness       => No effect
damage >= effective_toughness      => Shaken
damage >= effective_toughness + 4  => Shaken + 1 Wound
damage >= effective_toughness + 8  => Shaken + 2 Wounds
```

Nota: il bot non traccia ancora se il bersaglio fosse già Shaken.

## Initiative

L'iniziativa usa un mazzo da poker standard con:

- 52 carte regolari
- 2 Jokers

Ordine SWADE:

```text
Joker
Ace
King
Queen
Jack
10..2
```

Ordine semi a parità:

```text
Spades > Hearts > Diamonds > Clubs
```

Effetti MVP:

- i player pescano con `/swade initiative draw`
- possono andare in Hold con `/swade initiative hold` solo dopo aver pescato
- gli enemy usano `/swade initiative enemy draw` e `/swade initiative enemy hold`
- se compare qualunque Joker, il mazzo viene rimescolato dopo `/swade initiative round`
- se non compare Joker, le carte del round restano scartate
- se almeno un player pesca un Joker, il bot assegna +1 Benny solo ai player che hanno pescato in quel round
