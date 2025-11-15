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

    pub fn short_name(self) -> &'static str {
        match self {
            Type::Normal => "Nor",
            Type::Fire => "Fir",
            Type::Water => "Wat",
            Type::Electric => "Ele",
            Type::Grass => "Gra",
            Type::Ice => "Ice",
            Type::Fighting => "Fig",
            Type::Poison => "Poi",
            Type::Ground => "Gro",
            Type::Flying => "Fly",
            Type::Psychic => "Psy",
            Type::Bug => "Bug",
            Type::Rock => "Roc",
            Type::Ghost => "Gho",
            Type::Dragon => "Dra",
            Type::Dark => "Dar",
            Type::Steel => "Ste",
            Type::Fairy => "Fai",
        }
    }

    pub const fn as_index(self) -> usize {
        self as usize
    }

    pub fn from_api_name(name: &str) -> Option<Self> {
        match name {
            "normal" => Some(Type::Normal),
            "fire" => Some(Type::Fire),
            "water" => Some(Type::Water),
            "electric" => Some(Type::Electric),
            "grass" => Some(Type::Grass),
            "ice" => Some(Type::Ice),
            "fighting" => Some(Type::Fighting),
            "poison" => Some(Type::Poison),
            "ground" => Some(Type::Ground),
            "flying" => Some(Type::Flying),
            "psychic" => Some(Type::Psychic),
            "bug" => Some(Type::Bug),
            "rock" => Some(Type::Rock),
            "ghost" => Some(Type::Ghost),
            "dragon" => Some(Type::Dragon),
            "dark" => Some(Type::Dark),
            "steel" => Some(Type::Steel),
            "fairy" => Some(Type::Fairy),
            _ => None,
        }
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

pub fn colored_table() -> String {
    let mut output = String::new();
    output.push_str(r#"<table class="type-chart">"#);
    output.push('\n');
    output.push_str("<thead><tr><th>Att \\ Def</th>");
    for def in ORDERED_TYPES.iter() {
        write!(
            &mut output,
            r#"<th><abbr title="{full}">{short}</abbr></th>"#,
            full = def.name(),
            short = def.short_name()
        )
        .expect("writing header");
    }
    output.push_str("</tr></thead>\n<tbody>\n");

    for atk in ORDERED_TYPES.iter() {
        write!(
            &mut output,
            r#"<tr><th scope="row"><abbr title="{full}">{short}</abbr></th>"#,
            full = atk.name(),
            short = atk.short_name()
        )
        .expect("writing row header");
        for def in ORDERED_TYPES.iter() {
            let value = multiplier(*atk, *def);
            let display = format_multiplier(value);
            let color = color_for_multiplier(value);
            write!(
                &mut output,
                r#"<td style="background-color:{color};text-align:center;font-weight:500">{display}×</td>"#,
                color = color,
                display = display
            )
            .expect("writing cell");
        }
        output.push_str("</tr>\n");
    }

    output.push_str("</tbody>\n</table>\n");
    output
}

pub fn format_multiplier(value: f32) -> &'static str {
    if (value - 4.0).abs() < f32::EPSILON {
        "4"
    } else if (value - 2.0).abs() < f32::EPSILON {
        "2"
    } else if (value - 1.0).abs() < f32::EPSILON {
        "1"
    } else if (value - 0.5).abs() < f32::EPSILON {
        "0.5"
    } else if (value - 0.25).abs() < f32::EPSILON {
        "0.25"
    } else if value.abs() < f32::EPSILON {
        "0"
    } else {
        "?"
    }
}

fn color_for_multiplier(value: f32) -> &'static str {
    if (value - 2.0).abs() < f32::EPSILON {
        "#c8f7c5"
    } else if (value - 1.0).abs() < f32::EPSILON {
        "#ffffff"
    } else if (value - 0.5).abs() < f32::EPSILON {
        "#ffe6b3"
    } else if value.abs() < f32::EPSILON {
        "#e0e0e0"
    } else {
        "#f5f5f5"
    }
}
