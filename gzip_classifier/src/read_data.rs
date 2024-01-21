use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

pub fn main(file_path: &str) -> Vec<ClassifiedText> {
    let contents = read_into_vec(file_path);
    let processed_contents = process_raw_data(contents);
    preview(&processed_contents);
    return processed_contents;
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ClassifiedText {
    pub class: u8,
    pub text: String,
}

fn read_into_vec(file_path: &str) -> Vec<String> {
    let f = fs::File::open(&file_path).expect(&format!("could not read {}", file_path).to_string());

    let buffer = BufReader::new(f);
    let raw_content = buffer
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect::<Vec<_>>();

    return raw_content;
}

fn process_raw_data(raw_content: Vec<String>) -> Vec<ClassifiedText> {
    let mut processed = Vec::new();

    for line in raw_content.iter() {
        let mut parts = line.splitn(2, ',');
        let class = parts.next().unwrap().trim_matches('"').trim().as_bytes()[0] - b'0';
        let text = parts.next().unwrap().trim_matches('"').trim().to_string();
        processed.push(ClassifiedText { class, text });
    }

    return processed;
}

fn preview(contents: &Vec<ClassifiedText>) {
    for c in contents.iter().take(5) {
        println!("{:?}", c);
    }

    let mut counts = HashMap::new();
    for sample in contents {
        let class = sample.class;
        *counts.entry(class).or_insert(0) += 1;
    }
    println!("class distribution: {:?}", counts);
}
