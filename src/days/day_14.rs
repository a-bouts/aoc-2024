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

        let mut robots = vec![];
        for line in &self.input {
            let re = Regex::new(r"p=(?<px>[0-9]*),(?<py>[0-9]*) v=(?<vx>-?[0-9]*),(?<vy>-?[0-9]*)").unwrap();
            let caps = re.captures(line).unwrap();
            robots.push((caps["px"].parse::<i64>().unwrap(), caps["py"].parse::<i64>().unwrap(), caps["vx"].parse::<i64>().unwrap(), caps["vy"].parse::<i64>().unwrap()));
        }

        let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);

        for r in robots {
            let width = 101;
            let height = 103;

            let (mut x, mut y) = ((r.0 + 100 * r.2) % width, (r.1 + 100 * r.3) % height);

            if x < 0 {
                x += width;
            }
            if y < 0 {
                y += height;
            }
            if x < width / 2 && y < height / 2 {
                a += 1;
                println!("{:?} {:?} -> {:?} GH", (r.0, r.1), (r.2, r.3), (x, y));
            } else if x < width / 2 && y > height / 2 {
                b += 1;
                println!("{:?} {:?} -> {:?} GB", (r.0, r.1), (r.2, r.3), (x, y));
            } else if x > width / 2 && y < height / 2 {
                c += 1;
                println!("{:?} {:?} -> {:?} DH", (r.0, r.1), (r.2, r.3), (x, y));
            } else if x > width / 2 && y > height / 2 {
                d += 1;
                println!("{:?} {:?} -> {:?} DB", (r.0, r.1), (r.2, r.3), (x, y));
            } else {
                println!("{:?} {:?} -> {:?} ({:?}) !!!", (r.0, r.1), (r.2, r.3), (x, y), (width / 2, height / 2));
            }
        }
        
        a * b * c * d
    }

    fn part_two(&self) -> i64 {

        let mut robots = vec![];
        for line in &self.input {
            let re = Regex::new(r"p=(?<px>[0-9]*),(?<py>[0-9]*) v=(?<vx>-?[0-9]*),(?<vy>-?[0-9]*)").unwrap();
            let caps = re.captures(line).unwrap();
            robots.push((caps["px"].parse::<i64>().unwrap(), caps["py"].parse::<i64>().unwrap(), caps["vx"].parse::<i64>().unwrap(), caps["vy"].parse::<i64>().unwrap()));
        }

        const W: usize = 101;
        const H: usize = 103;

        let width = W as i64;
        let height = H as i64;

        let mut turn = 0;
        while turn < 10403 {
            let mut map = [[0; W]; H];
            for r in &robots {

                let (mut x, mut y) = ((r.0 + turn * r.2) % width, (r.1 + turn * r.3) % height);
    
                if x < 0 {
                    x += width;
                }
                if y < 0 {
                    y += height;
                }
    
                map[y as usize][x as usize] += 1;
            }

            println!("{turn}");
            for i in 0..H {
                println!("{}", map[i].iter().map(|n| if n == &0 { '.' } else { '#' }).collect::<String>());
            }
    
            turn += 1;
        }

        turn
    }
        
}

crate::test!(
    12,
    "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",

    0,
    ""
);
