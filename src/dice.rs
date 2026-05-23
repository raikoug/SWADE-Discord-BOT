use anyhow::{anyhow, Result};
use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, poise::ChoiceParameter)]
pub enum Die {
    #[name = "d4"]
    D4,
    #[name = "d6"]
    D6,
    #[name = "d8"]
    D8,
    #[name = "d10"]
    D10,
    #[name = "d12"]
    D12,
}

impl Die {
    pub fn sides(self) -> u32 {
        match self {
            Self::D4 => 4,
            Self::D6 => 6,
            Self::D8 => 8,
            Self::D10 => 10,
            Self::D12 => 12,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::D4 => "d4",
            Self::D6 => "d6",
            Self::D8 => "d8",
            Self::D10 => "d10",
            Self::D12 => "d12",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplodingRoll {
    pub sides: u32,
    pub rolls: Vec<u32>,
}

impl ExplodingRoll {
    pub fn total(&self) -> i32 {
        self.rolls.iter().map(|value| *value as i32).sum()
    }

    pub fn first(&self) -> u32 {
        self.rolls.first().copied().unwrap_or_default()
    }

    pub fn notation(&self) -> String {
        if self.rolls.len() == 1 {
            self.total().to_string()
        } else {
            let parts = self
                .rolls
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("+");
            format!("{} [{}]", self.total(), parts)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitRoll {
    pub trait_die: Die,
    pub trait_roll: ExplodingRoll,
    pub wild_roll: Option<ExplodingRoll>,
    pub modifier: i32,
    pub tn: i32,
}

impl TraitRoll {
    pub fn best_raw(&self) -> i32 {
        let trait_total = self.trait_roll.total();
        match &self.wild_roll {
            Some(wild) => trait_total.max(wild.total()),
            None => trait_total,
        }
    }

    pub fn final_total(&self) -> i32 {
        self.best_raw() + self.modifier
    }

    pub fn is_success(&self) -> bool {
        self.final_total() >= self.tn
    }

    pub fn raises(&self) -> i32 {
        if !self.is_success() {
            return 0;
        }
        (self.final_total() - self.tn) / 4
    }

    pub fn is_critical_failure(&self) -> bool {
        match &self.wild_roll {
            Some(wild) => self.trait_roll.first() == 1 && wild.first() == 1,
            None => self.trait_roll.first() == 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamageNotation {
    pub count: u32,
    pub die: Die,
}

impl fmt::Display for DamageNotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.count, self.die.label())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamageRoll {
    pub notation: DamageNotation,
    pub rolls: Vec<ExplodingRoll>,
    pub modifier: i32,
    pub toughness: i32,
    pub armor_piercing: i32,
}

impl DamageRoll {
    pub fn dice_total(&self) -> i32 {
        self.rolls.iter().map(ExplodingRoll::total).sum()
    }

    pub fn total(&self) -> i32 {
        self.dice_total() + self.modifier
    }

    pub fn effective_toughness(&self) -> i32 {
        (self.toughness - self.armor_piercing).max(0)
    }

    pub fn wounds(&self) -> i32 {
        let diff = self.total() - self.effective_toughness();
        if diff < 4 {
            return 0;
        }
        diff / 4
    }

    pub fn causes_shaken(&self) -> bool {
        self.total() >= self.effective_toughness()
    }
}

pub fn roll_trait(die: Die, wild: bool, modifier: i32, tn: i32) -> TraitRoll {
    let mut rng = rand::thread_rng();
    let trait_roll = roll_exploding_with_rng(die.sides(), &mut rng);
    let wild_roll = wild.then(|| roll_exploding_with_rng(6, &mut rng));

    TraitRoll {
        trait_die: die,
        trait_roll,
        wild_roll,
        modifier,
        tn,
    }
}

pub fn parse_damage_notation(input: &str) -> Result<DamageNotation> {
    let normalized = input.trim().to_ascii_lowercase();
    let Some((count_part, die_part)) = normalized.split_once('d') else {
        return Err(anyhow!("usa una notazione tipo 2d6, 3d8 o d12"));
    };

    let count = if count_part.is_empty() {
        1
    } else {
        count_part
            .parse::<u32>()
            .map_err(|_| anyhow!("numero di dadi non valido: {count_part}"))?
    };

    if !(1..=20).contains(&count) {
        return Err(anyhow!("il numero di dadi deve essere tra 1 e 20"));
    }

    let sides = die_part
        .parse::<u32>()
        .map_err(|_| anyhow!("dado non valido: d{die_part}"))?;

    let die = match sides {
        4 => Die::D4,
        6 => Die::D6,
        8 => Die::D8,
        10 => Die::D10,
        12 => Die::D12,
        _ => {
            return Err(anyhow!(
                "dado non supportato: d{sides}; usa d4, d6, d8, d10 o d12"
            ))
        }
    };

    Ok(DamageNotation { count, die })
}

pub fn roll_damage(
    notation: DamageNotation,
    modifier: i32,
    toughness: i32,
    armor_piercing: i32,
) -> DamageRoll {
    let mut rng = rand::thread_rng();
    let rolls = (0..notation.count)
        .map(|_| roll_exploding_with_rng(notation.die.sides(), &mut rng))
        .collect();

    DamageRoll {
        notation,
        rolls,
        modifier,
        toughness,
        armor_piercing,
    }
}

fn roll_exploding_with_rng<R: Rng + ?Sized>(sides: u32, rng: &mut R) -> ExplodingRoll {
    let mut rolls = Vec::new();

    loop {
        let value = rng.gen_range(1..=sides);
        rolls.push(value);
        if value != sides {
            break;
        }
    }

    ExplodingRoll { sides, rolls }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_damage_notation() {
        let parsed = parse_damage_notation("2d6").unwrap();
        assert_eq!(parsed.count, 2);
        assert_eq!(parsed.die, Die::D6);

        let parsed = parse_damage_notation("d12").unwrap();
        assert_eq!(parsed.count, 1);
        assert_eq!(parsed.die, Die::D12);
    }

    #[test]
    fn trait_roll_calculates_raises() {
        let roll = TraitRoll {
            trait_die: Die::D8,
            trait_roll: ExplodingRoll {
                sides: 8,
                rolls: vec![8, 3],
            },
            wild_roll: Some(ExplodingRoll {
                sides: 6,
                rolls: vec![4],
            }),
            modifier: -2,
            tn: 4,
        };

        assert_eq!(roll.best_raw(), 11);
        assert_eq!(roll.final_total(), 9);
        assert_eq!(roll.raises(), 1);
    }

    #[test]
    fn critical_failure_requires_double_one_for_wild_cards() {
        let roll = TraitRoll {
            trait_die: Die::D8,
            trait_roll: ExplodingRoll {
                sides: 8,
                rolls: vec![1],
            },
            wild_roll: Some(ExplodingRoll {
                sides: 6,
                rolls: vec![1],
            }),
            modifier: 4,
            tn: 4,
        };

        assert!(roll.is_critical_failure());
    }

    #[test]
    fn damage_calculates_wounds() {
        let damage = DamageRoll {
            notation: DamageNotation {
                count: 2,
                die: Die::D6,
            },
            rolls: vec![
                ExplodingRoll {
                    sides: 6,
                    rolls: vec![6, 3],
                },
                ExplodingRoll {
                    sides: 6,
                    rolls: vec![4],
                },
            ],
            modifier: 0,
            toughness: 7,
            armor_piercing: 0,
        };

        assert_eq!(damage.total(), 13);
        assert!(damage.causes_shaken());
        assert_eq!(damage.wounds(), 1);
    }
}
