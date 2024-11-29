use std::{fs::File, io::Read};

pub fn naive_toml_parser(file_location: &str) -> (String, Vec<String>) {
    let mut toml_file = File::open(file_location).unwrap();
    let mut toml_ingredients = String::default();
    toml_file.read_to_string(&mut toml_ingredients).unwrap();
    let mut toml_ingredients = toml_ingredients.lines().collect::<Vec<&str>>();

    let header = toml_ingredients.remove(0).trim_end().to_string();
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
