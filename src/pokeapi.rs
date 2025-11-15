use crate::type_chart;
use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;
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

    pub fn build_card(&self, identifier: &str) -> Result<PokemonCard> {
        let species = self.load_species(identifier)?;
        let pokemon = self.load_pokemon(species.id)?;
        let evolution_paths = self.load_evolution_paths(&species)?;
        PokemonCard::from_records(pokemon, species, evolution_paths)
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

    fn read_species_file(&self, id: u32) -> Result<PokemonSpecies> {
        let path = self
            .root
            .join("pokemon-species")
            .join(id.to_string())
            .join("index.json");
        read_json(&path).with_context(|| format!("reading species data from {}", path.display()))
    }

    fn load_evolution_paths(&self, species: &PokemonSpecies) -> Result<Vec<String>> {
        let Some(url) = species
            .evolution_chain
            .as_ref()
            .map(|link| link.url.as_str())
        else {
            return Ok(Vec::new());
        };
        let id = extract_id_from_url(url)?;
        let path = self
            .root
            .join("evolution-chain")
            .join(id.to_string())
            .join("index.json");

        let chain: EvolutionChain = read_json(&path)
            .with_context(|| format!("reading evolution chain {}", path.display()))?;
        Ok(render_paths(&chain))
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

#[derive(Debug, Clone)]
pub struct PokemonCard {
    id: u32,
    name: String,
    genus: Option<String>,
    types: Vec<String>,
    abilities: Vec<AbilityLine>,
    stats: Vec<StatLine>,
    stats_total: u32,
    height_m: f32,
    weight_kg: f32,
    base_experience: Option<u32>,
    flavor_text: Option<String>,
    evolution_paths: Vec<String>,
    moves: Vec<MoveLine>,
    type_matchups: TypeMatchups,
}

impl PokemonCard {
    fn from_records(
        pokemon: Pokemon,
        species: PokemonSpecies,
        evolution_paths: Vec<String>,
    ) -> Result<Self> {
        let display = display_name(&species.name);
        let fallback = display_name(&pokemon.name);
        let resolved_name = if display.is_empty() {
            fallback
        } else {
            display
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

        Ok(Self {
            id: pokemon.id,
            name: resolved_name,
            genus,
            types,
            abilities,
            stats,
            stats_total: total,
            height_m: pokemon.height as f32 / 10.0,
            weight_kg: pokemon.weight as f32 / 10.0,
            base_experience: pokemon.base_experience,
            flavor_text,
            evolution_paths,
            moves,
            type_matchups,
        })
    }

    pub fn render_markdown(&self) -> String {
        let mut buf = String::new();
        writeln!(&mut buf, "## {} (#{:03})", self.name, self.id).unwrap();
        if let Some(genus) = &self.genus {
            if !self.types.is_empty() {
                writeln!(&mut buf, "_{}_ • Types: {}", genus, self.types.join(" / ")).unwrap();
            } else {
                writeln!(&mut buf, "_{}_", genus).unwrap();
            }
        } else if !self.types.is_empty() {
            writeln!(&mut buf, "Types: {}", self.types.join(" / ")).unwrap();
        }

        let mut stat_line = format!(
            "Height: {:.1} m · Weight: {:.1} kg",
            self.height_m, self.weight_kg
        );
        if let Some(exp) = self.base_experience {
            stat_line.push_str(&format!(" · Base EXP: {}", exp));
        }
        writeln!(&mut buf, "{}", stat_line).unwrap();

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
                writeln!(&mut right_column, "| {} | {} |", stat.name, stat.value).unwrap();
            }
            writeln!(&mut right_column, "| Total | {} |\n", self.stats_total).unwrap();
        }

        if !self.moves.is_empty() {
            right_column.push_str("**Notable Level-Up Moves**\n");
            for mv in &self.moves {
                writeln!(
                    &mut right_column,
                    "- {} (Lv {}, {})",
                    mv.name, mv.level, mv.version
                )
                .unwrap();
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

        buf
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
    version: String,
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
    use std::collections::HashSet;

    let mut candidates: Vec<MoveLine> = Vec::new();
    for mv in &pokemon.moves {
        for detail in &mv.version_group_details {
            if detail.move_learn_method.name != "level-up" || detail.level_learned_at == 0 {
                continue;
            }
            candidates.push(MoveLine {
                name: display_name(&mv.move_ref.name),
                level: detail.level_learned_at,
                version: display_name(&detail.version_group.name),
            });
        }
    }
    if candidates.is_empty() {
        return candidates;
    }

    candidates.sort_by(|a, b| a.level.cmp(&b.level).then(a.name.cmp(&b.name)));
    let mut seen = HashSet::new();
    candidates.retain(|entry| seen.insert(entry.name.clone()));
    candidates.truncate(6);
    candidates
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
    height: u32,
    weight: u32,
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
    evolution_chain: Option<ApiResource>,
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
