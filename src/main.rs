mod parser;
mod error;


use std::error::Error;
use std::path::Path;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::result::Result::Ok;

///
///
///
///
fn main() -> Result<(), Box<dyn Error>> {
    let ssrs = read_lines("ssr.txt")?;
    for s in ssrs {
        let s = base64::decode(&s)?;
        let s = String::from_utf8(s)?.replace("ssr://", "");
        let s = s.lines();
        for s in s {
            if s.contains('_') {
                //let s = s.replace('_', "+");
                let s: String = String::from_utf8(base64::decode_config(&s, base64::URL_SAFE_NO_PAD)?)?;
                if !s.contains("/>") {
                    println!("error:{}", s);
                } else {
                    println!("{}", s);
                }
            } else {
                let s: String = String::from_utf8(base64::decode_config(&s, base64::STANDARD_NO_PAD)?)?;
                if !s.contains("/?") {
                    println!("error:{}", s);
                } else {
                    println!("{}", s);
                }
            }
        }
    }

    Ok(())
}

fn read_lines(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            result.push(line);
        }
    }
    Ok(result)
}
