use crate::encounters::SpeciesEncounter;
use crate::pokedex::{Abilities, LevelMove, PokemonEntry, Stats};
use crate::type_chart;

use std::fmt::Write;

pub fn render_card(entry: &PokemonEntry, encounters: &[SpeciesEncounter]) -> String {
    let mut buf = String::new();
    let dex_label = entry
        .dex
        .map_or(String::new(), |dex| format!(" (#{:03})", dex));
    writeln!(&mut buf, "## {}{}", entry.name, dex_label).unwrap();
    buf.push_str("<details class=\"pokemon-card-container\" open>\n");
    writeln!(&mut buf, "<summary>{} overview</summary>", entry.name).unwrap();

    let mut overview_parts = Vec::new();
    if !entry.types.is_empty() {
        overview_parts.push(format!("Types: {}", entry.types.join(" / ")));
    }
    if !entry.egg_groups.is_empty() {
        overview_parts.push(format!("Egg Groups: {}", entry.egg_groups.join(" / ")));
    }
    if let Some(location) = non_empty(&entry.location) {
        overview_parts.push(format!("Found: {}", location));
    }
    if !overview_parts.is_empty() {
        writeln!(&mut buf, "{}", overview_parts.join(" • ")).unwrap();
    }

    let mut left_column = String::new();
    let mut right_column = String::new();

    append_abilities(&mut left_column, &entry.abilities);
    append_type_matchups(&mut left_column, &entry.types);
    if let Some(held) = non_empty(&entry.held_item) {
        writeln!(
            &mut left_column,
            "**Notable Held Item**\n{}\n",
            held
        )
        .unwrap();
    }
    if let Some(evolution) = non_empty(&entry.evolution) {
        writeln!(&mut left_column, "**Evolution Info**\n{}\n", evolution).unwrap();
    }
    if let Some(item) = non_empty(&entry.held_item) {
        writeln!(&mut left_column, "**Held Item**\n{}\n", item).unwrap();
    }

    append_encounters_section(&mut left_column, encounters);

    append_stats_table(&mut right_column, &entry.stats);
    append_level_up_moves(&mut right_column, &entry.level_up_moves);
    append_optional_moves(&mut right_column, "Egg Moves", &entry.egg_moves);
    append_optional_moves(&mut right_column, "TM/HM Moves", &entry.tm_moves);
    append_optional_moves(&mut right_column, "Tutor Moves", &entry.tutor_moves);

    buf.push_str("\n<div class=\"pokemon-card\">\n");
    buf.push_str("<div class=\"card-column\">\n");
    buf.push_str(left_column.trim());
    buf.push_str("\n</div>\n<div class=\"card-column\">\n");
    buf.push_str(right_column.trim());
    buf.push_str("\n</div>\n</div>\n");

    buf.push_str("</details>\n");
    buf
}

fn append_abilities(buf: &mut String, abilities: &Abilities) {
    buf.push_str("**Abilities**\n");
    if let Some(primary) = non_empty(&abilities.primary) {
        writeln!(buf, "- {}", primary).unwrap();
    }
    if let Some(secondary) = non_empty(&abilities.secondary) {
        writeln!(buf, "- {}", secondary).unwrap();
    }
    if let Some(hidden) = non_empty(&abilities.hidden) {
        writeln!(buf, "- {} *(Hidden)*", hidden).unwrap();
    }
    buf.push('\n');
}

fn append_type_matchups(buf: &mut String, types: &[String]) {
    let type_elements: Vec<type_chart::Type> = types
        .iter()
        .filter_map(|name| type_chart::Type::from_api_name(&name.to_lowercase()))
        .collect();
    if type_elements.is_empty() {
        return;
    }
    let matchups = summarize_type_matchups(&type_elements);
    if !matchups.has_data() {
        return;
    }
    buf.push_str("**Type Matchups**\n");
    if !matchups.strong_against.is_empty() {
        buf.push_str("\n*Resists / Immune to*\n");
        for entry in &matchups.strong_against {
            writeln!(buf, "- {}", entry).unwrap();
        }
    }
    if !matchups.weak_against.is_empty() {
        buf.push_str("\n*Weak to*\n");
        for entry in &matchups.weak_against {
            writeln!(buf, "- {}", entry).unwrap();
        }
    }
    buf.push('\n');
}

