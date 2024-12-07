use itertools::Itertools;
use std::{collections::*, iter};

const INPUT: &str = include_str!("inputs/day7.txt");
/*const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;*/

fn recurse(acc: u64, vals: &[u64]) -> Vec<u64> {
    if vals.len() == 0 {
        return vec![acc];
    }
    let x = vals[0];
    recurse(acc + x, &vals[1..vals.len()])
        .into_iter()
        .chain(recurse(acc * x, &vals[1..vals.len()]))
        .collect_vec()
}

/*
fn orcurse(acc: u64, vals: &[u64]) -> Vec<u64> {
    if vals.len() == 0 {
        return vec![acc];
    }
    let rhss = recurse2(vals[0], &vals[1..vals.len()]);
    rhss.into_iter()
        .map(|rhs| {
            //println!("  {acc} || {x} = {}", acc.to_string() + &x.to_string());
            acc.to_string() + &rhs.to_string()
        })
        .map(|x| x.parse().unwrap())
        .collect_vec()
}*/

fn recurse2(acc: u64, vals: &[u64]) -> Vec<u64> {
    if vals.len() == 0 {
        return vec![acc];
    }
    if acc == 72 {
        println!("STOP");
    }
    let x = vals[0];
    recurse2(acc + x, &vals[1..vals.len()])
        .into_iter()
        .chain(recurse2(acc * x, &vals[1..vals.len()]))
        .chain(recurse2(
            (acc.to_string() + &x.to_string()).parse().unwrap(),
            &vals[1..vals.len()],
        ))
        .map(|val| {
            //println!("  {val}");
            val
        })
        .collect_vec()
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let mut p1 = 0;
    for line in lines.clone() {
        let (test, rest) = line.split_once(": ").unwrap();
        let test = test.parse::<u64>().unwrap();

        let vals = rest
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect_vec();

        let results = recurse(vals[0], &vals[1..]);
        if results.contains(&test) {
            p1 += test;
        }
    }
    println!("{p1}");

    let mut p2 = 0;
    for line in lines {
        let (test, rest) = line.split_once(": ").unwrap();
        let test = test.parse::<u64>().unwrap();

        //println!("test: {test}");
        let vals = rest
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect_vec();

        let results = recurse2(vals[0], &vals[1..]);
        //println!("  {results:?}");
        if results.contains(&test) {
            println!("{test}");
            p2 += test;
        }
    }
    println!("{p2}");

    Ok(())
}
// p2 wrong: 20281198396590
// p2 wrong: 74493743489417
