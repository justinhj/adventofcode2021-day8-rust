use scanf::sscanf;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Lines};

fn file_to_lines(file_path: &str) -> io::Result<Lines<BufReader<File>>> {
    let f = File::open(file_path)?;
    let b = BufReader::new(f);
    let lines = b.lines();
    Ok(lines)
}

#[derive(Debug)]
struct Pattern {
    patterns: Vec<HashSet<char>>,
    digits: Vec<HashSet<char>>,
}

fn parse_input(input: &str) -> Pattern {
    let split: Vec<&str> = input.split("|").collect();
    let patterns: Vec<&str> = split[0].split(" ").filter(|s| !s.is_empty()).collect();
    let digits: Vec<&str> = split[1].split(" ").filter(|s| !s.is_empty()).collect();

    Pattern {
        patterns: patterns.into_iter().map(|p| p.chars().collect()).collect(),
        digits: digits.into_iter().map(|p| p.chars().collect()).collect(),
    }
}

fn solve1(patterns: &Vec<Pattern>) -> u32 {
    let mut count = 0;
    let unique_digit_counts = vec!(2,3,4,7);
    for pattern in patterns {
        for digit in &pattern.digits {
            if unique_digit_counts.contains(&digit.len()) {
                count += 1;
            }
        }
    }
    count
}

fn update_candidate_map(
    new_digits: &HashSet<char>,
    candidates: &HashSet<char>,
    candidate_map: &HashMap<char,HashSet<char>>, // current map of new digit to originals
    ) -> HashMap<char,HashSet<char>> {
    let mut hm: HashMap<char, HashSet<char>> = HashMap::new();

    for digit in "abcdefg".chars() {
        let current = candidate_map.get(&digit).expect("Candidate map should have all digits");
        if new_digits.contains(&digit) {
            let intersect = current.intersection(&candidates).cloned().collect();
            hm.insert(digit, intersect);
        } else {
            hm.insert(digit, current.clone());
        }
    }

    hm
}

fn make_candidate_map() -> HashMap<char, HashSet<char>> {
    let cm: HashSet<char> = "abcdefg".chars().collect(); 
    let mut hm: HashMap<char, HashSet<char>> = HashMap::new();
    for digit in "abcdefg".chars() {
        hm.insert(digit, cm.clone());
    }
    hm
}

fn main() {
    let example_lines = file_to_lines("data/example.txt").unwrap();
    let example_signals: Vec<Pattern> = example_lines
        .into_iter()
        .map(|line| parse_input(&line.unwrap()))
        .collect();
    println!("example 1 {:?}", solve1(&example_signals));

    let input_lines = file_to_lines("data/input.txt").unwrap();
    let input_signals: Vec<Pattern> = input_lines
        .into_iter()
        .map(|line| parse_input(&line.unwrap()))
        .collect();
    println!("input 1 {:?}", solve1(&input_signals));
}

#[test]
fn compare_sets() {
    let s1: HashSet<char> = "cdfeb".chars().collect();
    let s2: HashSet<char> = "ecbfd".chars().collect();
    assert_eq!(s1,s2);
    let s3: HashSet<char> = "zcbde".chars().collect();
    assert_ne!(s1,s3);
}

#[test]
fn interset_sets() {
    let s1: HashSet<char> = "fab".chars().collect();
    let s2: HashSet<char> = "ab".chars().collect();
    let s3: HashSet<char> = s1.intersection(&s2).cloned().collect();
    let s4: HashSet<char> = "ab".chars().collect();
    assert_eq!(s3,s4);
}

#[test]
fn candidate_map() {
    let new_digits: HashSet<char> = "cf".chars().collect();
    let originals: HashSet<char> = "ab".chars().collect();
    let cm = make_candidate_map();
    let updated = update_candidate_map(&new_digits, &originals, &cm);
    assert_eq!(updated.get(&'c'), Some(&originals));
    assert_eq!(updated.get(&'f'), Some(&originals));
    assert_eq!(updated.get(&'a'), cm.get(&'a'));
}
