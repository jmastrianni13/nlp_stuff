use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

pub fn main() {
    let contents = read_into_vec();
    let processed_contents = process_ag_data(contents);
    let combined_contents = combine_text(processed_contents);
    preview(&combined_contents);
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct NewsSample {
    class: u8,
    title: String,
    intro: String,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct CleanedNewsSample {
    class: u8,
    text: String,
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

fn process_ag_data(raw_content: Vec<String>) -> Vec<NewsSample> {
    let mut processed = Vec::new();

    for line in raw_content.iter() {
        let mut parts = line.splitn(3, ',');
        let class = parts.next().unwrap().trim_matches('"').as_bytes()[0] - b'0';
        let title = parts.next().unwrap().trim_matches('"').to_string();
        let intro = parts.next().unwrap().trim_matches('"').to_string();
        let sample = NewsSample {
            class,
            title,
            intro,
        };
        processed.push(sample);
    }

    return processed;
}

fn combine_text(contents: Vec<NewsSample>) -> Vec<CleanedNewsSample> {
    let mut combined = Vec::new();

    for sample in contents.iter() {
        let text = format!("{} {}", sample.title, sample.intro);
        let cleaned_sample = CleanedNewsSample {
            class: sample.class,
            text,
        };
        combined.push(cleaned_sample);
    }

    return combined;
}

fn preview(contents: &Vec<CleanedNewsSample>) {
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
