use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut counts = BTreeMap::new();
    for line in BufReader::new(stdin()).lines() {
        for ic in line.unwrap().chars().enumerate() {
            *counts.entry(ic.0).or_insert({
                let mut count = BTreeMap::new();
                count.insert(ic.1, 1);
                count
            }).entry(ic.1).or_insert(1) += 1;
        }
    }
    for count in counts.values() {
        print!("{}", count.iter().max_by_key(|n| n.1).unwrap().0);
    }
    println!("");
    for count in counts.values() {
        print!("{}", count.iter().min_by_key(|n| n.1).unwrap().0);
    }
    println!("");
}
