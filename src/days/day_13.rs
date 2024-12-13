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
        let mut res: i64 = 0;

        let mut l = 0;
        while l < self.input.len() {
            let a = &self.input[l];
            let b = &self.input[l+1];
            let p = &self.input[l+2];
            
            let re = Regex::new(r"Button [AB]: X\+(?<x>[0-9]*), Y\+(?<y>[0-9]*)").unwrap();
            let caps = re.captures(a).unwrap();
            let a = (caps["x"].parse::<i64>().unwrap(), caps["y"].parse::<i64>().unwrap());
            let caps = re.captures(b).unwrap();
            let b = (caps["x"].parse::<i64>().unwrap(), caps["y"].parse::<i64>().unwrap());
            let re = Regex::new(r"Prize: X=(?<x>[0-9]*), Y=(?<y>[0-9]*)").unwrap();
            let caps = re.captures(p).unwrap();
            let p = (caps["x"].parse::<i64>().unwrap(), caps["y"].parse::<i64>().unwrap());

            l += 4;

            //u*a.0 + v*b.0 = p.0 (*a.1)
            //u*a.1 + v*b.1 = p.1 (*a.0)

            let v = (p.1 * a.0 - p.0 * a.1) / (b.1 * a.0 - b.0 * a.1);
            let u = (p.1 * b.0 - p.0 * b.1) / (a.1 * b.0 - a.0 * b.1);

            if u*a.0+v*b.0 == p.0 && u*a.1+v*b.1 == p.1 {
                res += 3 * u + v;
            }
        }
       
        res
    }

    fn part_two(&self) -> i64 {    
        let mut res: i64 = 0;

        let mut l = 0;
        while l < self.input.len() {
            let a = &self.input[l];
            let b = &self.input[l+1];
            let p = &self.input[l+2];
            
            let re = Regex::new(r"Button [AB]: X\+(?<x>[0-9]*), Y\+(?<y>[0-9]*)").unwrap();
            let caps = re.captures(a).unwrap();
            let a = (caps["x"].parse::<i64>().unwrap(), caps["y"].parse::<i64>().unwrap());
            let caps = re.captures(b).unwrap();
            let b = (caps["x"].parse::<i64>().unwrap(), caps["y"].parse::<i64>().unwrap());
            let re = Regex::new(r"Prize: X=(?<x>[0-9]*), Y=(?<y>[0-9]*)").unwrap();
            let caps = re.captures(p).unwrap();
            let p = (caps["x"].parse::<i64>().unwrap() + 10000000000000, caps["y"].parse::<i64>().unwrap() + 10000000000000);

            l += 4;

            //u*a.0 + v*b.0 = p.0 (*a.1)
            //u*a.1 + v*b.1 = p.1 (*a.0)

            let v = (p.1 * a.0 - p.0 * a.1) / (b.1 * a.0 - b.0 * a.1);
            let u = (p.1 * b.0 - p.0 * b.1) / (a.1 * b.0 - a.0 * b.1);

            if u*a.0+v*b.0 == p.0 && u*a.1+v*b.1 == p.1 {
                res += 3 * u + v;
            }
        }
       
        res
    }
        
}

crate::test!(
    480,
    "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",

    0,
    ""
);
