use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Lines};

fn file_to_lines(file_path: &str) -> io::Result<Lines<BufReader<File>>> {
    let f = File::open(file_path)?;
    let b = BufReader::new(f);
    let lines = b.lines();
    Ok(lines)
}

struct Pattern {
    patterns: Vec<HashSet<char>>,
    digits: Vec<HashSet<char>>,
}

fn parse_input(input: &str) -> Pattern {
    let split: Vec<&str> = input.split('|').collect();
    let patterns = split[0].split(' ').filter(|s| !s.is_empty());
    let digits = split[1].split(' ').filter(|s| !s.is_empty());

    Pattern {
        patterns: patterns.into_iter().map(|p| p.chars().collect()).collect(),
        digits: digits.into_iter().map(|p| p.chars().collect()).collect(),
    }
}

fn solve1(patterns: &[Pattern]) -> u32 {
    let mut count = 0;
    let unique_digit_counts = vec![2, 3, 4, 7];
    for pattern in patterns {
        for digit in &pattern.digits {
            if unique_digit_counts.contains(&digit.len()) {
                count += 1;
            }
        }
    }
    count
}

// TODO there must be a better pattern here than passing references to hashmaps and then 
// making copies of them for the next step for the recursion.
fn assign_mapping(
    input: &[HashSet<char>], // list of inputs sorted by len
    candidate_map: &HashMap<char, HashSet<char>>, // current map of new digit to originals
    digit_segments: &HashMap<u8, HashSet<char>>, // map of digits to the original segments they use
    digits_unused: &HashSet<u8>, // digits we have looked at and removed from consideration
) -> Option<HashMap<char, HashSet<char>>> {
    if input.is_empty() {
        return Some(candidate_map.clone());
    }

    let head = input.first()?;
    let hl = head.len() as u8;
    let candidates: Vec<u8> = find_digits_with_n_segments(hl, digit_segments)
        .iter()
        .filter(|digit| digits_unused.contains(digit))
        .cloned()
        .collect();

    if candidates.len() > 1 {
        // just skip non-unique ones
        let tail = &input[1..];
        return assign_mapping(tail, candidate_map, digit_segments, digits_unused);
    }

    for digit in candidates {
        if let Some(segments) = digit_segments.get(&digit) {
            let mut new_digits_unused = digits_unused.clone();
            new_digits_unused.remove(&digit);
            let tail = &input[1..];
            let updated_candidate_map = update_candidate_map(head, segments, candidate_map);

            let result = assign_mapping(
                tail,
                &updated_candidate_map,
                digit_segments,
                &new_digits_unused,
            );

            if result.is_some() {
                return result;
            }
        }
    }

    None
}

// TODO doesn't seem a very idiotatic name
fn make_digit_segments() -> HashMap<u8, HashSet<char>> {
    HashMap::from([
        (0, "abcefg".chars().collect()),
        (1, "cf".chars().collect()),
        (2, "acdeg".chars().collect()),
        (3, "acdfg".chars().collect()),
        (4, "bcdf".chars().collect()),
        (5, "abdfg".chars().collect()),
        (6, "abdefg".chars().collect()),
        (7, "acf".chars().collect()),
        (8, "abcdefg".chars().collect()),
        (9, "abcdfg".chars().collect()),
    ])
}

fn find_digits_with_n_segments(n: u8, digit_segments: &HashMap<u8, HashSet<char>>) -> Vec<u8> {
    digit_segments
        .iter()
        .filter(|(_, v)| v.len() as u8 == n)
        .map(|(k, _)| *k)
        .collect()
}

fn cm_helper(
    input: &HashMap<char, HashSet<char>>,
    om: &HashMap<char, char>,
    oms: &mut Vec<HashMap<char, char>>,
) {
    // Get one key, if we can't get one we're done...
    let keys: Vec<(&char, &HashSet<char>)> = input.iter().take(1).collect();
    if keys.is_empty() {
        oms.insert(0, om.clone())
    } else {
        let keyvalue = keys[0];
        for candidate in keyvalue.1 {
            // Don't consider candidate if it is already assigned in the output
            if om.values().any(|v| v == candidate) {
                continue;
            }

            let mut next_input = input.clone();
            next_input.remove(keyvalue.0);

            let mut next_output = om.clone();
            next_output.insert(*keyvalue.0, *candidate);

            cm_helper(&next_input, &next_output, oms);
        }
    }
}

// Take a mappying of segments to possible original segments and reduce it down to a consistent
// mapping with only one value per input by brute force
fn consistent_mapping(cm: &HashMap<char, HashSet<char>>) -> Vec<HashMap<char, char>> {
    let collector: &mut Vec<HashMap<char, char>> = &mut vec![];
    cm_helper(cm, &HashMap::new(), collector);
    collector.clone()
}

