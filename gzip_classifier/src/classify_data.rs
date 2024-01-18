use crate::read_data;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

pub fn main(sample_data: &Vec<read_data::CleanedNewsSample>) {
    let target = &sample_data[0].text;
    let distances = get_distances(&target, sample_data);
}

fn get_distances(target: &str, data: &Vec<read_data::CleanedNewsSample>) -> Vec<f64> {
    let mut distances: Vec<f64> = Vec::with_capacity(data.len());
    let sample_count = &data.len();
    for (s, sample) in data.iter().enumerate() {
        let text = &sample.text;
        let ncd = get_ncd(target, &text);
        distances.push(ncd);
        print!("   {s}\r");
    }

    return distances;
}

fn get_ncd(x: &str, y: &str) -> f64 {
    let c_x = compress_string(x).len();
    let c_y = compress_string(y).len();
    let xy;
    let binding: String;
    if x == y {
        xy = x;
    } else {
        binding = concat_strings(x, y);
        xy = binding.as_str();
    }
    let c_xy = compress_string(xy).len();

    let min_c_x_c_y = c_x.min(c_y);
    let max_c_x_c_y = c_x.max(c_y);

    let ncd = (c_xy - min_c_x_c_y) as f64 / max_c_x_c_y as f64;

    return ncd;
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
