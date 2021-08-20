use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let counts = get_word_count(&contents);
    let (word, max_count) = get_max_word(&counts).unwrap();

    println!("'{}' is counted maximum {} times", word, max_count);
    // println!("count is {:?}", counts);

    Ok(())
}

pub fn get_word_count(contents: &str) -> HashMap<&str, u32> {
    let mut map = HashMap::new();

    for word in contents.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    map
}

pub fn get_max_word<K, V>(map: &HashMap<K, V>) -> Option<(&K, &V)>
where
    V: Ord,
{
    map.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, v)| (k, v))
}
