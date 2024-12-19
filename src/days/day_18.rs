use crate::days::Day as D;

use anyhow::Result;

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

        let size: usize = 71;
        
        let mut map = vec![];
        for _ in 0..size {
            map.push(vec![-1;size]);
        }
        let mut path = vec![];
        for _ in 0..size {
            path.push(vec![-1;size]);
        }

        let mut cpt = 0;

        for line in &self.input {
            let bytes = line.split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let(x, y) = (bytes[0], bytes[1]);
            map[y][x] = cpt;


            cpt += 1;
            if cpt == 1024 {
                break;
            }
        }

        path[0][0] = 0;
        let change = &mut vec![(0_usize, 0_usize)];
        while change.len() > 0 {
            let mut new_change = vec![];
            for (x, y) in change.iter() {
                let (x, y) = (*x, *y);
                for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0,1)] {
                    let (nx,ny) = (x as isize + dx, y as isize + dy);

                    if ny < 0 || ny as usize >= map.len() || nx < 0 || nx as usize >= map[ny as usize].len() {
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);

                    if map[ny][nx] < 0
                    && (path[ny][nx] < 0 || path[ny][nx] > path[y][x] + 1) {
                        path[ny][nx] = path[y][x] + 1;
                        new_change.push((nx, ny));
                    }
                }
            }

            *change = new_change;
        }


        Ok(path[path.len() - 1][path[path.len() - 1].len() - 1])
    }

    fn part_two(&self) -> Result<i64> {    

        let size: usize = 71;

        let mut limit = 1024;
        loop {

            println!("{limit}");

            let mut map = vec![];
            for _ in 0..size {
                map.push(vec![-1;size]);
            }
            let mut path = vec![];
            for _ in 0..size {
                path.push(vec![-1;size]);
            }
    
            let mut cpt: i32 = 0;

            for line in &self.input {
                let bytes = line.split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();
                let(x, y) = (bytes[0], bytes[1]);
                map[y][x] = cpt;
    
    
                cpt += 1;
                if cpt == limit {
                    break;
                }
            }
    
            path[0][0] = 0;
            let change = &mut vec![(0_usize, 0_usize)];
            while change.len() > 0 {
                let mut new_change = vec![];
                for (x, y) in change.iter() {
                    let (x, y) = (*x, *y);
                    for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0,1)] {
                        let (nx,ny) = (x as isize + dx, y as isize + dy);
    
                        if ny < 0 || ny as usize >= map.len() || nx < 0 || nx as usize >= map[ny as usize].len() {
                            continue;
                        }
                        let (nx, ny) = (nx as usize, ny as usize);
    
                        if map[ny][nx] < 0
                        && (path[ny][nx] < 0 || path[ny][nx] > path[y][x] + 1) {
                            path[ny][nx] = path[y][x] + 1;
                            new_change.push((nx, ny));
                        }
                    }
                }
    
                *change = new_change;
            }
    
            if path[path.len() - 1][path[path.len() - 1].len() - 1] == -1 {
                println!("{}", &self.input[limit as usize - 1]);
                break
            }

            limit += 1;
        }

        Ok(0)
    }
        
}

crate::test!(
    0,
    "",

    0,
    ""
);
