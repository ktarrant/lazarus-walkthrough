use crate::encounters::SpeciesEncounter;
use crate::pokedex::{Abilities, LevelMove, PokemonEntry, Stats};
use crate::type_chart;
use std::collections::HashMap;
use std::fmt::Write;

pub fn render_deck(
    chain: &[&PokemonEntry],
    active_slug: &str,
    encounters_map: &HashMap<String, Vec<SpeciesEncounter>>,
) -> String {
    if chain.is_empty() {
        return String::new();
    }
    let summary_label = summary_label(
        chain
            .iter()
            .find(|e| e.slug == active_slug)
            .copied()
            .unwrap_or(chain[0]),
    );

    if chain.len() == 1 {
        let entry = chain[0];
        let encounters = encounters_map
            .get(&entry.slug)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        let mut output = String::new();
        output.push_str("<details class=\"pokemon-card-container\">\n");
        writeln!(&mut output, "<summary>{}</summary>", summary_label).unwrap();
        output.push_str(&render_single(entry, encounters));
        output.push_str("</details>\n");
        return output;
    }

    let active_index = chain
        .iter()
        .position(|e| e.slug == active_slug)
        .unwrap_or(0);
    let group_name = format!("pokemon-tabs-{}", active_slug);
    let radio_name = format!("{}-group", group_name);
    let mut output = String::new();
    output.push_str("<details class=\"pokemon-card-container\">\n");
    writeln!(&mut output, "<summary>{}</summary>", summary_label).unwrap();
    writeln!(
        &mut output,
        "<div class=\"pokemon-tabs\" id=\"{}\">",
        group_name
    )
    .unwrap();

    for (idx, entry) in chain.iter().enumerate() {
        let tab_id = format!("{}-tab-{}", group_name, idx);
        let checked = if idx == active_index { " checked" } else { "" };
        writeln!(
            &mut output,
            "<input type=\"radio\" name=\"{}\" id=\"{}\"{}>",
            radio_name, tab_id, checked
        )
        .unwrap();
        writeln!(
            &mut output,
            "<label for=\"{}\">{}</label>",
            tab_id, entry.name
        )
        .unwrap();
    }

    output.push_str("<div class=\"pokemon-tab-panels\">\n");
    for (idx, entry) in chain.iter().enumerate() {
        let panel_id = format!("{}-panel-{}", group_name, idx);
        writeln!(
            &mut output,
            "<div class=\"pokemon-tab-panel\" id=\"{}\">",
            panel_id
        )
        .unwrap();
        let encounters = encounters_map
            .get(&entry.slug)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        output.push_str(&render_single(entry, encounters));
        output.push_str("</div>\n");
    }
    output.push_str("</div>\n</div>\n<style>\n");
    for idx in 0..chain.len() {
        let tab_id = format!("{}-tab-{}", group_name, idx);
        let panel_id = format!("{}-panel-{}", group_name, idx);
        writeln!(
            &mut output,
            "#{}:checked ~ .pokemon-tab-panels #{} {{ display: block; }}",
            tab_id, panel_id
        )
        .unwrap();
    }
    output.push_str("</style>\n</details>\n");
    output
}

