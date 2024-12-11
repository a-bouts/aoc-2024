use std::collections::BTreeMap;

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
        fn blink(stone: i64, stop: i64) -> i64 {
            if stop == 0 {
                return 1;
            }
            if stone == 0 {
                return blink(1, stop - 1);
            }

            let stone_str = stone.to_string();

            if stone_str.len() % 2 == 0 {
                let stones = stone_str.split_at(stone_str.len()/2);
                return blink(stones.0.parse::<i64>().unwrap(), stop - 1)
                + blink(stones.1.parse::<i64>().unwrap(), stop - 1)
            }

            return blink(stone * 2024, stop - 1);
        }
        
        let mut res = 0;
        for line in &self.input {
            let stones = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
            for stone in stones {
                res += blink(stone, 25);
            }
        }

        res
    }

    fn part_two(&self) -> i64 {    
        // 0 1 2024 - 20 24 - 2 0 2 4
        // 2 4048 - 40 48 - 4 0 4 8
        // 4 8096 - 80 96 - 8 0 9 6
        // 8 16192 - 32772608 - 3277 2608 - 32 77 26 8 -

        let mut cache = BTreeMap::new();

        fn blink(stone: i64, stop: usize, cache: &mut BTreeMap<i64, Vec<i64>>) -> i64 {

            match cache.get(&stone) {
                Some(cache) if cache[stop] > 0 => return cache[stop],
                _ => {}
            }

            if stop == 0 {
                return 1;
            }
            if stone == 0 {
                let res = blink(1, stop - 1, cache);
                cache.entry(stone).or_insert(vec![-1; 76])[stop] = res;
                return res;
            }

            let stone_str = stone.to_string();

            if stone_str.len() % 2 == 0 {
                let stones = stone_str.split_at(stone_str.len()/2);
                let res = blink(stones.0.parse::<i64>().unwrap(), stop - 1, cache)
                + blink(stones.1.parse::<i64>().unwrap(), stop - 1, cache);
                cache.entry(stone).or_insert(vec![-1; 76])[stop] = res;
                return res;
            }

            let res = blink(stone * 2024, stop - 1, cache);
            cache.entry(stone).or_insert(vec![-1; 76])[stop] = res;
            return res;
        }
        
        let mut res = 0;
        for line in &self.input {
            let stones = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
            for stone in stones {
                res += blink(stone, 75, &mut cache);
            }
        }

        res
    }
        
}

crate::test!(
    55312,
    "
125 17",

    0,
    ""
);
