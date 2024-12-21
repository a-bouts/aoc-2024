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
        
        let mut map = vec!();
        let mut mov: Vec<Vec<i64>> = vec!();

        for line in &self.input {
            map.push(line.chars().collect::<Vec<_>>());
            mov.push(vec![-1; line.len()]);
        }

        let change = &mut vec!();
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 'S' {
                    mov[i][j] = 0;
                    change.push((i, j));
                }
            }
        }

        while change.len() > 0 {
            let mut new_change = vec!();

            for (i, j) in change.iter() {
                let (i, j) = (*i, *j);
                for (d, di, dj) in &vec![('^', -1, 0), ('>', 0, 1), ('v', 1, 0), ('<', 0, -1)] {
                    let (ni, nj) = ((i as i64 + di) as usize, (j as i64 + dj) as usize);
                    if map[ni][nj] != '#' {
                        let score = mov[i][j];
                        if mov[ni][nj] == -1 || mov[ni][nj] > score + 1 {
                            mov[ni][nj] = score + 1;
                            new_change.push((ni, nj))
                        }
                    }
                }
            }

            *change = new_change;
        }

        let mut res = 0;
        for i in 1..map.len()-1 {
            for j in 1..map[i].len()-1 {
                if map[i][j] == '#'
                && (mov[i-1][j] >= 0 && mov[i+1][j] >= 0
                  && (mov[i-1][j] - mov[i+1][j]).abs() > 101
                || mov[i][j-1] >= 0 && mov[i][j+1] >= 0
                  && (mov[i][j-1] - mov[i][j+1]).abs() > 101) {
                    res += 1
                }
            }
        }

        for i in 0..map.len() {
            println!("{}", (0..map[i].len()).map(|j| if mov[i][j] > 0 { 'O' } else { map[i][j] }).collect::<String>());
        }
        
        Ok(res)
    }

    fn part_two(&self) -> Result<i64> {

        fn distance(map: &Vec<Vec<char>>, from: (usize, usize), to: (usize, usize)) -> i64 {

            let mut mov: Vec<Vec<i64>> = vec!();
            for i in 0..map.len() {
                mov.push(vec![-1; map[i].len()]);
            }

            mov[from.0][from.1] = 0;
            let change = &mut vec![from];
            while change.len() > 0 {
                let mut new_change = vec!();

                for (i, j) in change.iter() {
                    let (i, j) = (*i, *j);
                    for (_d, di, dj) in &vec![('^', -1, 0), ('>', 0, 1), ('v', 1, 0), ('<', 0, -1)] {
                        let (ni, nj) = ((i as i64 + di) as usize, (j as i64 + dj) as usize);
                        if ni > 0 && ni < map.len() - 1 && nj > 0 && nj < map[i].len() - 1 {
                            if mov[ni][nj] == -1 || mov[ni][nj] > mov[i][j] + 1 {
                                mov[ni][nj] = mov[i][j] + 1;
                                if (ni, nj) == to {

                                    if (from.0, from.1, ni, nj) == (2, 2, 6, 3) {
                                        for i in 0..map.len() {
                                            println!("{}", (0..map[i].len()).map(|j| if mov[i][j] >= 0 { 'O' } else { map[i][j] }).collect::<String>());
                                        }    
                                    }
                            
                                    return mov[to.0][to.1]
                                }
                                new_change.push((ni, nj))
                            }
                        }
                    }
                }

                *change = new_change;
            }

            mov[to.0][to.1]
        }

        let mut map = vec!();
        let mut mov: Vec<Vec<i64>> = vec!();

        for line in &self.input {
            map.push(line.chars().collect::<Vec<_>>());
            mov.push(vec![-1; line.len()]);
        }

        let change = &mut vec!();
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 'S' {
                    mov[i][j] = 0;
                    change.push((i, j));
                }
            }
        }

        while change.len() > 0 {
            let mut new_change = vec!();

            for (i, j) in change.iter() {
                let (i, j) = (*i, *j);
                for (d, di, dj) in &vec![('^', -1, 0), ('>', 0, 1), ('v', 1, 0), ('<', 0, -1)] {
                    let (ni, nj) = ((i as i64 + di) as usize, (j as i64 + dj) as usize);
                    if map[ni][nj] != '#' {
                        let score = mov[i][j];
                        if mov[ni][nj] == -1 || mov[ni][nj] > score + 1 {
                            mov[ni][nj] = score + 1;
                            new_change.push((ni, nj))
                        }
                    }
                }
            }

            *change = new_change;
        }


        for i in 0..map.len() {
            for j in 0..map[i].len() {
                print!("{}", match &mov[i][j] {
                    -1 => "#### ".to_string(),
                    n => {
                        format!("{:>4} ", n)
                    },
                    _ => "#### ".to_string()
                });
            }
            println!("");
        }


        let mut res = 0;

        for i in 1..map.len()-1 {
            for j in 1..map[i].len()-1 {
                for k in i..map.len()-1 {
                    for l in j..map[i].len()-1 {

                        if (i, j) == (k, l) || map[i][j] == '#' || map[k][l] == '#' {
                            continue
                        }

                        let dist = (k - i + l - j) as i64;
                        if dist > 0 && dist <= 20 {
                            if mov[k][l] - mov[i][j] - dist >= 70 {
                                println!("{:?} -> {:?} : {dist} ({})", (i, j), (k, l), (mov[k][l] - mov[i][j] - dist).abs());
                            }
                            if (mov[k][l] - mov[i][j]).abs() - dist  >= 70 {
                                //println!("{:?} -> {:?} : {dist} ({})", (i, j), (k, l), (mov[k][l] - mov[i][j]).abs());
                                res += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(res)
    }
        
}

crate::test!(
    14,
    "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",

    29+12,
    "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
);
