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

#[derive(Debug, Clone, Copy, PartialEq, Eq, poise::ChoiceParameter)]
pub enum TraitDie {
    #[name = "d0"]
    D0,
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

impl TraitDie {
    pub fn effective_die(self) -> Die {
        match self {
            Self::D0 | Self::D4 => Die::D4,
            Self::D6 => Die::D6,
            Self::D8 => Die::D8,
            Self::D10 => Die::D10,
            Self::D12 => Die::D12,
        }
    }

    pub fn is_unskilled(self) -> bool {
        matches!(self, Self::D0)
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
    pub trait_die: TraitDie,
    pub trait_roll: ExplodingRoll,
    pub wild_roll: Option<ExplodingRoll>,
    pub modifier: i32,
    pub tn: i32,
}

impl TraitRoll {
    pub fn effective_trait_die(&self) -> Die {
        self.trait_die.effective_die()
    }

    pub fn unskilled_penalty(&self) -> i32 {
        if self.trait_die.is_unskilled() {
            -2
        } else {
            0
        }
    }

    pub fn best_raw(&self) -> i32 {
        let trait_total = self.trait_roll.total();
        match &self.wild_roll {
            Some(wild) => trait_total.max(wild.total()),
            None => trait_total,
        }
    }

    pub fn final_total(&self) -> i32 {
        self.best_raw() + self.modifier + self.unskilled_penalty()
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
            None => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageDice {
    pub attr_die: Die,
    pub weapon_die: Die,
}

impl fmt::Display for DamageDice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}", self.attr_die.label(), self.weapon_die.label())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamageRoll {
    pub dice: DamageDice,
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

pub fn roll_trait(die: TraitDie, wild: bool, modifier: i32, tn: i32) -> TraitRoll {
    let mut rng = rand::thread_rng();
    let trait_roll = roll_exploding_with_rng(die.effective_die().sides(), &mut rng);
    let wild_roll = wild.then(|| roll_exploding_with_rng(6, &mut rng));

    TraitRoll {
        trait_die: die,
        trait_roll,
        wild_roll,
        modifier,
        tn,
    }
}

pub fn roll_damage(
    dice: DamageDice,
    modifier: i32,
    toughness: i32,
    armor_piercing: i32,
) -> DamageRoll {
    let mut rng = rand::thread_rng();
    let rolls = vec![
        roll_exploding_with_rng(dice.attr_die.sides(), &mut rng),
        roll_exploding_with_rng(dice.weapon_die.sides(), &mut rng),
    ];

    DamageRoll {
        dice,
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
    fn damage_dice_display_lists_both_dice() {
        let dice = DamageDice {
            attr_die: Die::D8,
            weapon_die: Die::D6,
        };

        assert_eq!(dice.to_string(), "d8 + d6");
    }

    #[test]
    fn trait_roll_calculates_raises() {
        let roll = TraitRoll {
            trait_die: TraitDie::D8,
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
            trait_die: TraitDie::D8,
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
    fn unskilled_applies_minus_two_after_best_die() {
        let roll = TraitRoll {
            trait_die: TraitDie::D0,
            trait_roll: ExplodingRoll {
                sides: 4,
                rolls: vec![2],
            },
            wild_roll: Some(ExplodingRoll {
                sides: 6,
                rolls: vec![6],
            }),
            modifier: 0,
            tn: 4,
        };

        assert_eq!(roll.best_raw(), 6);
        assert_eq!(roll.unskilled_penalty(), -2);
        assert_eq!(roll.final_total(), 4);
        assert!(roll.is_success());
    }

    #[test]
    fn extras_do_not_critical_fail_on_single_one() {
        let roll = TraitRoll {
            trait_die: TraitDie::D6,
            trait_roll: ExplodingRoll {
                sides: 6,
                rolls: vec![1],
            },
            wild_roll: None,
            modifier: 0,
            tn: 4,
        };

        assert!(!roll.is_critical_failure());
    }

    #[test]
    fn damage_calculates_wounds() {
        let damage = DamageRoll {
            dice: DamageDice {
                attr_die: Die::D6,
                weapon_die: Die::D6,
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
