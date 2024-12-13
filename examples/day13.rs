use aoc::*;
use itertools::Itertools;
use log::info;

const INPUT: &str = include_str!("inputs/day13.txt");

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!(env!("CARGO_BIN_NAME"));
    let lines = INPUT.lines();

    let mut machs: Vec<((Co2<i64>, Co2<i64>), Co2<i64>)> = vec![];
    let parse = |line: &str, delim| {
        let mut v = line
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| s.split_once(delim).unwrap().1.parse::<i64>().unwrap());
        (v.next().unwrap(), v.next().unwrap())
    };
    for mut lines in &lines.chunks(4) {
        let (oxa, oya) = parse(lines.next().unwrap(), '+');
        let (oxb, oyb) = parse(lines.next().unwrap(), '+');
        let (oxp, oyp) = parse(lines.next().unwrap(), '=');

        machs.push((
            (co2! {y: oya, x: oxa}, co2! {y: oyb, x: oxb}),
            co2! {y: oyp, x: oxp},
        ));
    }

    let p1: i64 = machs
        .iter()
        .cloned()
        .filter_map(|((a, b), p)| {
            let det = a.x() * b.y() - b.x() * a.y();
            let n0 = (p.x() * b.y() - p.y() * b.x()) / det;
            let n1 = (-p.x() * a.y() + p.y() * a.x()) / det;
            (a * n0 + b * n1 == p.into()).then(|| n0 * 3 + n1)
        })
        .sum();

    println!("{p1}");

    let p2: i64 = machs
        .iter()
        .cloned()
        .map(|(rest, p)| {
            (
                rest,
                Co2 {
                    row: p.row + 10000000000000,
                    col: p.col + 10000000000000,
                },
            )
        })
        .filter_map(|((a, b), p)| {
            let det = a.x() * b.y() - b.x() * a.y();
            let n0 = (p.x() * b.y() - p.y() * b.x()) / det;
            let n1 = (-p.x() * a.y() + p.y() * a.x()) / det;
            (a * n0 + b * n1 == p.into()).then(|| n0 * 3 + n1)
        })
        .sum();

    println!("{p2}");

    Ok(())
}
