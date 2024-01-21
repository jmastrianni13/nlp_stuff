mod classify_data;
mod read_data;

use std::env;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("{:?}", file_path);
    let sample_data = read_data::main(file_path);
    classify_data::main(&sample_data);
    let elapsed = start.elapsed();

    println!("real time: {:.3?}", elapsed);
}
