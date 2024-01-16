use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

pub fn main() {
    let contents = read_into_vec();
    let processed_contents = process_ag_data(contents);
    preview(&processed_contents);
    let combined_contents = combine_text(processed_contents);
    preview(&combined_contents);
}

fn read_into_vec() -> Vec<String> {
    let file_path = "hf_ag_train.txt";
    let f = fs::File::open(&file_path).expect(&format!("could not read {}", file_path).to_string());

    let buffer = BufReader::new(f);
    let raw_content = buffer
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect::<Vec<_>>();

    return raw_content;
}

fn process_ag_data(raw_content: Vec<String>) -> Vec<Vec<String>> {
    let mut processed = Vec::new();

    for line in raw_content.iter() {
        let mut parts = line.splitn(3, ',');
        let class = parts.next().unwrap().trim_matches('"');
        let title = parts.next().unwrap().trim_matches('"');
        let intro = parts.next().unwrap().trim_matches('"');
        processed.push(vec![
            class.to_string(),
            title.to_string(),
            intro.to_string(),
        ]);
    }

    return processed;
}

fn combine_text(contents: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut combined = Vec::new();

    for line in contents.iter() {
        let (class, title, intro) = (&line[0], &line[1], &line[2]);
        let mut text: String = title.to_string();
        text.push_str(" ");
        text.push_str(&intro);
        combined.push(vec![class.to_string(), text.to_string()]);
    }

    return combined;
}

fn preview<T: std::fmt::Debug + std::cmp::Eq + std::hash::Hash>(contents: &Vec<Vec<T>>) {
    for c in contents.iter().take(5) {
        println!("{:?}", c);
    }

    let mut counts = HashMap::new();
    for row in contents {
        let first_element = &row[0];
        *counts.entry(first_element).or_insert(0) += 1;
    }
    println!("class distribution: {:?}", counts);
}