fn solve_pattern(pattern: &Pattern) -> u64 {
    let candidate_map = make_candidate_map();
    let digit_segments = make_digit_segments();
    let unused_digits = (0..=9).collect();
    let mut sorted_pattern = pattern.patterns.clone();
    sorted_pattern.sort_by_key(|k| k.len());
    let candidate_map = assign_mapping(
        &sorted_pattern,
        &candidate_map,
        &digit_segments,
        &unused_digits,
    );

    // Now we reduced the search space to a smaller map find a valid
    // mapping with brute force

    let cm = candidate_map.expect("Candidate map should not be empty");
    let candidate_map_2 = consistent_mapping(&cm);

    // We have a bunch of candidate maps now try them out for a solution

    for map in candidate_map_2 {
        // Remap the segment digits
        let remapped_digits: Vec<HashSet<char>> = pattern
            .digits
            .iter()
            .map(|a| {
                a.iter()
                    .map(|s| {
                        *map
                            .get(s)
                            .unwrap_or_else(|| panic!("Unknown segment {:?}", s))
                    })
                    .collect()
            })
            .collect();

        // Lookup what each digit is
        let candidate_solution: Vec<Option<&u8>> = remapped_digits
            .iter()
            .map(|segments| {
                digit_segments
                    .iter()
                    .find(|(_, v)| *v == segments)
            })
            .map(|n| n.map(|s| s.0))    
            .collect();

        if candidate_solution.iter().all(|n| (*n).is_some()) {
            let mut multiplier: u64 = 1;
            let mut acc: u64 = 0;
            // now convert it into a number and return it
            for digit in candidate_solution.iter().rev() {
                let num: u64 = *digit.expect("Whoops") as u64;
                acc += num * multiplier;
                multiplier *= 10;
            }
            return acc;
        }

    }
    panic!("no solution");
}

fn solve2(patterns: &[Pattern]) -> u64 {
    let mut sum = 0;
    for pattern in patterns {
        sum += solve_pattern(pattern);
    }
    sum
}

fn update_candidate_map(
    new_digits: &HashSet<char>,
    candidates: &HashSet<char>,
    candidate_map: &HashMap<char, HashSet<char>>, // current map of new digit to originals
) -> HashMap<char, HashSet<char>> {
    let mut hm: HashMap<char, HashSet<char>> = HashMap::new();

    for digit in "abcdefg".chars() {
        let current = candidate_map
            .get(&digit)
            .expect("Candidate map should have all digits");
        if new_digits.contains(&digit) {
            let intersect = current.intersection(candidates).cloned().collect();
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
    println!("example 2 {:?}", solve2(&example_signals));

    let input_lines = file_to_lines("data/input.txt").unwrap();
    let input_signals: Vec<Pattern> = input_lines
        .into_iter()
        .map(|line| parse_input(&line.unwrap()))
        .collect();
    println!("input 1 {:?}", solve1(&input_signals));
    println!("input 2 {:?}", solve2(&input_signals));
}

#[test]
fn compare_sets() {
    let s1: HashSet<char> = "cdfeb".chars().collect();
    let s2: HashSet<char> = "ecbfd".chars().collect();
    assert_eq!(s1, s2);
    let s3: HashSet<char> = "zcbde".chars().collect();
    assert_ne!(s1, s3);
}

#[test]
fn interset_sets() {
    let s1: HashSet<char> = "fab".chars().collect();
    let s2: HashSet<char> = "ab".chars().collect();
    let s3: HashSet<char> = s1.intersection(&s2).cloned().collect();
    let s4: HashSet<char> = "ab".chars().collect();
    assert_eq!(s3, s4);
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

#[test]
fn find_digits_with_6_segments_test() {
    let ds = make_digit_segments();
    let mut left = find_digits_with_n_segments(6, &ds);
    left.sort();
    let mut right = vec![0, 6, 9];
    right.sort();

    assert_eq!(left, right);
}

#[test]
fn find_digits_with_5_segments_test() {
    let ds = make_digit_segments();
    let mut left = find_digits_with_n_segments(5, &ds);
    left.sort();
    let mut right = vec![2, 3, 5];
    right.sort();

    assert_eq!(left, right);
}

#[test]
fn find_digits_with_2_segments_test() {
    let ds = make_digit_segments();
    let mut left = find_digits_with_n_segments(2, &ds);
    left.sort();
    let mut right = vec![1];
    right.sort();

    assert_eq!(left, right);
}

#[test]
fn part2_test_pattern() {
    let example_lines = file_to_lines("data/example2.txt").unwrap();
    let example_signals: Vec<Pattern> = example_lines
        .into_iter()
        .map(|line| parse_input(&line.unwrap()))
        .collect();

    solve_pattern(&example_signals[0]);
}
