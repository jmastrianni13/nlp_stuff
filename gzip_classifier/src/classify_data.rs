use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

pub fn main() {
    println!("hello classify_data.rs");
    let x = "fjreiofrejaioerajreaiorgjeao;greahjrgea grjaeigo;aeghjrae og;aehg raeo;grhraeo;ghjraejio;a jieao;jrao;ivgjrae ojreao;joajv reao;oja fjawsfioahf hfoda";
    let y = "fdhasaondfava nvjaddvnavbdafnv ifabvfi abvaibvfeiabviaebvfa baibvidabvibabv aibbviaefaibabav baibvai";
    let my_ncd = get_ncd(x.to_string(), y.to_string());
    println!("{:?}", my_ncd);
}

fn get_ncd(x: String, y: String) -> f64 {
    let c_x = compress_string(&x).len();
    let c_y = compress_string(&y).len();
    let xy = concat_strings(&x, &y);
    let c_xy = compress_string(&xy).len();

    let min_c_x_c_y = c_x.min(c_y);
    let max_c_x_c_y = c_x.max(c_y);

    let ncd = (c_xy - min_c_x_c_y) as f64 / max_c_x_c_y as f64;

    return ncd;
}

fn compress_string(target: &String) -> Vec<u8> {
    let target_bytes = &target.clone().into_bytes();
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write(target_bytes);
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
}
