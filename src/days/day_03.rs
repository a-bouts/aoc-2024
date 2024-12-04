use regex::Regex;

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

    fn part_one(&self) -> i64 {

        let mut res = 0;

        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        
        for line in &self.input {
            for (_, [a, b]) in re.captures_iter(line).map(|c| c.extract()) {
                res += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
            }
        }

        res
    }

    fn part_two(&self) -> i64 {    
        
        let mut res = 0;

        let re = Regex::new(r"(do(n't)?\(\))|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        
        let mut d = true;
        for line in &self.input {
            re.captures_iter(line).for_each(|c| {
                match c.get(1) {
                    Some(c) if c.as_str() == "do()" => d = true,
                    Some(c) if c.as_str() == "don't()" => d = false,
                    _ if d => res += c.get(3).unwrap().as_str().parse::<i64>().unwrap() * c.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                    _ => {}
                }
            });

        }

        res
    }
        
}

crate::test!(
    161,
    "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",

    48,
    "
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
);
