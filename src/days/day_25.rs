use anyhow::Result;

use crate::days::Day as D;

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

        fn parse(lines: &Vec<&String>) -> [usize;5] {

            let mut res = [0;5];


            for line in lines {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        res[i] += 1;
                    }
                }
            }

            res
        }

        fn overlap(lock: &[usize; 5], key: &[usize; 5]) -> bool {
            for i in 0..lock.len() {
                if lock[i] + key[i] > 7 {
                    return true
                }
            }
            false
        }
        
        let mut locks = vec![];
        let mut keys = vec![];

        let mut lines = vec![];
        for line in &self.input {
            if line.len() == 0 {
                if lines[0] == "#####" {
                    locks.push(parse(&lines));
                } else if lines[0] == "....." {
                    lines.reverse();
                    keys.push(parse(&lines));
                }

                lines.clear();
            } else {
                lines.push(line);
            }
        }

        if lines[0] == "#####" {
            locks.push(parse(&lines));
        } else if lines[0] == "....." {
            lines.reverse();
            keys.push(parse(&lines));
        }

        let mut res = 0;
        for lock in &locks {
            for key in &keys {
                if !overlap(lock, key) {
                    res += 1;
                }
            }
        }

        Ok(res)
    }

    fn part_two(&self) -> Result<i64> {    
        todo!()
    }
        
}

crate::test!(
    3,
    "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
",

    0,
    ""
);
