use seahorse::{App, Context, Flag, FlagType};
use std::env;
use itertools::Itertools;
use base64::{Engine, engine::general_purpose};
use arboard::Clipboard;

fn main() {
	let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .action(default_action)
        .flag(
            Flag::new("decode", FlagType::Bool)
                .description("Decode")
                .alias("d"),
        );

    app.run(args);
}

fn default_action(c: &Context) {
    let mut clipboard = Clipboard::new().unwrap();
    if c.bool_flag("decode") {
        let decoded = decode_str(&c.args[0]);
        clipboard.set_text(decoded.clone()).unwrap();
        println!("'{}' is copied!", decoded);
    } else {
        let encoded = encode_vec_str(&c.args);
        clipboard.set_text(encoded.clone()).unwrap();
        println!("'{}' is copied!", encoded);
    }
}

fn encode_vec_str(strs: &Vec<String>) -> String {
    let joined_str = strs.iter().join(":");
    let encoded = general_purpose::STANDARD.encode(joined_str);
    return encoded;
}

fn decode_str(str: &String) -> String {
    let bytes = general_purpose::STANDARD.decode(str).unwrap();
    let decoded = bytes.iter().map(|&s| s as char).collect::<String>();
    return decoded;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let args = vec!["hello".to_string(), "world".to_string()];
        let expected = "aGVsbG86d29ybGQ=";
        let actual = encode_vec_str(&args);
        assert_eq!(expected, actual);
    }

    #[test]
    fn decode() {
        let args = "aGVsbG86d29ybGQ=".to_string();
        let expected = "hello:world";
        let actual = decode_str(&args);
        assert_eq!(expected, actual);
    }
}