fn render_single(entry: &PokemonEntry, encounters: &[SpeciesEncounter]) -> String {
    let mut buf = String::new();

    let mut overview_parts = Vec::new();
    if !entry.types.is_empty() {
        overview_parts.push(format!("Types: {}", entry.types.join(" / ")));
    }
    if !entry.egg_groups.is_empty() {
        overview_parts.push(format!("Egg Groups: {}", entry.egg_groups.join(" / ")));
    }
    if !overview_parts.is_empty() {
        writeln!(&mut buf, "{}", overview_parts.join(" • ")).unwrap();
    }

    let mut left_column = String::new();
    let mut right_column = String::new();

    append_abilities(&mut left_column, &entry.abilities);
    append_type_matchups(&mut left_column, &entry.types);
    append_move_list(&mut left_column, "TM/HM Moves", &entry.tm_moves);
    if let Some(item) = non_empty(&entry.held_item) {
        writeln!(&mut left_column, "**Held Item**\n{}\n", item).unwrap();
    }
    if let Some(evolution) = non_empty(&entry.evolution) {
        writeln!(&mut left_column, "**Evolution Info**\n{}\n", evolution).unwrap();
    }

    append_encounters_section(&mut left_column, encounters);

    // Caught checkbox shared with encounter tables
    right_column.push_str(&format!(
        "<label><input type=\"checkbox\" class=\"caught-check\" data-species=\"{}\" /> Caught</label>\n\n",
        entry.slug
    ));
    append_stats_table(&mut right_column, &entry.stats);
    append_level_up_moves(&mut right_column, &entry.level_up_moves);
    append_move_list(&mut right_column, "Egg Moves", &entry.egg_moves);
    append_move_list(&mut right_column, "Tutor Moves", &entry.tutor_moves);

    buf.push_str("\n<div class=\"pokemon-card\">\n");
    buf.push_str("<div class=\"card-column\">\n");
    buf.push_str(left_column.trim());
    buf.push_str("\n</div>\n<div class=\"card-column\">\n");
    buf.push_str(right_column.trim());
    buf.push_str("\n</div>\n</div>\n");

    buf.push_str(
        "<script>\n(function() {\n  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;\n  const STORAGE_KEY = 'lazarusCaught';\n  function loadState() {\n    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }\n  }\n  function saveState(state) {\n    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}\n  }\n  function applyState() {\n    const state = loadState();\n    const boxes = Array.from(document.querySelectorAll('.caught-check'));\n    const bySpecies = boxes.reduce((m, cb) => {\n      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});\n    boxes.forEach(cb => {\n      const key = cb.dataset.species;\n      cb.checked = !!state[key];\n      cb.onchange = () => {\n        const checked = cb.checked;\n        if (checked) state[key] = true; else delete state[key];\n        saveState(state);\n        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });\n      };\n    });\n  }\n  if (document.readyState === 'loading') {\n    document.addEventListener('DOMContentLoaded', applyState);\n  } else {\n    applyState();\n  }\n})();\n</script>\n",
    );

    buf
}

fn summary_label(entry: &PokemonEntry) -> String {
    let dex_label = entry
        .dex
        .map_or(String::new(), |dex| format!(" (#{:03})", dex));
    format!("{}{}", entry.name, dex_label)
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
        let link = if move_name.is_empty() {
            String::new()
        } else {
            format!(
                "<a href=\"move-lookup.html?q={}\">{}</a>",
                move_slug(move_name),
                move_name
            )
        };
        if move_name.is_empty() {
            writeln!(buf, "- {}", level).unwrap();
        } else if level.is_empty() {
            writeln!(buf, "- {}", link).unwrap();
        } else {
            writeln!(buf, "- {} (Lv {})", link, level).unwrap();
        }
    }
    buf.push('\n');
}

fn append_move_list(buf: &mut String, heading: &str, moves: &[String]) {
    let entries: Vec<_> = moves
        .iter()
        .map(|m| m.trim())
        .filter(|m| !m.is_empty())
        .collect();
    if entries.is_empty() {
        return;
    }
    writeln!(buf, "**{}**", heading).unwrap();
    for mv in entries {
        writeln!(
            buf,
            "- <a href=\"move-lookup.html?q={}\">{}</a>",
            move_slug(mv),
            mv
        )
        .unwrap();
    }
    buf.push('\n');
}

fn move_slug(raw: &str) -> String {
    let normalized = normalize_move_name(raw);
    crate::encounters::slugify(&normalized)
}

fn normalize_move_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.starts_with("TM") || trimmed.starts_with("HM") {
        if let Some((_, rest)) = trimmed.split_once('-') {
            return rest.trim().to_string();
        }
        if let Some((_, rest)) = trimmed.split_once(' ') {
            return rest.trim().to_string();
        }
    }
    trimmed.to_string()
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

pub fn non_empty(text: &str) -> Option<&str> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
