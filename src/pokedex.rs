use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

use crate::encounters;

#[derive(Debug, Deserialize, Clone)]
pub struct PokemonEntry {
    pub name: String,
    pub slug: String,
    pub dex: Option<u32>,
    pub types: Vec<String>,
    pub stats: Stats,
    pub abilities: Abilities,
    pub evolution: String,
    pub held_item: String,
    pub location: String,
    pub egg_groups: Vec<String>,
    #[serde(default)]
    pub level_up_moves: Vec<LevelMove>,
    #[serde(default)]
    pub egg_moves: Vec<String>,
    #[serde(default)]
    pub tm_moves: Vec<String>,
    #[serde(default)]
    pub tutor_moves: Vec<String>,
    #[serde(default)]
    pub evolves_from: Option<String>,
    #[serde(default)]
    pub evolves_to: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Stats {
    pub hp: u32,
    #[serde(rename = "attack")]
    pub atk: u32,
    #[serde(rename = "defense")]
    pub def: u32,
    pub sp_atk: u32,
    pub sp_def: u32,
    pub speed: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Abilities {
    pub primary: String,
    pub secondary: String,
    pub hidden: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LevelMove {
    pub level: String,
    #[serde(rename = "move")]
    pub move_name: String,
}

pub struct LazarusPokedex {
    entries: HashMap<String, PokemonEntry>,
}

impl LazarusPokedex {
    pub fn load(path: PathBuf) -> Result<Self> {
        let data = fs::read_to_string(&path)
            .with_context(|| format!("reading pokedex data {}", path.display()))?;
        let entries: Vec<PokemonEntry> = serde_json::from_str(&data)
            .with_context(|| format!("parsing pokedex data {}", path.display()))?;
        let mut map = HashMap::new();
        for entry in entries {
            map.insert(entry.slug.clone(), entry);
        }
        Ok(Self { entries: map })
    }

    pub fn find(&self, identifier: &str) -> Option<&PokemonEntry> {
        let slug = encounters::slugify(identifier);
        self.entries.get(&slug)
    }

    pub fn get_by_slug(&self, slug: &str) -> Option<&PokemonEntry> {
        self.entries.get(slug)
    }

    pub fn evolution_chain(&self, slug: &str) -> Vec<&PokemonEntry> {
        let Some(root) = self.find_chain_root(slug) else {
            return Vec::new();
        };
        let mut chain = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while let Some(current) = queue.pop_front() {
            if let Some(entry) = self.entries.get(current) {
                chain.push(entry);
                let mut children: Vec<&String> = entry.evolves_to.iter().collect();
                children.sort();
                for child in children {
                    queue.push_back(child);
                }
            }
        }
        chain
    }

    fn find_chain_root(&self, slug: &str) -> Option<&str> {
        let mut current = self.entries.get(slug)?;
        while let Some(parent) = current
            .evolves_from
            .as_ref()
            .and_then(|s| self.entries.get(s))
        {
            current = parent;
        }
        Some(&current.slug)
    }
}
