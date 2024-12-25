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
        
        for line in &self.input {
            todo!();
        }

        Ok(0)
    }

    fn part_two(&self) -> Result<i64> {    
        todo!()
    }
        
}

crate::test!(
    0,
    "",

    0,
    ""
);