fn append_encounters_section(buf: &mut String, encounters: &[SpeciesEncounter]) {
    if encounters.is_empty() {
        return;
    }
    buf.push_str("**Encounter Locations**\n");
    for enc in encounters {
        let rate = enc.rate.as_deref().unwrap_or("—");
        writeln!(buf, "- {} — {} ({})", enc.location, enc.method, rate).unwrap();
    }
    buf.push('\n');
}

fn append_stats_table(buf: &mut String, stats: &Stats) {
    let total = stats.hp + stats.atk + stats.def + stats.sp_atk + stats.sp_def + stats.speed;
    buf.push_str("**Base Stats**\n\n| Stat | Value |\n| --- | --- |\n");
    for (name, value) in [
        ("HP", stats.hp),
        ("Attack", stats.atk),
        ("Defense", stats.def),
        ("Sp. Atk", stats.sp_atk),
        ("Sp. Def", stats.sp_def),
        ("Speed", stats.speed),
    ] {
        let class = stat_class(value);
        writeln!(
            buf,
            "| {} | <span class=\"stat-value {}\">{}</span> |",
            name, class, value
        )
        .unwrap();
    }
    let avg = total / 6;
    writeln!(
        buf,
        "| Total | <span class=\"stat-value {}\">{}</span> |\n",
        stat_class(avg),
        total
    )
    .unwrap();
}

fn append_level_up_moves(buf: &mut String, moves: &[LevelMove]) {
    if moves.is_empty() {
        return;
    }
    buf.push_str("**Level-Up Moves**\n");
    for mv in moves {
        let move_name = mv.move_name.trim();
        let level = mv.level.trim();
        if move_name.is_empty() && level.is_empty() {
            continue;
        }
        if move_name.is_empty() {
            writeln!(buf, "- {}", level).unwrap();
        } else if level.is_empty() {
            writeln!(buf, "- {}", move_name).unwrap();
        } else {
            writeln!(buf, "- {} (Lv {})", move_name, level).unwrap();
        }
    }
    buf.push('\n');
}

fn append_optional_moves(buf: &mut String, heading: &str, moves: &[String]) {
    let filtered: Vec<_> = moves
        .iter()
        .map(|m| m.trim())
        .filter(|m| !m.is_empty())
        .collect();
    if filtered.is_empty() {
        return;
    }
    writeln!(buf, "**{}**\n{}\n", heading, filtered.join(", ")).unwrap();
}

#[derive(Debug, Clone, Default)]
struct TypeMatchups {
    strong_against: Vec<String>,
    weak_against: Vec<String>,
}

impl TypeMatchups {
    fn has_data(&self) -> bool {
        !(self.strong_against.is_empty() && self.weak_against.is_empty())
    }
}

fn summarize_type_matchups(types: &[type_chart::Type]) -> TypeMatchups {
    if types.is_empty() {
        return TypeMatchups::default();
    }

    let mut strong = Vec::new();
    let mut weak = Vec::new();
    for attack in type_chart::ORDERED_TYPES.iter() {
        let mut multiplier = 1.0;
        for pokemon_type in types {
            multiplier *= type_chart::multiplier(*attack, *pokemon_type);
        }
        if (multiplier - 1.0).abs() < f32::EPSILON {
            continue;
        }
        let label = format!(
            "{} ({}×)",
            attack.name(),
            type_chart::format_multiplier(multiplier)
        );
        if multiplier < 1.0 {
            strong.push(label);
        } else if multiplier > 1.0 {
            weak.push(label);
        }
    }

    TypeMatchups {
        strong_against: strong,
        weak_against: weak,
    }
}

fn stat_class(value: u32) -> &'static str {
    match value {
        0..=50 => "stat-low",
        51..=90 => "stat-mid",
        _ => "stat-high",
    }
}

fn non_empty(text: &str) -> Option<&str> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
