use std::collections::HashMap;

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
        
        fn next(secret: i64) -> i64 {
            let secret = ((secret * 64) ^ secret) % 16777216;
            let secret = ((secret / 32) ^ secret) % 16777216;
            let secret = ((secret * 2048) ^ secret) % 16777216;

            secret
        }


        let mut res = 0;
        for line in &self.input {
            let mut secret = line.parse::<i64>().unwrap();

            for _ in 0..2000 {
                secret = next(secret);
            }

            res += secret;
        }

        Ok(res)
    }

    fn part_two(&self) -> Result<i64> {    

        fn next(secret: i64) -> i64 {
            let secret = ((secret * 64) ^ secret) % 16777216;
            let secret = ((secret / 32) ^ secret) % 16777216;
            let secret = ((secret * 2048) ^ secret) % 16777216;

            secret
        }

        let mut scores = HashMap::new();
        
        let mut res = 0;
        for line in &self.input {
            println!("{line}");
            let mut secret = line.parse::<i64>().unwrap();
            let mut price = secret % 10;

            let mut score = HashMap::new();

            let mut a = 0;
            let mut b = 0;
            let mut c = 0;
            let mut d = 0;
            for i in 0..2000 {
                secret = next(secret);
                let new_price = secret % 10;
                a = b;
                b = c;
                c = d;
                d = new_price - price;
                price = new_price;

                if i >=3 && !score.contains_key(&(a, b, c, d)) {
                    score.insert((a, b, c, d), price);
                }
            }

            for (seq, price) in score {
                *scores.entry(seq).or_insert(0) += price;
            }
        }

        res = *scores.values().max().unwrap();

        Ok(res)

    }
        
}

crate::test!(
    37327623,
    "
1
10
100
2024",

    23,
    "
1
2
3
2024"
);
