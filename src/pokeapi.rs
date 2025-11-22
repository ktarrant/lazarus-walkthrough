use crate::encounters::SpeciesEncounter;
use crate::type_chart;
use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn build_card_deck(&self, identifier: &str) -> Result<PokemonCardDeck> {
        let slug = slugify(identifier);
        let (base_slug, form) = split_form_slug(&slug);
        let species = self.load_species(&base_slug)?;
        let suffix = form.as_ref().map(|f| f.suffix.as_str());
        let pokemon = self.load_pokemon_for_species(&species, suffix)?;
        if let Some(chain) = self.load_evolution_chain(&species)? {
            let evolution_paths = render_paths(&chain);
            let mut cards = Vec::new();
            let mut active_idx = 0;
            for (idx, species_id) in collect_chain_species(&chain).iter().enumerate() {
                let species_data = self.read_species_file(*species_id)?;
                let pokemon_data = self.load_pokemon_for_species(&species_data, suffix)?;
                if species_data.id == species.id {
                    active_idx = idx;
                }
                let card =
                    PokemonCard::from_records(pokemon_data, species_data, evolution_paths.clone())?;
                cards.push(card);
            }
            Ok(PokemonCardDeck {
                cards,
                active_index: active_idx,
            })
        } else {
            let card = PokemonCard::from_records(pokemon, species, Vec::new())?;
            Ok(PokemonCardDeck {
                cards: vec![card],
                active_index: 0,
            })
        }
    }

    fn load_species(&self, identifier: &str) -> Result<PokemonSpecies> {
        if let Ok(id) = identifier.trim().parse::<u32>() {
            return self.read_species_file(id);
        }

        let slug = slugify(identifier);
        let species_dir = self.root.join("pokemon-species");
        let entries = std::fs::read_dir(&species_dir)
            .with_context(|| format!("reading species data in {}", species_dir.display()))?;

        for entry in entries {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }

            let path = entry.path().join("index.json");
            if !path.exists() {
                continue;
            }

            let species: PokemonSpecies = read_json(&path)
                .with_context(|| format!("reading species data from {}", path.display()))?;
            if species.name == slug {
                return Ok(species);
            }
        }

        bail!("No Pokémon species named {} found in cache", identifier);
    }

    fn load_pokemon(&self, id: u32) -> Result<Pokemon> {
        let path = self
            .root
            .join("pokemon")
            .join(id.to_string())
            .join("index.json");
        read_json(&path).with_context(|| format!("reading Pokémon data from {}", path.display()))
    }

    fn load_pokemon_for_species(
        &self,
        species: &PokemonSpecies,
        suffix: Option<&str>,
    ) -> Result<Pokemon> {
        if let Some(suffix) = suffix {
            let target = format!("{}{}", species.name, suffix);
            if let Some(variety) = species.varieties.iter().find(|v| v.pokemon.name == target) {
                let id = extract_id_from_url(&variety.pokemon.url)?;
                return self.load_pokemon(id);
            }
        }
        if let Some(default) = species.varieties.iter().find(|v| v.is_default) {
            let id = extract_id_from_url(&default.pokemon.url)?;
            return self.load_pokemon(id);
        }
        self.load_pokemon(species.id)
    }

    fn read_species_file(&self, id: u32) -> Result<PokemonSpecies> {
        let path = self
            .root
            .join("pokemon-species")
            .join(id.to_string())
            .join("index.json");
        read_json(&path).with_context(|| format!("reading species data from {}", path.display()))
    }

    fn load_evolution_chain(&self, species: &PokemonSpecies) -> Result<Option<EvolutionChain>> {
        let Some(url) = species
            .evolution_chain
            .as_ref()
            .map(|link| link.url.as_str())
        else {
            return Ok(None);
        };
        let id = extract_id_from_url(url)?;
        let path = self
            .root
            .join("evolution-chain")
            .join(id.to_string())
            .join("index.json");

        let chain: EvolutionChain = read_json(&path)
            .with_context(|| format!("reading evolution chain {}", path.display()))?;
        Ok(Some(chain))
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

fn extract_id_from_url(url: &str) -> Result<u32> {
    url.trim_matches('/')
        .rsplit('/')
        .next()
        .ok_or_else(|| anyhow!("unable to parse id from url {}", url))?
        .parse::<u32>()
        .map_err(|_| anyhow!("unable to parse id from url {}", url))
}

fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let trimmed = input.trim();
    for ch in trimmed.chars() {
        match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' => slug.push(ch.to_ascii_lowercase()),
            ' ' | '_' | '-' => {
                if !slug.ends_with('-') {
                    slug.push('-');
                }
            }
            '♀' | '⚥' => slug.push_str("-f"),
            '♂' => slug.push_str("-m"),
            '\'' | '’' | '.' | ',' => {}
            _ => slug.push(ch.to_ascii_lowercase()),
        }
    }
    slug.trim_matches('-').to_string()
}

