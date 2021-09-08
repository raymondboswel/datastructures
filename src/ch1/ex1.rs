use crate::ch1::read_lines::read_lines;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
