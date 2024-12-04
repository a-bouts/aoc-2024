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

        let mut from = vec![];
        let mut to = vec![];
        
        
        for line in &self.input {
            let nbrs = line.split("   ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
            from.push(nbrs[0]);
            to.push(nbrs[1]);
        }

        from.sort();
        to.sort();

        let mut res = 0;
        for i in 0..from.len() {
            res += (to[i] - from[i]).abs();
        }
        res
    }

    fn part_two(&self) -> i64 {    
        let mut from = vec![];
        let mut to = vec![];
        
        
        for line in &self.input {
            let nbrs = line.split("   ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
            from.push(nbrs[0]);
            to.push(nbrs[1]);
        }

        from.iter().map(|f| f * to.iter().filter(|t| t == &f).count() as i64).sum()
    }
        
}

crate::test!(
    11,
    "
3   4
4   3
2   5
1   3
3   9
3   3",

    0,
    ""
);