fn encounter_slug_from_name(name: &str) -> String {
    let slug = slugify(name);
    const SUFFIX_TO_PREFIX: [(&str, &str); 4] = [
        ("-alola", "alolan"),
        ("-hisui", "hisuian"),
        ("-galar", "galarian"),
        ("-paldea", "paldean"),
    ];
    for (suffix, prefix) in SUFFIX_TO_PREFIX {
        if slug.ends_with(suffix) {
            let base = slug.trim_end_matches(suffix).trim_matches('-');
            if base.is_empty() {
                return slug;
            }
            return format!("{}-{}", prefix, base);
        }
    }
    slug
}

struct FormSpec {
    base: String,
    suffix: String,
}

fn split_form_slug(slug: &str) -> (String, Option<FormSpec>) {
    if let Some(spec) = canonical_form_spec(slug) {
        (spec.base.clone(), Some(spec))
    } else {
        (slug.to_string(), None)
    }
}

fn canonical_form_spec(slug: &str) -> Option<FormSpec> {
    const FORM_PREFIXES: [(&str, &str); 8] = [
        ("alolan-", "alola"),
        ("hisuian-", "hisui"),
        ("galarian-", "galar"),
        ("paldean-", "paldea"),
        ("white-striped-", "white-striped"),
        ("blue-striped-", "blue-striped"),
        ("red-striped-", "red-striped"),
        ("black-striped-", "black-striped"),
    ];
    for (prefix, suffix) in FORM_PREFIXES.iter() {
        if slug.starts_with(prefix) {
            let base = slug
                .trim_start_matches(prefix)
                .trim_matches('-')
                .to_string();
            let suffix_str = format!("-{suffix}");
            return Some(FormSpec {
                base: base.clone(),
                suffix: suffix_str.clone(),
            });
        }
    }

    const FORM_SUFFIXES: [(&str, &str); 5] = [
        ("-blue-flower", "blue"),
        ("-orange-flower", "orange"),
        ("-red-flower", "red"),
        ("-white-flower", "white"),
        ("-yellow-flower", "yellow"),
    ];
    for (pattern, suffix) in FORM_SUFFIXES.iter() {
        if slug.ends_with(pattern) {
            let base = slug.trim_end_matches(pattern).trim_matches('-').to_string();
            let suffix_str = format!("-{suffix}");
            return Some(FormSpec {
                base: base.clone(),
                suffix: suffix_str.clone(),
            });
        }
    }

    match slug {
        "sligoo" => Some(FormSpec {
            base: "sliggoo".to_string(),
            suffix: String::new(),
        }),
        "hisuian-sligoo" => Some(FormSpec {
            base: "sliggoo".to_string(),
            suffix: "-hisui".to_string(),
        }),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct PokemonCard {
    id: u32,
    name: String,
    encounter_slug: String,
    genus: Option<String>,
    types: Vec<String>,
    egg_groups: Vec<String>,
    abilities: Vec<AbilityLine>,
    stats: Vec<StatLine>,
    stats_total: u32,
    base_experience: Option<u32>,
    flavor_text: Option<String>,
    evolution_paths: Vec<String>,
    moves: Vec<MoveLine>,
    type_matchups: TypeMatchups,
}

pub struct PokemonCardDeck {
    cards: Vec<PokemonCard>,
    active_index: usize,
}

impl PokemonCard {
    fn from_records(
        pokemon: Pokemon,
        species: PokemonSpecies,
        evolution_paths: Vec<String>,
    ) -> Result<Self> {
        let species_display = display_name(&species.name);
        let pokemon_display = display_name(&pokemon.name);
        let resolved_name = if species.name != pokemon.name && !pokemon_display.is_empty() {
            pokemon_display
        } else if !species_display.is_empty() {
            species_display
        } else if !pokemon_display.is_empty() {
            pokemon_display
        } else {
            species.name.clone()
        };
        let types = extract_types(&pokemon);
        let type_elements: Vec<type_chart::Type> = pokemon
            .types
            .iter()
            .filter_map(|slot| type_chart::Type::from_api_name(&slot.type_info.name))
            .collect();
        let type_matchups = summarize_type_matchups(&type_elements);
        let abilities = extract_abilities(&pokemon);
        let (stats, total) = extract_stats(&pokemon);
        let moves = extract_moves(&pokemon);
        let flavor_text = pick_flavor_text(&species);
        let genus = species
            .genera
            .iter()
            .find(|g| g.language.name == "en")
            .map(|g| g.genus.clone());
        let egg_groups = species
            .egg_groups
            .iter()
            .map(|group| display_name(&group.name))
            .collect();

        let encounter_slug = encounter_slug_from_name(&pokemon.name);

        Ok(Self {
            id: pokemon.id,
            name: resolved_name,
            encounter_slug,
            genus,
            types,
            egg_groups,
            abilities,
            stats,
            stats_total: total,
            base_experience: pokemon.base_experience,
            flavor_text,
            evolution_paths,
            moves,
            type_matchups,
        })
    }

    fn lookup_encounters<'a>(
        &'a self,
        map: &'a HashMap<String, Vec<SpeciesEncounter>>,
    ) -> &'a [SpeciesEncounter] {
        map.get(&self.encounter_slug)
            .map(|entries| entries.as_slice())
            .unwrap_or(&[])
    }

    pub fn render_markdown(&self, encounters: &[SpeciesEncounter]) -> String {
        let mut buf = String::new();
        writeln!(&mut buf, "## {} (#{:03})", self.name, self.id).unwrap();
        buf.push_str("<details class=\"pokemon-card-container\" open>\n");
        writeln!(&mut buf, "<summary>{} overview</summary>", self.name).unwrap();

        let mut overview_parts = Vec::new();
        if let Some(genus) = &self.genus {
            overview_parts.push(format!("_{}_", genus));
        }
        if !self.types.is_empty() {
            overview_parts.push(format!("Types: {}", self.types.join(" / ")));
        }
        if !self.egg_groups.is_empty() {
            overview_parts.push(format!("Egg Groups: {}", self.egg_groups.join(" / ")));
        }
        if let Some(exp) = self.base_experience {
            overview_parts.push(format!("Base EXP: {}", exp));
        }
        if !overview_parts.is_empty() {
            writeln!(&mut buf, "{}", overview_parts.join(" • ")).unwrap();
        }

        let mut left_column = String::new();
        let mut right_column = String::new();

        if !self.abilities.is_empty() {
            left_column.push_str("**Abilities**\n");
            for ability in &self.abilities {
                if ability.is_hidden {
                    writeln!(&mut left_column, "- {} *(Hidden)*", ability.name).unwrap();
                } else {
                    writeln!(&mut left_column, "- {}", ability.name).unwrap();
                }
            }
            left_column.push('\n');
        }

        if self.type_matchups.has_data() {
            left_column.push_str("**Type Matchups**\n");
            if !self.type_matchups.strong_against.is_empty() {
                left_column.push_str("\n*Resists / Immune to*\n");
                for entry in &self.type_matchups.strong_against {
                    writeln!(&mut left_column, "- {}", entry).unwrap();
                }
            }
            if !self.type_matchups.weak_against.is_empty() {
                left_column.push_str("\n*Weak to*\n");
                for entry in &self.type_matchups.weak_against {
                    writeln!(&mut left_column, "- {}", entry).unwrap();
                }
            }
            left_column.push('\n');
        }

        if !self.stats.is_empty() {
            right_column.push_str("**Base Stats**\n\n| Stat | Value |\n| --- | --- |\n");
            for stat in &self.stats {
                let class = stat_class(stat.value);
                writeln!(
                    &mut right_column,
                    "| {} | <span class=\"stat-value {}\">{}</span> |",
                    stat.name, class, stat.value
                )
                .unwrap();
            }
            let total_class = stat_class(self.stats_total / (self.stats.len() as u32));
            writeln!(
                &mut right_column,
                "| Total | <span class=\"stat-value {}\">{}</span> |\n",
                total_class, self.stats_total
            )
            .unwrap();
        }

        if !self.moves.is_empty() {
            right_column.push_str("**Notable Level-Up Moves**\n");
            for mv in &self.moves {
                writeln!(&mut right_column, "- {} (Lv {})", mv.name, mv.level).unwrap();
            }
            right_column.push('\n');
        }

        if !self.evolution_paths.is_empty() {
            left_column.push_str("**Evolution Paths**\n");
            for path in &self.evolution_paths {
                writeln!(&mut left_column, "- {}", path).unwrap();
            }
            left_column.push('\n');
        }

        if !encounters.is_empty() {
            left_column.push_str("**Encounter Locations**\n");
            for enc in encounters {
                let rate = enc.rate.as_deref().unwrap_or("—");
                writeln!(
                    &mut left_column,
                    "- {} — {} ({})",
                    enc.location, enc.method, rate
                )
                .unwrap();
            }
            left_column.push('\n');
        }

        buf.push_str("\n<div class=\"pokemon-card\">\n");
        buf.push_str("<div class=\"card-column\">\n");
        buf.push_str(left_column.trim());
        buf.push_str("\n</div>\n<div class=\"card-column\">\n");
        buf.push_str(right_column.trim());
        buf.push_str("\n</div>\n</div>\n");

        if let Some(text) = &self.flavor_text {
            buf.push_str("\n**Flavor Text**\n");
            writeln!(&mut buf, "> {}", text).unwrap();
        }

        buf.push_str("</details>\n");
        buf
    }
}

