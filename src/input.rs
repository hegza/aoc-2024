use std::ops;

use crate::{co2, Co2};

/// A 2D grid of characters
pub struct CharGrid(Vec<Vec<char>>);

impl CharGrid {
    pub fn from_input(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }

    /// Returns grid width
    pub fn w(&self) -> usize {
        self.0[0].len()
    }

    /// Returns grid height
    pub fn h(&self) -> usize {
        self.0.len()
    }

    /// Returns (height, width) or (rows, columns)
    pub fn dim(&self) -> (usize, usize) {
        (self.h(), self.w())
    }

    pub fn positions(&self, c: char) -> Vec<Co2<usize>> {
        let mut poss = Vec::new();
        for row in 0..self.h() {
            for col in 0..self.w() {
                if self.0[row][col] == c {
                    poss.push(co2!(row, col));
                }
            }
        }
        poss
    }
}

impl From<CharGrid> for Vec<Vec<char>> {
    fn from(value: CharGrid) -> Self {
        value.0
    }
}

impl ops::Deref for CharGrid {
    type Target = [Vec<char>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
