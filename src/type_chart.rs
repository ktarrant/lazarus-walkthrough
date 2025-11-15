//! Gen 6+ Pokémon type chart data and helpers.

use std::fmt::Write;

pub const TYPE_COUNT: usize = 18;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Type {
    Normal = 0,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

impl Type {
    pub fn name(self) -> &'static str {
        match self {
            Type::Normal => "Normal",
            Type::Fire => "Fire",
            Type::Water => "Water",
            Type::Electric => "Electric",
            Type::Grass => "Grass",
            Type::Ice => "Ice",
            Type::Fighting => "Fighting",
            Type::Poison => "Poison",
            Type::Ground => "Ground",
            Type::Flying => "Flying",
            Type::Psychic => "Psychic",
            Type::Bug => "Bug",
            Type::Rock => "Rock",
            Type::Ghost => "Ghost",
            Type::Dragon => "Dragon",
            Type::Dark => "Dark",
            Type::Steel => "Steel",
            Type::Fairy => "Fairy",
        }
    }

    pub const fn as_index(self) -> usize {
        self as usize
    }
}

pub const ORDERED_TYPES: [Type; TYPE_COUNT] = [
    Type::Normal,
    Type::Fire,
    Type::Water,
    Type::Electric,
    Type::Grass,
    Type::Ice,
    Type::Fighting,
    Type::Poison,
    Type::Ground,
    Type::Flying,
    Type::Psychic,
    Type::Bug,
    Type::Rock,
    Type::Ghost,
    Type::Dragon,
    Type::Dark,
    Type::Steel,
    Type::Fairy,
];

const EFFECTIVENESS: [[f32; TYPE_COUNT]; TYPE_COUNT] = [
    // Normal
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.0, 1.0, 1.0, 0.5, 1.0,
    ],
    // Fire
    [
        1.0, 0.5, 0.5, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 2.0, 1.0,
    ],
    // Water
    [
        1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0,
    ],
    // Electric
    [
        1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0,
    ],
    // Grass
    [
        1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 1.0, 0.5, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 0.5, 1.0,
    ],
    // Ice
    [
        1.0, 0.5, 0.5, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0,
    ],
    // Fighting
    [
        2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5, 0.5, 0.5, 2.0, 0.0, 1.0, 2.0, 2.0, 0.5,
    ],
    // Poison
    [
        1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 0.0, 2.0,
    ],
    // Ground
    [
        1.0, 2.0, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0,
    ],
    // Flying
    [
        1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 0.5, 1.0,
    ],
    // Psychic
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.0, 0.5, 1.0,
    ],
    // Bug
    [
        1.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 1.0, 0.5, 2.0, 1.0, 1.0, 0.5, 1.0, 2.0, 0.5, 0.5,
    ],
    // Rock
    [
        1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0,
    ],
    // Ghost
    [
        0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0,
    ],
    // Dragon
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 0.0,
    ],
    // Dark
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5,
    ],
    // Steel
    [
        1.0, 0.5, 0.5, 0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 0.5, 2.0,
    ],
    // Fairy
    [
        1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 0.5, 1.0,
    ],
];

pub fn multiplier(attacking: Type, defending: Type) -> f32 {
    EFFECTIVENESS[attacking.as_index()][defending.as_index()]
}

pub fn markdown_table() -> String {
    let mut output = String::new();
    output.push('|');
    output.push_str(" Att \\ Def |");
    for def in ORDERED_TYPES.iter() {
        output.push(' ');
        output.push_str(def.name());
        output.push_str(" |");
    }
    output.push('\n');

    output.push('|');
    output.push_str(" --- |");
    for _ in ORDERED_TYPES.iter() {
        output.push_str(" --- |");
    }
    output.push('\n');

    for atk in ORDERED_TYPES.iter() {
        output.push('|');
        output.push(' ');
        output.push_str(atk.name());
        output.push_str(" |");
        for def in ORDERED_TYPES.iter() {
            let value = multiplier(*atk, *def);
            write!(&mut output, " {} |", format_multiplier(value)).expect("formatting to succeed");
        }
        output.push('\n');
    }

    output
}

fn format_multiplier(value: f32) -> &'static str {
    if (value - 2.0).abs() < f32::EPSILON {
        "2"
    } else if (value - 1.0).abs() < f32::EPSILON {
        "1"
    } else if (value - 0.5).abs() < f32::EPSILON {
        "0.5"
    } else if value.abs() < f32::EPSILON {
        "0"
    } else {
        "?"
    }
}