impl PokemonCardDeck {
    pub fn render_markdown_with_encounters(
        &self,
        encounter_map: &HashMap<String, Vec<SpeciesEncounter>>,
    ) -> String {
        if self.cards.is_empty() {
            return String::new();
        }
        if self.cards.len() == 1 {
            let entries = self.cards[0].lookup_encounters(encounter_map);
            return self.cards[0].render_markdown(entries);
        }

        let active_card = &self.cards[self.active_index];
        let slug = slugify(&active_card.name);
        let group_name = format!("pokemon-tabs-{}-{}", slug, active_card.id);
        let radio_name = format!("{}-group", group_name);
        let mut output = String::new();
        writeln!(
            &mut output,
            r#"<div class="pokemon-tabs" id="{group}">"#,
            group = group_name
        )
        .unwrap();

        for (idx, card) in self.cards.iter().enumerate() {
            let tab_id = format!("{}-tab-{}", group_name, idx);
            let checked = if idx == self.active_index {
                " checked"
            } else {
                ""
            };
            writeln!(
                &mut output,
                r#"<input type="radio" name="{name}" id="{tab}"{checked}>"#,
                name = radio_name,
                tab = tab_id,
                checked = checked
            )
            .unwrap();
            writeln!(
                &mut output,
                r#"<label for="{tab}">{name}</label>"#,
                tab = tab_id,
                name = card.name
            )
            .unwrap();
        }

        output.push_str("<div class=\"pokemon-tab-panels\">\n");
        for (idx, card) in self.cards.iter().enumerate() {
            let panel_id = format!("{}-panel-{}", group_name, idx);
            writeln!(
                &mut output,
                r#"<div class="pokemon-tab-panel" id="{panel}">"#,
                panel = panel_id
            )
            .unwrap();
            let entries = card.lookup_encounters(encounter_map);
            output.push_str(&card.render_markdown(entries));
            output.push_str("</div>\n");
        }
        output.push_str("</div>\n</div>\n<style>\n");
        for idx in 0..self.cards.len() {
            let tab_id = format!("{}-tab-{}", group_name, idx);
            let panel_id = format!("{}-panel-{}", group_name, idx);
            writeln!(
                &mut output,
                "#{tab}:checked ~ .pokemon-tab-panels #{panel} {{ display: block; }}",
                tab = tab_id,
                panel = panel_id
            )
            .unwrap();
        }
        output.push_str("</style>\n");
        output
    }
}

