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
    // Read contents from file
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // Get word with counts
    let counts = get_word_count(&contents);

    // Get maximum counted word
    let (largest, max_count) = get_max_word(&counts).unwrap();

    // Sort the result in descending order
    let mut sorted = sort_hashmap(&counts);
    sorted.reverse();

    println!("'{}' is counted maximum {} times", largest, max_count);
    println!("total counts {:?}", sorted);

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

pub fn sort_hashmap<K, V>(map: &HashMap<K, V>) -> Vec<(&K, &V)>
where
    V: Ord,
{
    let mut sorted: Vec<_> = map.iter().collect();
    sorted.sort_by_key(|a| a.1);

    sorted
}

pub fn get_max_word<K, V>(map: &HashMap<K, V>) -> Option<(&K, &V)>
where
    V: Ord,
{
    map.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, v)| (k, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn it_gets_word_count() {
        let mut demo_data = HashMap::new();
        demo_data.insert("Two".to_string(), 2);

        let contents = "Two Two";
        let counts = get_word_count(&contents);

        assert_eq!(counts["Two"], 2);
    }

    #[test]
    fn it_gets_max_word() {
        let mut demo_counts = HashMap::new();
        demo_counts.insert("One", 1);
        demo_counts.insert("Two", 2);

        let (largest, max_count) = get_max_word(&demo_counts).unwrap();

        assert_eq!(*largest, String::from("Two"));
        assert_eq!(*max_count as i32, 2 as i32);
    }
}
