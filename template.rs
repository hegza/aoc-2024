use itertools::Itertools;
use log::info;
use std::collections::*;

const INPUT: &str = include_str!("inputs/dayN.txt");

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!(env!("CARGO_BIN_NAME"));
    let mut lines = INPUT.lines();

    Ok(())
}