#[derive(Debug, Clone)]
struct AbilityLine {
    name: String,
    is_hidden: bool,
}

#[derive(Debug, Clone)]
struct StatLine {
    name: String,
    value: u32,
}

#[derive(Debug, Clone)]
struct MoveLine {
    name: String,
    level: u32,
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

fn stat_class(value: u32) -> &'static str {
    match value {
        0..=50 => "stat-low",
        51..=90 => "stat-mid",
        _ => "stat-high",
    }
}

fn display_name(slug: &str) -> String {
    slug.split('-')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_types(pokemon: &Pokemon) -> Vec<String> {
    let mut slots = pokemon.types.clone();
    slots.sort_by_key(|slot| slot.slot);
    slots
        .iter()
        .map(|slot| display_name(&slot.type_info.name))
        .collect()
}

fn extract_abilities(pokemon: &Pokemon) -> Vec<AbilityLine> {
    let mut abilities = pokemon.abilities.clone();
    abilities.sort_by_key(|a| a.slot);
    abilities
        .iter()
        .map(|ability| AbilityLine {
            name: display_name(&ability.ability.name),
            is_hidden: ability.is_hidden,
        })
        .collect()
}

fn extract_stats(pokemon: &Pokemon) -> (Vec<StatLine>, u32) {
    const ORDER: [&str; 6] = [
        "hp",
        "attack",
        "defense",
        "special-attack",
        "special-defense",
        "speed",
    ];
    let mut stats = pokemon.stats.clone();
    stats.sort_by_key(|stat| {
        ORDER
            .iter()
            .position(|name| name == &stat.stat.name)
            .unwrap_or(ORDER.len())
    });
    let lines: Vec<StatLine> = stats
        .iter()
        .map(|stat| StatLine {
            name: stat_display_name(&stat.stat.name),
            value: stat.base_stat,
        })
        .collect();
    let total = lines.iter().map(|stat| stat.value).sum();
    (lines, total)
}

fn stat_display_name(slug: &str) -> String {
    match slug {
        "hp" => "HP".to_string(),
        "attack" => "Attack".to_string(),
        "defense" => "Defense".to_string(),
        "special-attack" => "Sp. Atk".to_string(),
        "special-defense" => "Sp. Def".to_string(),
        "speed" => "Speed".to_string(),
        other => display_name(other),
    }
}

fn extract_moves(pokemon: &Pokemon) -> Vec<MoveLine> {
    use std::collections::BTreeSet;

    // Emerald learnset acts as our canonical reference for now.
    const TARGET_VERSION_GROUP: &str = "emerald";
    let mut entries: BTreeSet<(u32, String)> = BTreeSet::new();
    for mv in &pokemon.moves {
        let name = display_name(&mv.move_ref.name);
        for detail in &mv.version_group_details {
            if detail.move_learn_method.name != "level-up"
                || detail.level_learned_at == 0
                || detail.version_group.name != TARGET_VERSION_GROUP
            {
                continue;
            }
            entries.insert((detail.level_learned_at, name.clone()));
        }
    }

    entries
        .into_iter()
        .map(|(level, name)| MoveLine { name, level })
        .collect()
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

fn pick_flavor_text(species: &PokemonSpecies) -> Option<String> {
    const VERSION_PRIORITY: [&str; 12] = [
        "violet",
        "scarlet",
        "sword",
        "shield",
        "sun",
        "moon",
        "x",
        "y",
        "alpha-sapphire",
        "omega-ruby",
        "black",
        "white",
    ];

    species
        .flavor_text_entries
        .iter()
        .filter(|entry| entry.language.name == "en")
        .min_by_key(|entry| {
            VERSION_PRIORITY
                .iter()
                .position(|name| name == &entry.version.name)
                .unwrap_or(VERSION_PRIORITY.len())
        })
        .map(|entry| normalize_flavor_text(&entry.flavor_text))
}

fn normalize_flavor_text(text: &str) -> String {
    text.replace('\n', " ")
        .replace('\u{000C}', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn render_paths(chain: &EvolutionChain) -> Vec<String> {
    let mut paths = Vec::new();
    let root = vec![display_name(&chain.chain.species.name)];
    build_paths(&chain.chain, root, &mut paths);
    paths
}

fn collect_chain_species(chain: &EvolutionChain) -> Vec<u32> {
    fn visit(link: &ChainLink, out: &mut Vec<u32>) {
        if let Ok(id) = extract_id_from_url(&link.species.url) {
            out.push(id);
        }
        for child in &link.evolves_to {
            visit(child, out);
        }
    }

    let mut ids = Vec::new();
    visit(&chain.chain, &mut ids);
    ids
}

fn build_paths(link: &ChainLink, prefix: Vec<String>, output: &mut Vec<String>) {
    if link.evolves_to.is_empty() {
        output.push(prefix.join(" → "));
        return;
    }

    for evo in &link.evolves_to {
        let qualifier = describe_evolution(&evo.evolution_details);
        let mut branch = prefix.clone();
        if let Some(detail) = qualifier {
            let label = format!("{} ({})", display_name(&evo.species.name), detail);
            branch.push(label);
        } else {
            branch.push(display_name(&evo.species.name));
        }
        build_paths(evo, branch, output);
    }
}

fn describe_evolution(details: &[EvolutionDetail]) -> Option<String> {
    for detail in details {
        if let Some(level) = detail.min_level {
            return Some(format!("Lv {}", level));
        }
        if let Some(item) = &detail.item {
            return Some(format!("Use {}", display_name(&item.name)));
        }
        if let Some(item) = &detail.held_item {
            return Some(format!("Hold {}", display_name(&item.name)));
        }
        if let Some(move_name) = &detail.known_move {
            return Some(format!("Learn {}", display_name(&move_name.name)));
        }
        if let Some(move_type) = &detail.known_move_type {
            return Some(format!("{} move", display_name(&move_type.name)));
        }
        if let Some(location) = &detail.location {
            return Some(format!("Level at {}", display_name(&location.name)));
        }
        if let Some(happiness) = detail.min_happiness {
            return Some(format!("Friendship {}", happiness));
        }
        if let Some(affection) = detail.min_affection {
            return Some(format!("Affection {}", affection));
        }
        if let Some(beauty) = detail.min_beauty {
            return Some(format!("Beauty {}", beauty));
        }
        if let Some(trigger) = &detail.trigger {
            return Some(display_name(&trigger.name));
        }
    }
    None
}

#[derive(Debug, Deserialize, Clone)]
struct Pokemon {
    id: u32,
    name: String,
    base_experience: Option<u32>,
    abilities: Vec<PokemonAbility>,
    stats: Vec<PokemonStat>,
    types: Vec<PokemonType>,
    moves: Vec<PokemonMove>,
}

#[derive(Debug, Deserialize, Clone)]
struct PokemonAbility {
    slot: u8,
    is_hidden: bool,
    ability: NamedResource,
}

#[derive(Debug, Deserialize, Clone)]
struct PokemonType {
    slot: u8,
    #[serde(rename = "type")]
    type_info: NamedResource,
}

#[derive(Debug, Deserialize, Clone)]
struct PokemonStat {
    base_stat: u32,
    #[serde(rename = "stat")]
    stat: NamedResource,
}

#[derive(Debug, Deserialize, Clone)]
struct PokemonMove {
    #[serde(rename = "move")]
    move_ref: NamedResource,
    version_group_details: Vec<MoveVersionDetail>,
}

#[derive(Debug, Deserialize, Clone)]
struct MoveVersionDetail {
    level_learned_at: u32,
    #[serde(rename = "move_learn_method")]
    move_learn_method: NamedResource,
    #[serde(rename = "version_group")]
    version_group: NamedResource,
}

#[derive(Debug, Deserialize, Clone)]
struct PokemonSpecies {
    id: u32,
    name: String,
    genera: Vec<GenusEntry>,
    flavor_text_entries: Vec<FlavorTextEntry>,
    #[serde(default)]
    egg_groups: Vec<NamedResource>,
    #[serde(default)]
    evolution_chain: Option<ApiResource>,
    #[serde(default)]
    varieties: Vec<PokemonVariety>,
}

#[derive(Debug, Deserialize, Clone)]
struct PokemonVariety {
    is_default: bool,
    pokemon: NamedResource,
}

#[derive(Debug, Deserialize, Clone)]
struct GenusEntry {
    genus: String,
    language: NamedResource,
}

#[derive(Debug, Deserialize, Clone)]
struct FlavorTextEntry {
    flavor_text: String,
    language: NamedResource,
    version: NamedResource,
}

#[derive(Debug, Deserialize, Clone)]
struct ApiResource {
    url: String,
}

#[derive(Debug, Deserialize, Clone)]
struct EvolutionChain {
    chain: ChainLink,
}

#[derive(Debug, Deserialize, Clone)]
struct ChainLink {
    species: NamedResource,
    evolves_to: Vec<ChainLink>,
    #[serde(default)]
    evolution_details: Vec<EvolutionDetail>,
}

#[derive(Debug, Deserialize, Clone)]
struct EvolutionDetail {
    min_level: Option<u32>,
    #[serde(default)]
    item: Option<NamedResource>,
    #[serde(default)]
    trigger: Option<NamedResource>,
    #[serde(default)]
    held_item: Option<NamedResource>,
    #[serde(default)]
    known_move: Option<NamedResource>,
    #[serde(default)]
    known_move_type: Option<NamedResource>,
    #[serde(default)]
    location: Option<NamedResource>,
    #[serde(default)]
    min_affection: Option<u32>,
    #[serde(default)]
    min_beauty: Option<u32>,
    #[serde(default)]
    min_happiness: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
struct NamedResource {
    name: String,
    #[allow(dead_code)]
    url: String,
}
