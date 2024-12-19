use std::sync::{Arc, Mutex};

use crate::days::Day as D;
use anyhow::Result;
use rayon::prelude::*;

pub(crate) struct Day {
    input: Vec<String>
}

impl D for Day {
    fn new(input: Vec<String>) -> Self {
        Self {
            input
        }
    }

    fn part_one(&self) -> Result<i64> {

        fn is_possible(design: &str, patterns: &Vec<&str>) -> bool {

            if design.len() == 0 {
                return true;
            }

            for pattern in patterns.iter() {
                if design.starts_with(pattern)
                && is_possible(&design[pattern.len()..], patterns) {
                    return true
                }
            }

            false
        }

        let mut res = 0;
        
        let patterns = &self.input[0].split(", ").collect::<Vec<_>>();
        let len= &self.input.len();
        for l in 2..*len {
            let design = *(&self.input[l].as_str());

            if is_possible(design, patterns) {
                res += 1;
            }
        }

        Ok(res)
    }

    fn part_two(&self) -> Result<i64> {

        fn is_possible(design: &str, patterns: &Vec<&str>) -> i64 {

            if design.len() == 0 {
                return 1;
            }

            patterns.par_iter().map(|pattern| {
                if design.starts_with(pattern) {
                    is_possible(&design[pattern.len()..], patterns)
                } else {
                    0
                }
            }).sum::<i64>()
        }

        let patterns = &self.input[0].split(", ").collect::<Vec<_>>();
        let len= &self.input.len();
        let res = (2..*len).into_par_iter().map(|l| {
            let design = *(&self.input[l].as_str());

            is_possible(design, patterns)
        }).sum::<i64>();

        Ok(res)
    }
        
}

crate::test!(
    6,
    "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",

    16,
    "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
);
