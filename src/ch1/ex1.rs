use crate::ch1::read_lines::read_lines;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub fn reverse_lines<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut stack = Vec::new();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for res in lines {
            if let Ok(line) = res {
                stack.push(line);
            }
        }

        while let Some(top) = stack.pop() {
            println!("{}", top)
        }
    }
}

pub fn reverse_n_lines<P>(path: P, n: u8)
where
    P: AsRef<Path>,
{
    let mut stack = Vec::new();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for res in lines {
            if let Ok(line) = res {
                stack.push(line);
                if stack.len() == n.into() {
                    while let Some(top) = stack.pop() {
                        println!("{}", top)
                    }
                }
            }
        }

        while let Some(top) = stack.pop() {
            println!("{}", top)
        }
    }
}

pub fn queue_n_till_blank_line<P>(path: P, n: u8)
where
    P: AsRef<Path>,
{
    let max_length = 5;
    let mut queue = VecDeque::with_capacity(6);
    if let Ok(lines) = read_lines(path) {
        for res in lines {
            if let Ok(line) = res {
                queue.push_front(line);
                if queue.len() > max_length {
                    queue.pop_back();
                }
                let empty = "".to_string();
                match queue.front() {
                    Some(l) => match l.as_ref() {
                        "" => println!("{}", queue.back().unwrap()),
                        _ => println!(""),
                    },
                    _ => {
                        println!("Nothing at front of queue!")
                    }
                }
            }
        }
    }
}

pub fn filter_duplicates<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut uniq_lines = HashMap::new();

    if let Ok(lines) = read_lines(path) {
        for res in lines {
            if let Ok(line) = res {
                // Create hash of string
                let mut s = DefaultHasher::new();
                line.hash(&mut s);
                let hash = s.finish();
                if !uniq_lines.contains_key(&hash) {
                    uniq_lines.insert(hash, line);

                    println!("{}", uniq_lines.get(&hash).unwrap());
                }
            }
        }
    }
}

pub fn only_duplicates<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut uniq_lines = HashMap::new();

    if let Ok(lines) = read_lines(path) {
        for res in lines {
            if let Ok(line) = res {
                // Create hash of string
                let mut s = DefaultHasher::new();
                line.hash(&mut s);
                let hash = s.finish();
                if uniq_lines.contains_key(&hash) {
                    println!("{}", uniq_lines.get(&hash).unwrap());
                }
                uniq_lines.insert(hash, line);
            }
        }
    }
}

pub fn filter_duplicates_sort_by_length<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut uniq_lines = BTreeSet::new();

    if let Ok(lines) = read_lines(path) {
        for res in lines {
            if let Ok(line) = res {
                uniq_lines.insert(line);
            }
        }

        let mut v: Vec<_> = uniq_lines.into_iter().collect();
        v.sort_by(|a, b| a.len().cmp(&b.len()));

        for l in v {
            println!("{}", l);
        }
    }
}
