use crate::days::Day as D;

pub(crate) struct Day {
    input: Vec<String>
}

fn is_safe(report: &Vec<i64>) -> bool {
    let mut is_safe = true;
    if report [0] < report[1] {
        for i in 1..report.len() {
            let d = report[i] - report [i - 1];
            if d <= 0 || d > 3 {
                is_safe = false;
                break
            }
        }
    } else {
        for i in 1..report.len() {
            let d = report[i - 1] - report [i];
            if d <= 0 || d > 3 {
                is_safe = false;
                break
            }
        }
    }
    is_safe
}

impl D for Day {
    fn new(input: Vec<String>) -> Self {
        Self {
            input
        }
    }

    fn part_one(&self) -> i64 {

        let mut res = 0;
        
        for line in &self.input {
            let report = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();

            let mut is_safe = true;
            if report [0] < report[1] {
                for i in 1..report.len() {
                    let d = report[i] - report [i - 1];
                    if d <= 0 || d > 3 {
                        is_safe = false;
                        break
                    }
                }
            } else {
                for i in 1..report.len() {
                    let d = report[i - 1] - report [i];
                    if d <= 0 || d > 3 {
                        is_safe = false;
                        break
                    }
                }
            }
            if is_safe {
                res += 1;
            }
        }

        res
    }

    fn part_two(&self) -> i64 {    
        let mut res = 0;
        
        for line in &self.input {
            let report: Vec<i64> = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();

            if !is_safe(&report) {
                for i in 0..report.len() {
                    let mut report = report.clone();
                    report.remove(i);
                    if is_safe(&report) {
                        res += 1;
                        break
                    }
                }
            } else {
                res += 1;
            }

        }

        res
    }
        
}

crate::test!(
    2,
    "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",

    4,
    "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
);
