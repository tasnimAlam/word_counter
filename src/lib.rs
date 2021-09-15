//! # Word Counter
//!
//! A program that displays word count of a file.

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;
#[macro_use]
extern crate prettytable;
use prettytable::Table;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "word_counter",
    about = "A program that displays word count of a file."
)]
pub struct Opt {
    /// Top result limit
    #[structopt(short = "t", long = "--top", default_value = "10")]
    top: usize,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Reverse order
    #[structopt(short, long)]
    reverse: bool,

    /// Search specific word
    #[structopt(short = "s", long = "--search")]
    search: Option<String>,

    /// Show most counted word
    #[structopt(short = "m", long = "--max")]
    show_max: bool,

    /// Duration of all the calculations
    #[structopt(short = "d", long = "--duration")]
    duration: bool,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

pub fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    let time = Instant::now();

    // Read contents from file
    let mut f = File::open(opt.input)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // Get searched word counts
    let search: Option<String> = opt.search;

    // When there is no word to search, count all words
    if let None = search {
        let counts = get_word_count(&contents);

        // Sort the result in descending order
        let mut result = sort_hashmap(&counts);

        // Reverse the search order
        if !opt.reverse {
            result.reverse();
        }

        // Show only the top results
        result.truncate(opt.top);

        // Show maximum counted word
        if opt.show_max {
            let (max_word, max_count) = get_max_word(&counts).unwrap();

            // Create table for maximum word count
            let mut table = Table::new();
            table.add_row(row!["Maximum count", &max_word, &max_count]);
            table.printstd();
        }

        // Print count table
        print_counts(&result);
    }

    // When there is a word to search, only count that word
    if let Some(i) = search {
        let searched_word = String::from(i);
        let count = get_search_count(&searched_word, &contents);

        //  Create search result table
        let mut table = Table::new();
        table.add_row(row!["Search result", &searched_word, &count]);
        table.printstd();
    }

    // Print program duration
    if opt.duration {
        println!("Duration : {}ms", time.elapsed().as_millis());
    }

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

pub fn get_search_word<'a>(map: &'a HashMap<&'a str, u32>, query: &str) -> Option<(&'a str, u32)> {
    map.iter().find_map(|(&key, val)| {
        if key == query {
            Some((key, *val))
        } else {
            None
        }
    })
}

// FIXME: need to check "&&str"
pub fn print_counts(vec: &Vec<(&&str, &u32)>) {
    // Create the table
    let mut table = Table::new();
    table.add_row(row!["Word", "Count"]);

    for (key, value) in vec {
        table.add_row(row![&key, &value]);
    }

    // Print the table to stdout
    table.printstd();
}

pub fn get_search_count(text: &str, contents: &str) -> u32 {
    let mut count = 0;

    for word in contents.split_whitespace() {
        if word == text {
            count += 1;
        }
    }

    count
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

    #[test]
    fn it_gets_searched_word() {
        let mut demo_counts = HashMap::new();
        demo_counts.insert("One", 1);
        demo_counts.insert("Two", 2);

        let searched_word = String::from("Two");
        let no_result = (&searched_word[..], 2 as u32);
        let (word, count) = get_search_word(&demo_counts, &searched_word).unwrap_or(no_result);

        assert_eq!(word, "Two");
        assert_eq!(count, 2);
    }

    #[test]
    fn it_sorts_hashmap() {
        let mut map = HashMap::new();
        map.insert("One", 1);
        map.insert("Two", 2);

        let mut sorted_counts = sort_hashmap(&map);
        sorted_counts.reverse();
        let demo_counts = [("One", 1), ("Two", 2)];

        assert_eq!(demo_counts.len(), sorted_counts.len());
        assert_eq!(demo_counts[0].1, *sorted_counts[1].1);
        assert_eq!(demo_counts[1].0, *sorted_counts[0].0);
    }

    #[test]
    fn it_gets_search_count() {
        let contents = "one one two two one one";
        let text = "one";

        assert_eq!(get_search_count(&text, &contents), 4);
    }
}
