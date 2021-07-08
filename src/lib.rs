use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    event: Event,
    sets: Vec<Set>,
    entrants: Vec<Entrant>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    name: String,
    date: Option<String>,
    game_name: Option<String>,
    tournament_structure: Option<String>,
    phases: Option<Vec<Phase>>,
    ruleset: Option<String>,
    origin_u_r_l: Option<String>,
    number_entrants: Option<u32>,
    other: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entrant {
    entrant_i_d: String,
    entrant_tag: Option<String>,
    initial_seed: Option<u32>,
    final_placement: Option<u32>,
    personal_information: Option<PersonalInformation>,
    other: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalInformation {
    name: Option<String>,
    country: Option<String>,
    other: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    set_i_d: String,
    entrant_1_i_d: Option<String>,
    entrant_2_i_d: Option<String>,
    status: Option<Status>,
    entrant_1_result: Option<EntrantResult>,
    entrant_2_result: Option<EntrantResult>,
    entrant_1_score: Option<i32>,
    entrant_2_score: Option<i32>,
    winner_next_set_i_d: Option<String>,
    loser_next_set_i_d: Option<String>,
    entrant_1_prev_set_i_d: Option<String>,
    entrant_2_prev_set_i_d: Option<String>,
    set_format: Option<String>,
    phase_i_d: Option<String>,
    round_i_d: Option<String>,
    games: Option<Vec<Game>>,
    other: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    game_number: u32,
    entrant_1_character: Option<String>,
    entrant_2_character: Option<String>,
    stage: Option<String>,
    entrant_1_result: Option<EntrantResult>,
    entrant_2_result: Option<EntrantResult>,
    other: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phase {
    phase_i_d: String,
    phase_structure: Option<String>,
    other: Option<HashMap<String, String>>,
}

// #[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Completed,
    Started,
    Pending,
}

// #[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EntrantResult {
    Win,
    Lose,
    Draw,
}

#[cfg(test)]
mod tests {
    #[test]
    fn minimal() {
        let minimal_str = include_str!("example_json/minimal.json");

        let minimal_de: super::Tournament = serde_json::from_str(minimal_str).unwrap();

        let minimal_se = serde_json::to_string_pretty(&minimal_de).unwrap();

        assert!(minimal_str
            .split_whitespace()
            .eq((&minimal_se[..]).split_whitespace()))
    }

    #[test]
    fn example() {
        let example_str = include_str!("example_json/example.json");

        let example_de: super::Tournament = serde_json::from_str(example_str).unwrap();

        let example_se = serde_json::to_string_pretty(&example_de).unwrap();

        assert!(example_str
            .split_whitespace()
            .eq((&example_se[..]).split_whitespace()))
    }
}
