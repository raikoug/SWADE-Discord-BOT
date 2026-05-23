use crate::dice::{DamageRoll, TraitRoll};

pub fn signed(value: i32) -> String {
    if value >= 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

pub fn format_trait_roll(actor: &str, roll_name: &str, roll: &TraitRoll) -> String {
    let mut lines = vec![format!("🎲 **{actor} tira {roll_name}**"), String::new()];

    lines.push(format!(
        "**Trait Die {}:** {}",
        roll.trait_die.label(),
        roll.trait_roll.notation()
    ));

    if let Some(wild) = &roll.wild_roll {
        lines.push(format!("**Wild Die d6:** {}", wild.notation()));
        lines.push(format!("**Best Die:** {}", roll.best_raw()));
    }

    lines.push(format!("**Mod:** {}", signed(roll.modifier)));
    lines.push(format!(
        "**Risultato finale:** {} vs **TN {}**",
        roll.final_total(),
        roll.tn
    ));
    lines.push(String::new());

    if roll.is_critical_failure() {
        lines.push("☠️ **Critical Failure**".to_string());
    } else if roll.is_success() {
        match roll.raises() {
            0 => lines.push("✅ **Successo**".to_string()),
            1 => lines.push("✅ **Successo con 1 Raise**".to_string()),
            raises => lines.push(format!("✅ **Successo con {raises} Raises**")),
        }
    } else {
        lines.push("❌ **Fallimento**".to_string());
    }

    lines.join("\n")
}

pub fn format_damage_roll(actor: &str, name: &str, roll: &DamageRoll) -> String {
    let roll_parts = roll
        .rolls
        .iter()
        .map(|single| single.notation())
        .collect::<Vec<_>>()
        .join(", ");

    let mut lines = vec![format!("💥 **{actor} tira danno: {name}**"), String::new()];
    lines.push(format!("**Dadi:** {} → {roll_parts}", roll.notation));
    lines.push(format!("**Totale dadi:** {}", roll.dice_total()));
    lines.push(format!("**Mod:** {}", signed(roll.modifier)));
    lines.push(format!("**AP:** {}", roll.armor_piercing));
    lines.push(format!("**Toughness:** {}", roll.toughness));
    lines.push(format!(
        "**Toughness effettiva:** {}",
        roll.effective_toughness()
    ));
    lines.push(format!("**Danno finale:** {}", roll.total()));
    lines.push(String::new());

    if !roll.causes_shaken() {
        lines.push("🛡️ **Nessun effetto**".to_string());
    } else {
        match roll.wounds() {
            0 => lines.push(
                "⚠️ **Shaken**\n_Se il bersaglio era già Shaken, il Master può convertirlo in 1 Wound._"
                    .to_string(),
            ),
            1 => lines.push("🩸 **Shaken + 1 Wound**".to_string()),
            wounds => lines.push(format!("🩸 **Shaken + {wounds} Wounds**")),
        }
    }

    lines.join("\n")
}
