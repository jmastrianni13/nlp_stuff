use std::fs;
use std::io::BufRead;
use std::io::BufReader;

pub fn main() {
    let contents = read_into_vec();
    let processed_contents = process_ag_data(contents);
    preview(&processed_contents);
}

fn read_into_vec() -> Vec<String> {
    let file_path = "hf_ag_train.txt";
    let f = fs::File::open(&file_path).expect(&format!("could not read {}", file_path).to_string());
    // let f = fs::File::open(&file_path).expect("could not read file");

    let buffer = BufReader::new(f);
    let raw_content = buffer.lines()
        .map(|l| l.expect("could not parse line"))
        .collect::<Vec<_>>();

    return raw_content;

}

fn process_ag_data(raw_content: Vec<String>) -> Vec<(String, String, String)> {
    let mut processed = Vec::new();
    
    for line in raw_content.iter() {
        let mut parts = line.splitn(3, ',');
        let class = parts.next().unwrap().trim_matches('"');
        let title = parts.next().unwrap().trim_matches('"');
        let intro = parts.next().unwrap().trim_matches('"');
        processed.push(
            (
            class.to_string(),
            title.to_string(),
            intro.to_string(),
            )
        );
    }

    return processed;

}

fn preview<T: std::fmt::Debug>(contents: &Vec<T>) {
    for (l, line) in contents.iter().take(20).enumerate() {
        println!("{l}: {:?}", line);
    }
}


