use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    fn sort_value(self) -> u8 {
        match self {
            Self::Clubs => 1,
            Self::Diamonds => 2,
            Self::Hearts => 3,
            Self::Spades => 4,
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            Self::Clubs => "♣",
            Self::Diamonds => "♦",
            Self::Hearts => "♥",
            Self::Spades => "♠",
        }
    }

    fn code(self) -> &'static str {
        match self {
            Self::Clubs => "C",
            Self::Diamonds => "D",
            Self::Hearts => "H",
            Self::Spades => "S",
        }
    }

    fn from_code(code: &str) -> Result<Self> {
        match code {
            "C" => Ok(Self::Clubs),
            "D" => Ok(Self::Diamonds),
            "H" => Ok(Self::Hearts),
            "S" => Ok(Self::Spades),
            _ => Err(anyhow!("seme non valido: {code}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn sort_value(self) -> u8 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Jack => 11,
            Self::Queen => 12,
            Self::King => 13,
            Self::Ace => 14,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "J",
            Self::Queen => "Q",
            Self::King => "K",
            Self::Ace => "A",
        }
    }

    fn from_label(label: &str) -> Result<Self> {
        match label {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "10" => Ok(Self::Ten),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(anyhow!("rango non valido: {label}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JokerColor {
    Black,
    Red,
}

impl JokerColor {
    fn label(self) -> &'static str {
        match self {
            Self::Black => "Joker Nero",
            Self::Red => "Joker Rosso",
        }
    }

    fn code(self) -> &'static str {
        match self {
            Self::Black => "JB",
            Self::Red => "JR",
        }
    }

    fn from_code(code: &str) -> Result<Self> {
        match code {
            "JB" => Ok(Self::Black),
            "JR" => Ok(Self::Red),
            _ => Err(anyhow!("joker non valido: {code}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Card {
    Standard { rank: Rank, suit: Suit },
    Joker(JokerColor),
}

impl Card {
    pub fn label(self) -> String {
        match self {
            Self::Standard { rank, suit } => format!("{}{}", rank.label(), suit.symbol()),
            Self::Joker(color) => format!("🃏 {}", color.label()),
        }
    }

    pub fn code(self) -> String {
        match self {
            Self::Standard { rank, suit } => format!("{}{}", rank.label(), suit.code()),
            Self::Joker(color) => color.code().to_string(),
        }
    }

    pub fn is_joker(self) -> bool {
        matches!(self, Self::Joker(_))
    }

    fn sort_key(self) -> (u8, u8) {
        match self {
            Self::Joker(_) => (15, 0),
            Self::Standard { rank, suit } => (rank.sort_value(), suit.sort_value()),
        }
    }

    pub fn from_code(code: &str) -> Result<Self> {
        if code == "JB" || code == "JR" {
            return Ok(Self::Joker(JokerColor::from_code(code)?));
        }

        if code.len() < 2 {
            return Err(anyhow!("codice carta non valido: {code}"));
        }

        let (rank_label, suit_label) = code.split_at(code.len() - 1);
        Ok(Self::Standard {
            rank: Rank::from_label(rank_label)?,
            suit: Suit::from_code(suit_label)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticipantKind {
    Player,
    Enemy,
}

impl ParticipantKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Player => "player",
            Self::Enemy => "enemy",
        }
    }

    pub fn from_str(value: &str) -> Result<Self> {
        match value {
            "player" => Ok(Self::Player),
            "enemy" => Ok(Self::Enemy),
            _ => Err(anyhow!("tipo partecipante non valido: {value}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitiativeDraw {
    pub kind: ParticipantKind,
    pub participant_id: String,
    pub display_name: String,
    pub card: Card,
    pub on_hold: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerParticipant {
    pub user_id: u64,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundResolution {
    pub round: u32,
    pub ordered_draws: Vec<InitiativeDraw>,
    pub any_joker: bool,
    pub player_joker: bool,
    pub benny_recipients: Vec<PlayerParticipant>,
    pub next_round: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnemyDrawResult {
    pub drawn: Vec<InitiativeDraw>,
    pub skipped_duplicates: Vec<String>,
    pub skipped_existing: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnemyHoldResult {
    pub held: Vec<InitiativeDraw>,
    pub missing: Vec<String>,
    pub duplicate_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitiativeError {
    AlreadyDrawn,
    HoldWithoutDraw,
    NoValidEnemyNames,
    NotEnoughCards { requested: usize, remaining: usize },
    NoDrawsThisRound,
}

impl std::fmt::Display for InitiativeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyDrawn => write!(f, "partecipante già pescato in questo round"),
            Self::HoldWithoutDraw => write!(f, "non puoi andare in Hold prima di aver pescato"),
            Self::NoValidEnemyNames => {
                write!(
                    f,
                    "devi indicare almeno un nome valido separato da punto e virgola"
                )
            }
            Self::NotEnoughCards {
                requested,
                remaining,
            } => write!(
                f,
                "mazzo insufficiente: richieste {requested} carte, disponibili {remaining}"
            ),
            Self::NoDrawsThisRound => write!(f, "nessuna carta pescata in questo round"),
        }
    }
}

impl std::error::Error for InitiativeError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitiativeSession {
    pub round: u32,
    pub deck: Vec<Card>,
    pub draws: Vec<InitiativeDraw>,
}

impl InitiativeSession {
    pub fn new() -> Self {
        let mut deck = full_deck();
        deck.shuffle(&mut rand::thread_rng());
        Self {
            round: 1,
            deck,
            draws: Vec::new(),
        }
    }

    pub fn from_parts(round: u32, deck: Vec<Card>, draws: Vec<InitiativeDraw>) -> Self {
        Self { round, deck, draws }
    }

    pub fn serialize_deck(&self) -> String {
        serialize_deck(&self.deck)
    }

    pub fn draw_player(&mut self, user_id: u64, display_name: &str) -> Result<InitiativeDraw> {
        let participant_id = user_id.to_string();
        if self.draws.iter().any(|draw| {
            draw.kind == ParticipantKind::Player && draw.participant_id == participant_id
        }) {
            return Err(InitiativeError::AlreadyDrawn.into());
        }

        let card = self.draw_next_card(1)?[0];
        let draw = InitiativeDraw {
            kind: ParticipantKind::Player,
            participant_id,
            display_name: display_name.to_string(),
            card,
            on_hold: false,
        };
        self.draws.push(draw.clone());
        Ok(draw)
    }

    pub fn hold_player(&mut self, user_id: u64) -> Result<InitiativeDraw> {
        let participant_id = user_id.to_string();
        let draw = self
            .draws
            .iter_mut()
            .find(|draw| {
                draw.kind == ParticipantKind::Player && draw.participant_id == participant_id
            })
            .ok_or(InitiativeError::HoldWithoutDraw)?;
        draw.on_hold = true;
        Ok(draw.clone())
    }

    pub fn draw_enemies(&mut self, names: &str) -> Result<EnemyDrawResult> {
        let parsed_names = parse_enemy_names(names)?;
        let mut unique_names = Vec::new();
        let mut seen = HashSet::new();
        let mut skipped_duplicates = Vec::new();
        let mut skipped_existing = Vec::new();

        for name in parsed_names {
            if !seen.insert(name.clone()) {
                skipped_duplicates.push(name);
                continue;
            }

            if self
                .draws
                .iter()
                .any(|draw| draw.kind == ParticipantKind::Enemy && draw.participant_id == name)
            {
                skipped_existing.push(name);
                continue;
            }

            unique_names.push(name);
        }

        let cards = self.draw_next_card(unique_names.len())?;
        let mut drawn = Vec::new();
        for (name, card) in unique_names.into_iter().zip(cards.into_iter()) {
            let draw = InitiativeDraw {
                kind: ParticipantKind::Enemy,
                participant_id: name.clone(),
                display_name: name,
                card,
                on_hold: false,
            };
            self.draws.push(draw.clone());
            drawn.push(draw);
        }

        Ok(EnemyDrawResult {
            drawn,
            skipped_duplicates,
            skipped_existing,
        })
    }

    pub fn hold_enemies(&mut self, names: &str) -> Result<EnemyHoldResult> {
        let parsed_names = parse_enemy_names(names)?;
        let mut seen = HashSet::new();
        let mut held = Vec::new();
        let mut missing = Vec::new();
        let mut duplicate_names = Vec::new();

        for name in parsed_names {
            if !seen.insert(name.clone()) {
                duplicate_names.push(name);
                continue;
            }

            match self
                .draws
                .iter_mut()
                .find(|draw| draw.kind == ParticipantKind::Enemy && draw.participant_id == name)
            {
                Some(draw) => {
                    draw.on_hold = true;
                    held.push(draw.clone());
                }
                None => missing.push(name),
            }
        }

        Ok(EnemyHoldResult {
            held,
            missing,
            duplicate_names,
        })
    }

    pub fn resolve_round(&mut self) -> Result<RoundResolution> {
        if self.draws.is_empty() {
            return Err(InitiativeError::NoDrawsThisRound.into());
        }

        let ordered_draws = sort_draws(&self.draws);
        let any_joker = ordered_draws.iter().any(|draw| draw.card.is_joker());
        let player_joker = ordered_draws
            .iter()
            .any(|draw| draw.kind == ParticipantKind::Player && draw.card.is_joker());

        let benny_recipients = if player_joker {
            ordered_draws
                .iter()
                .filter(|draw| draw.kind == ParticipantKind::Player)
                .map(|draw| PlayerParticipant {
                    user_id: draw.participant_id.parse::<u64>().unwrap_or_default(),
                    display_name: draw.display_name.clone(),
                })
                .collect()
        } else {
            Vec::new()
        };

        if any_joker {
            self.deck = full_deck();
            self.deck.shuffle(&mut rand::thread_rng());
        }

        let resolved_round = self.round;
        self.round += 1;
        self.draws.clear();

        Ok(RoundResolution {
            round: resolved_round,
            ordered_draws,
            any_joker,
            player_joker,
            benny_recipients,
            next_round: self.round,
        })
    }

    fn draw_next_card(&mut self, requested: usize) -> Result<Vec<Card>> {
        if requested == 0 {
            return Ok(Vec::new());
        }

        if self.deck.len() < requested {
            return Err(InitiativeError::NotEnoughCards {
                requested,
                remaining: self.deck.len(),
            }
            .into());
        }

        let mut cards = Vec::with_capacity(requested);
        for _ in 0..requested {
            if let Some(card) = self.deck.pop() {
                cards.push(card);
            }
        }
        Ok(cards)
    }
}

pub fn parse_enemy_names(names: &str) -> Result<Vec<String>> {
    let parsed = names
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    if parsed.is_empty() {
        return Err(InitiativeError::NoValidEnemyNames.into());
    }

    Ok(parsed)
}

pub fn serialize_deck(deck: &[Card]) -> String {
    deck.iter()
        .map(|card| card.code())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn deserialize_deck(serialized: &str) -> Result<Vec<Card>> {
    if serialized.trim().is_empty() {
        return Ok(Vec::new());
    }

    serialized
        .split(',')
        .map(|part| Card::from_code(part.trim()))
        .collect()
}

pub fn full_deck() -> Vec<Card> {
    let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
    let ranks = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    let mut cards = Vec::with_capacity(54);
    for suit in suits {
        for rank in ranks {
            cards.push(Card::Standard { rank, suit });
        }
    }
    cards.push(Card::Joker(JokerColor::Black));
    cards.push(Card::Joker(JokerColor::Red));
    cards
}

pub fn sort_draws(draws: &[InitiativeDraw]) -> Vec<InitiativeDraw> {
    let mut ordered = draws.to_vec();
    ordered.sort_by(|left, right| {
        right
            .card
            .sort_key()
            .cmp(&left.card.sort_key())
            .then_with(|| left.display_name.cmp(&right.display_name))
    });
    ordered
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enemy_draw(name: &str, card: Card) -> InitiativeDraw {
        InitiativeDraw {
            kind: ParticipantKind::Enemy,
            participant_id: name.to_string(),
            display_name: name.to_string(),
            card,
            on_hold: false,
        }
    }

    fn player_draw(user_id: u64, name: &str, card: Card) -> InitiativeDraw {
        InitiativeDraw {
            kind: ParticipantKind::Player,
            participant_id: user_id.to_string(),
            display_name: name.to_string(),
            card,
            on_hold: false,
        }
    }

    #[test]
    fn parses_enemy_names_with_semicolons() {
        let parsed = parse_enemy_names("Goblin 1; Goblin 2 ; Troll;  Imp brutto ").unwrap();
        assert_eq!(parsed, vec!["Goblin 1", "Goblin 2", "Troll", "Imp brutto"]);
    }

    #[test]
    fn rejects_empty_enemy_name_list() {
        let err = parse_enemy_names(" ; ; ").unwrap_err();
        assert!(err.to_string().contains("almeno un nome valido"));
    }

    #[test]
    fn prevents_duplicate_player_draws() {
        let mut session = InitiativeSession::from_parts(
            1,
            vec![Card::Standard {
                rank: Rank::Ace,
                suit: Suit::Spades,
            }],
            Vec::new(),
        );

        session.draw_player(42, "Marco").unwrap();
        let err = session.draw_player(42, "Marco").unwrap_err();
        assert!(err.to_string().contains("già pescato"));
    }

    #[test]
    fn prevents_duplicate_enemy_draws() {
        let mut session = InitiativeSession::from_parts(
            1,
            vec![
                Card::Standard {
                    rank: Rank::King,
                    suit: Suit::Spades,
                },
                Card::Standard {
                    rank: Rank::Queen,
                    suit: Suit::Spades,
                },
            ],
            vec![enemy_draw(
                "Goblin 1",
                Card::Standard {
                    rank: Rank::Two,
                    suit: Suit::Clubs,
                },
            )],
        );

        let result = session
            .draw_enemies("Goblin 1; Goblin 2; Goblin 2")
            .unwrap();
        assert_eq!(result.drawn.len(), 1);
        assert_eq!(result.drawn[0].display_name, "Goblin 2");
        assert_eq!(result.skipped_existing, vec!["Goblin 1"]);
        assert_eq!(result.skipped_duplicates, vec!["Goblin 2"]);
    }

    #[test]
    fn player_hold_requires_prior_draw() {
        let mut session = InitiativeSession::from_parts(1, Vec::new(), Vec::new());
        let err = session.hold_player(7).unwrap_err();
        assert!(err.to_string().contains("Hold"));
    }

    #[test]
    fn enemy_hold_requires_prior_draw_and_reports_missing_names() {
        let mut session = InitiativeSession::from_parts(
            1,
            Vec::new(),
            vec![enemy_draw(
                "Goblin 1",
                Card::Standard {
                    rank: Rank::Five,
                    suit: Suit::Hearts,
                },
            )],
        );

        let result = session.hold_enemies("Goblin 1; Trol; Trol").unwrap();
        assert_eq!(result.held.len(), 1);
        assert!(result.held[0].on_hold);
        assert_eq!(result.missing, vec!["Trol"]);
        assert_eq!(result.duplicate_names, vec!["Trol"]);
    }

    #[test]
    fn sorts_initiative_by_rank_and_suit() {
        let draws = vec![
            enemy_draw(
                "Clubs",
                Card::Standard {
                    rank: Rank::Ace,
                    suit: Suit::Clubs,
                },
            ),
            enemy_draw(
                "Hearts",
                Card::Standard {
                    rank: Rank::Ace,
                    suit: Suit::Hearts,
                },
            ),
            enemy_draw(
                "King",
                Card::Standard {
                    rank: Rank::King,
                    suit: Suit::Spades,
                },
            ),
        ];

        let ordered = sort_draws(&draws);
        assert_eq!(ordered[0].display_name, "Hearts");
        assert_eq!(ordered[1].display_name, "Clubs");
        assert_eq!(ordered[2].display_name, "King");
    }

    #[test]
    fn sorts_joker_above_all_other_cards() {
        let draws = vec![
            enemy_draw(
                "Ace",
                Card::Standard {
                    rank: Rank::Ace,
                    suit: Suit::Spades,
                },
            ),
            enemy_draw("Joker", Card::Joker(JokerColor::Red)),
        ];

        let ordered = sort_draws(&draws);
        assert_eq!(ordered[0].display_name, "Joker");
    }

    #[test]
    fn player_joker_awards_only_current_round_players() {
        let mut session = InitiativeSession::from_parts(
            3,
            vec![Card::Standard {
                rank: Rank::Two,
                suit: Suit::Clubs,
            }],
            vec![
                player_draw(1, "Marco", Card::Joker(JokerColor::Red)),
                player_draw(
                    2,
                    "Sara",
                    Card::Standard {
                        rank: Rank::Ace,
                        suit: Suit::Spades,
                    },
                ),
                enemy_draw(
                    "Troll",
                    Card::Standard {
                        rank: Rank::King,
                        suit: Suit::Hearts,
                    },
                ),
            ],
        );

        let resolution = session.resolve_round().unwrap();
        assert!(resolution.player_joker);
        assert!(resolution.any_joker);
        assert_eq!(resolution.benny_recipients.len(), 2);
        assert_eq!(resolution.benny_recipients[0].display_name, "Marco");
        assert_eq!(resolution.benny_recipients[1].display_name, "Sara");
    }

    #[test]
    fn enemy_only_joker_reshuffles_without_bennies() {
        let original_deck = vec![Card::Standard {
            rank: Rank::Two,
            suit: Suit::Clubs,
        }];
        let mut session = InitiativeSession::from_parts(
            1,
            original_deck,
            vec![enemy_draw("Troll", Card::Joker(JokerColor::Black))],
        );

        let resolution = session.resolve_round().unwrap();
        assert!(resolution.any_joker);
        assert!(!resolution.player_joker);
        assert!(resolution.benny_recipients.is_empty());
        assert_eq!(session.deck.len(), 54);
    }

    #[test]
    fn no_joker_keeps_cards_discarded() {
        let mut session = InitiativeSession::from_parts(
            1,
            vec![
                Card::Standard {
                    rank: Rank::Three,
                    suit: Suit::Clubs,
                },
                Card::Standard {
                    rank: Rank::Four,
                    suit: Suit::Clubs,
                },
            ],
            vec![player_draw(
                99,
                "Marco",
                Card::Standard {
                    rank: Rank::Ace,
                    suit: Suit::Spades,
                },
            )],
        );

        let resolution = session.resolve_round().unwrap();
        assert!(!resolution.any_joker);
        assert_eq!(session.deck.len(), 2);
        assert_eq!(session.round, 2);
        assert!(session.draws.is_empty());
    }

    #[test]
    fn serializes_and_deserializes_deck() {
        let deck = vec![
            Card::Standard {
                rank: Rank::Ace,
                suit: Suit::Spades,
            },
            Card::Joker(JokerColor::Red),
        ];
        let serialized = serialize_deck(&deck);
        let restored = deserialize_deck(&serialized).unwrap();
        assert_eq!(deck, restored);
    }
}
