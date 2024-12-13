use itertools::Itertools;
use log::info;
use std::collections::*;

const INPUT: &str = include_str!("inputs/day5.txt");

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!(env!("CARGO_BIN_NAME"));
    let mut lines = INPUT.lines();

    let mut rules = HashMap::<usize, usize>::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (l, r) = line.split_once('|').unwrap();
        rules.insert(l.parse().unwrap(), r.parse().unwrap());
    }

    let mut prints = Vec::<Vec<usize>>::new();
    while let Some(line) = lines.next() {
        prints.push(line.split(',').map(|s| s.parse().unwrap()).collect_vec());
    }

    let mut p1 = 0;
    'outer: for print in prints {
        for (l, r) in print.iter().tuple_combinations() {
            if let Some(must_be_before) = rules.get(l) {
                if r == must_be_before && l >= must_be_before {
                    continue 'outer;
                }
            }
        }
        p1 += print[print.len() / 2];
    }
    println!("{p1}");

    Ok(())
}

// p1 2719 too low
// p1 11681 too high
// p1 9121 too high
// p1 3420 wrong
