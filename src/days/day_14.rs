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

        let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);

        const w: usize = 101;
        const h: usize = 103;

        let width = w as i64;
        let height = h as i64;

        let mut turn = 5427718;
        loop {
            if turn % 1000000 == 0 {
                println!("{turn}");
            }

            let mut ok = true;
            let mut map = [[0; w]; h];
            for r in &robots {

                let (mut x, mut y) = ((r.0 + turn * r.2) % width, (r.1 + turn * r.3) % height);
    
                if x < 0 {
                    x += width;
                }
                if y < 0 {
                    y += height;
                }

                if y == 0 && x != 50
                || y == 1 && (x < 49 || x > 51) {
                        ok = false;
                    break
                }
                // if y > 95 && (x < 40 || x > 60)
                // || x + y < 50  {
                //     ok = false;
                //     break
                // }
    
                map[y as usize][x as usize] += 1;
            }

            if ok {
                for i in 0..h {
                    println!("{}", map[i].iter().map(|n| if n == &0 { '.' } else { '#' }).collect::<String>());
                }
                break
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
