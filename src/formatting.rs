use crate::dice::{DamageRoll, TraitRoll};
use crate::initiative::{EnemyDrawResult, EnemyHoldResult, InitiativeDraw, RoundResolution};

pub fn signed(value: i32) -> String {
    if value >= 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

pub fn format_trait_roll(actor: &str, roll_name: &str, roll: &TraitRoll) -> String {
    let mut lines = vec![format!("🎲 **{actor} tira {roll_name}**"), String::new()];

    if roll.trait_die.is_unskilled() {
        lines.push(format!(
            "**Trait Die {} (Unskilled):** {}",
            roll.effective_trait_die().label(),
            roll.trait_roll.notation()
        ));
    } else {
        lines.push(format!(
            "**Trait Die {}:** {}",
            roll.effective_trait_die().label(),
            roll.trait_roll.notation()
        ));
    }

    if let Some(wild) = &roll.wild_roll {
        lines.push(format!("**Wild Die d6:** {}", wild.notation()));
        lines.push(format!("**Best Die:** {}", roll.best_raw()));
    }

    lines.push(format!("**Mod:** {}", signed(roll.modifier)));
    if roll.trait_die.is_unskilled() {
        lines.push("**Unskilled:** -2".to_string());
    }
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
    lines.push(format!(
        "**Dadi:** {} + {} → {roll_parts}",
        roll.dice.attr_die.label(),
        roll.dice.weapon_die.label()
    ));
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

pub fn format_initiative_card(draw: &InitiativeDraw) -> String {
    draw.card.label()
}

pub fn format_initiative_hold(draw: &InitiativeDraw) -> String {
    format!(
        "✋ **{} va in Hold**\nCarta attuale: **{}**",
        draw.display_name,
        format_initiative_card(draw)
    )
}

pub fn format_player_initiative_draw(draw: &InitiativeDraw, round: u32) -> String {
    format!(
        "🃏 **Carta pescata**\n**{}:** {}\nRound corrente: **{}**",
        draw.display_name,
        format_initiative_card(draw),
        round
    )
}

pub fn format_initiative_order(resolution: &RoundResolution) -> String {
    let mut lines = vec![format!(
        "🃏 **Ordine di iniziativa - Round {}**",
        resolution.round
    )];
    lines.push(String::new());

    for (index, draw) in resolution.ordered_draws.iter().enumerate() {
        let hold_suffix = if draw.on_hold { " _(Hold)_" } else { "" };
        lines.push(format!(
            "{}. **{}** - {}{}",
            index + 1,
            draw.display_name,
            format_initiative_card(draw),
            hold_suffix
        ));
    }

    lines.join("\n")
}

pub fn format_enemy_draw_result(result: &EnemyDrawResult, round: u32) -> String {
    let mut lines = vec![format!(
        "👹 **Carte nemici pescate**\nRound corrente: **{}**",
        round
    )];
    lines.push(String::new());

    if result.drawn.is_empty() {
        lines.push("Nessun nemico valido ha pescato una carta.".to_string());
    } else {
        for draw in &result.drawn {
            lines.push(format!(
                "- **{}**: {}",
                draw.display_name,
                format_initiative_card(draw)
            ));
        }
    }

    if !result.skipped_existing.is_empty() {
        lines.push(String::new());
        lines.push(format!(
            "Già presenti in questo round: {}.",
            result.skipped_existing.join(", ")
        ));
    }

    if !result.skipped_duplicates.is_empty() {
        lines.push(format!(
            "Duplicati nello stesso comando: {}.",
            result.skipped_duplicates.join(", ")
        ));
    }

    lines.join("\n")
}

pub fn format_enemy_hold_result(result: &EnemyHoldResult) -> String {
    let mut lines = vec!["✋ **Nemici in Hold aggiornati**".to_string()];

    if !result.held.is_empty() {
        lines.push(String::new());
        lines.push("Messi in Hold:".to_string());
        for draw in &result.held {
            lines.push(format!("- **{}**", draw.display_name));
        }
    }

    if !result.missing.is_empty() {
        lines.push(String::new());
        lines.push(format!(
            "Non trovati nel round corrente, possibile refuso: {}.",
            result.missing.join(", ")
        ));
    }

    if !result.duplicate_names.is_empty() {
        lines.push(format!(
            "Nomi duplicati nello stesso comando: {}.",
            result.duplicate_names.join(", ")
        ));
    }

    lines.join("\n")
}

pub fn format_round_resolution(
    resolution: &RoundResolution,
    awarded_bennies: &[(String, i64)],
) -> String {
    let mut lines = vec![format_initiative_order(resolution), String::new()];

    if resolution.any_joker {
        if resolution.player_joker {
            lines.push(
                "🃏 È uscito almeno un Joker pescato da un player: il mazzo verrà rimescolato per il prossimo round."
                    .to_string(),
            );

            if awarded_bennies.is_empty() {
                lines.push("Nessun Benny assegnato.".to_string());
            } else {
                lines.push("🎟️ Bennies assegnati ai partecipanti del round:".to_string());
                for (name, total) in awarded_bennies {
                    lines.push(format!("- **{}** ora è a **{}** Bennies", name, total));
                }
            }
        } else {
            lines.push(
                "🃏 È uscito almeno un Joker, ma nessun player lo ha pescato: il mazzo verrà rimescolato e la party non riceve Bennies."
                    .to_string(),
            );
        }
    } else {
        lines
            .push("🂠 Nessun Joker in questo round: le carte pescate restano scartate.".to_string());
    }

    lines.push(format!(
        "➡️ Il prossimo round è **{}**. Si può tornare a pescare.",
        resolution.next_round
    ));

    lines.join("\n")
}
