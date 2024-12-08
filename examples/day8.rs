use aoc::{co2, Co2};
use itertools::Itertools;
use log::info;
use std::collections::*;

const INPUT: &str = include_str!("inputs/day8.txt");
/*const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;*/

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!(env!("CARGO_BIN_NAME"));
    let lines = INPUT.lines();
    let (w, h) = (
        lines.clone().next().unwrap().chars().count() as isize,
        lines.clone().count() as isize,
    );
    let mut antennas: HashMap<char, Vec<Co2<usize>>> = HashMap::new();
    for (c, co2) in lines.clone().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c != &'.')
            .map(move |(x, c)| (c, co2!(x, y)))
    }) {
        antennas.entry(c).or_insert(Vec::new()).push(co2);
    }

    let in_bounds = |(x, y): (isize, isize)| x >= 0 && y >= 0 && x < w && y < h;

    let mut ans: HashSet<(isize, isize)> = HashSet::new();
    for (&l, &r) in antennas
        .keys()
        .flat_map(|c| antennas[c].iter().tuple_combinations::<(_, _)>())
    {
        let ofs = r - l;
        let an0 = (l - ofs).as_tuple();
        let an1 = (r + ofs).as_tuple();

        if in_bounds(an0) {
            ans.insert(an0);
        }
        if in_bounds(an1) {
            ans.insert(an1);
        }
    }
    println!("p1: {}", ans.len());

    let mut ans: HashSet<(isize, isize)> = HashSet::new();
    for (&l, &r) in antennas
        .keys()
        .flat_map(|c| antennas[c].iter().tuple_combinations::<(_, _)>())
    {
        ans.insert(r.try_as_tuple().unwrap());
        ans.insert(l.try_as_tuple().unwrap());

        let ofs = r - l;
        let mut an0 = l - ofs;
        let mut an1 = r + ofs;

        while in_bounds(an0.as_tuple()) {
            ans.insert(an0.as_tuple());
            an0 = an0 - ofs;
        }
        while in_bounds(an1.as_tuple()) {
            ans.insert(an1.as_tuple());
            an1 = an1 + ofs;
        }
    }
    println!("p2: {}", ans.len());

    for (y, line) in lines.clone().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if ans.contains(&(x as isize, y as isize)) {
                print!("#");
            } else {
                print!("{c}");
            }
        }
        println!();
    }

    Ok(())
}
