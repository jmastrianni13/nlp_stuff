use crate::read_data;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

pub fn main(sample_data: &Vec<read_data::CleanedNewsSample>) {
    // let target = &sample_data[0].text;
    let target = "New England Patriots Hire Mayo for HC The New England Patriots named Jerod Mayo, former first round pick, as new head coach";
    // let target = "Dow Jones drops 15% today on Unemployment News The Dow Jones Industrial Average fell today after DOL released forecasts painting a gloomy picture for unemployment for the remainder of 2023";
    // let target = "More delays for NASA’s astronaut moonshots, with crew landing off until 2026  Astronauts will have to wait until next year before flying to the moon and at least two years before landing on it, under the latest round of delays announced by NASA on Tuesday.";
    let distances = get_distances(&target, sample_data);
    let pred_class = &distances[0].class;
    let pred_class_name = match pred_class {
        1 => "World",
        2 => "Sports",
        3 => "Business",
        4 => "Sci/Tech",
        _ => "Unknown",
    };

    println!("{} > {}", target, pred_class_name);
}

#[derive(Debug)]
struct TextDistance {
    class: u8,
    distance: f64,
}

fn get_distances(target: &str, data: &Vec<read_data::CleanedNewsSample>) -> Vec<TextDistance> {
    let mut distances: Vec<TextDistance> = Vec::with_capacity(data.len());
    let sample_count = &data.len();
    for (s, sample) in data.iter().enumerate() {
        let text = &sample.text;
        let ncd = get_ncd(target, &text);
        let text_distance = TextDistance {
            class: sample.class.clone(),
            distance: ncd,
        };
        distances.push(text_distance);
        print!("   {s}\r");
    }
    println!();

    distances.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    return distances;
}

fn get_ncd(x: &str, y: &str) -> f64 {
    let c_x = compress_string(x).len();
    let c_y = compress_string(y).len();
    let binding: String;
    let c_xy;
    if x == y {
        c_xy = c_x;
    } else {
        let mut buffer = Vec::with_capacity(x.len() + y.len());
        buffer.extend_from_slice(x.as_bytes());
        buffer.extend_from_slice(y.as_bytes());
        c_xy = compress_buffer(&buffer).len();
    }

    let min_c_x_c_y = c_x.min(c_y);
    let max_c_x_c_y = c_x.max(c_y);

    let ncd = (c_xy - min_c_x_c_y) as f64 / max_c_x_c_y as f64;

    return ncd;
}

fn compress_buffer(target: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = encoder.write(target);
    return encoder.finish().unwrap();
}

fn compress_string(target: &str) -> Vec<u8> {
    let target_bytes = &target.as_bytes();
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = encoder.write(target_bytes);
    let compressed_target = encoder.finish().unwrap();
    return compressed_target;
}

fn concat_strings(string_a: &str, string_b: &str) -> String {
    return format!("{string_a}{string_b}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_string() {
        let target_string = String::from("a long string for testing the compression that needs enough text so the ize of the compression plus metadata about the compression is less than the size of the original target string");
        let target_string_size = target_string.clone().into_bytes().len();
        let compressed_string = compress_string(&target_string);

        assert!(compressed_string.len() < target_string_size);
    }

    #[test]
    fn test_concat_string() {
        let string_a = "string a";
        let string_b = "string b";
        let concat_string = concat_strings(string_a, string_b);
        assert_eq!(concat_string, "string astring b");
    }

    #[test]
    fn test_ncd_x_eq_y() {
        let x = "some string for testing the ncd function";
        let y = "some string for testing the ncd function";
        let ncd = get_ncd(x, y);
        assert_eq!(ncd, 0.0);
    }

    #[test]
    fn test_ncd_x_neq_y() {
        let x = "some string for testing the ncd function";
        let y = "some other string for testing the ncd function";
        let ncd = get_ncd(x, y);
        assert!(ncd != 0.0);
    }
}
