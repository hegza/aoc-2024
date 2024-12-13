use aoc::*;
use itertools::Itertools;
use log::info;
use std::collections::*;

const INPUT: &str = include_str!("inputs/day13.txt");

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!(env!("CARGO_BIN_NAME"));
    let lines = INPUT.lines();

    let mut machs: Vec<((Co2<i64>, Co2<i64>), Co2<i64>)> = vec![];
    for mut lines in &lines.chunks(4) {
        let mut a = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| s.split_once('+').unwrap().1.parse::<i64>().unwrap());
        let (oxa, oya) = (a.next().unwrap(), a.next().unwrap());

        let mut b = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| s.split_once('+').unwrap().1.parse::<i64>().unwrap());
        let (oxb, oyb) = (b.next().unwrap(), b.next().unwrap());

        let mut p = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| s.split_once('=').unwrap().1.parse::<i64>().unwrap());
        let (oxp, oyp) = (p.next().unwrap(), p.next().unwrap());

        machs.push((
            (co2! {y: oya, x: oxa}, co2! {y: oyb, x: oxb}),
            co2! {y: oyp, x: oxp},
        ));
    }

    let p1: i64 = machs
        .iter()
        .cloned()
        .map(|((a, b), p)| {
            let det = a.x() * b.y() - b.x() * a.y();
            let n0 = (p.x() * b.y() - p.y() * b.x()) / det;
            let n1 = (-p.x() * a.y() + p.y() * a.x()) / det;
            if a * n0 + b * n1 == p.into() {
                n0 * 3 + n1
            } else {
                0
            }
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
        .map(|((a, b), p)| {
            let det = a.x() * b.y() - b.x() * a.y();
            let n0 = (p.x() * b.y() - p.y() * b.x()) / det;
            let n1 = (-p.x() * a.y() + p.y() * a.x()) / det;
            if a * n0 + b * n1 == p.into() {
                n0 * 3 + n1
            } else {
                0
            }
        })
        .sum();

    println!("{p2}");

    Ok(())
}
