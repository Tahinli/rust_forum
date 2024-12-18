use std::{collections::VecDeque, fs::File, io::Read};

use crate::error::ForumInputError;

pub fn naive_toml_parser(file_location: &str) -> (String, VecDeque<String>) {
    let mut toml_file = File::open(file_location).unwrap();
    let mut toml_ingredients = String::default();
    toml_file.read_to_string(&mut toml_ingredients).unwrap();
    let mut toml_ingredients = toml_ingredients.lines().collect::<VecDeque<&str>>();

    let header = toml_ingredients.pop_front().unwrap().trim_end().to_string();
    let parsed = toml_ingredients
        .iter()
        .map(|ingredient| {
            ingredient
                .split_once('=')
                .unwrap()
                .1
                .replace('"', "")
                .trim()
                .to_string()
        })
        .collect();

    (header, parsed)
}

pub fn input_checker(input: &String) -> Result<String, ForumInputError> {
    if input.is_empty() {
        Err(ForumInputError::EmptyParameter)
    } else {
        let input_without_space = input.split(' ').collect::<Vec<_>>().join("");
        if input_without_space.chars().all(char::is_alphanumeric) {
            Ok(input.to_owned())
        } else {
            Err(ForumInputError::ForbiddenCharacter)
        }
    }
}
