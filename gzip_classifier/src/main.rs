mod classify_data;
mod read_data;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let sample_data = read_data::main();
    classify_data::main(&sample_data);
    let elapsed = start.elapsed();

    println!("real time: {:.3?}", elapsed);
}
