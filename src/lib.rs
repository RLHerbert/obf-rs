use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub event: Event,
    pub sets: Vec<Set>,
    pub entrants: Vec<Entrant>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub name: String,
    pub date: Option<String>,
    pub game_name: Option<String>,
    pub tournament_structure: Option<String>,
    pub phases: Option<Vec<Phase>>,
    pub ruleset: Option<String>,
    pub origin_u_r_l: Option<String>,
    pub number_entrants: Option<u32>,
    pub other: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entrant {
    pub entrant_i_d: String,
    pub entrant_tag: Option<String>,
    pub initial_seed: Option<u32>,
    pub final_placement: Option<u32>,
    pub personal_information: Option<PersonalInformation>,
    pub other: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalInformation {
    pub name: Option<String>,
    pub country: Option<String>,
    pub other: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub set_i_d: String,
    pub entrant_1_i_d: Option<String>,
    pub entrant_2_i_d: Option<String>,
    pub status: Option<Status>,
    pub entrant_1_result: Option<EntrantResult>,
    pub entrant_2_result: Option<EntrantResult>,
    pub entrant_1_score: Option<i32>,
    pub entrant_2_score: Option<i32>,
    pub winner_next_set_i_d: Option<String>,
    pub loser_next_set_i_d: Option<String>,
    pub entrant_1_prev_set_i_d: Option<String>,
    pub entrant_2_prev_set_i_d: Option<String>,
    pub set_format: Option<String>,
    pub phase_i_d: Option<String>,
    pub round_i_d: Option<String>,
    pub games: Option<Vec<Game>>,
    pub other: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_number: u32,
    pub entrant_1_character: Option<String>,
    pub entrant_2_character: Option<String>,
    pub stage: Option<String>,
    pub entrant_1_result: Option<EntrantResult>,
    pub entrant_2_result: Option<EntrantResult>,
    pub other: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phase {
    pub phase_i_d: String,
    pub phase_structure: Option<String>,
    pub other: Option<HashMap<String, String>>,
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
