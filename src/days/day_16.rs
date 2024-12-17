use std::{collections::HashMap, fmt::Display};

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

        #[derive(Clone)]
        struct Best {
            dirs: Vec<char>,
            score: usize,
        }

        impl Best {
            fn new(dir: char, score: usize) -> Self {
                Self {
                    dirs: vec![dir], score
                }
            }

            fn to(&self, dir: &char) -> usize {
                if self.dirs.contains(dir) {
                    self.score + 1
                } else {
                    self.score + 1001
                }
            }
        }
        
        let mut map = vec!();
        let mut mov: Vec<Vec<Option<Best>>> = vec!();

        for line in &self.input {
            map.push(line.chars().collect::<Vec<_>>());
            mov.push(vec![None; line.len()]);
        }

        let change = &mut vec!();
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 'S' {
                    mov[i][j] = Some(Best::new('>', 0));
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
                        let score = mov[i][j].as_ref().unwrap().to(d);
                        if mov[ni][nj].is_none() || mov[ni][nj].as_ref().unwrap().score > score {
                            mov[ni][nj] = Some(Best::new(*d, score));
                            new_change.push((ni, nj))
                        } else if mov[ni][nj].as_ref().unwrap().score == score {
                            mov[ni][nj].as_mut().unwrap().dirs.push(*d);
                            new_change.push((ni, nj))
                        }
                    }
                }
            }

            *change = new_change;
        }

        for i in 0..map.len() {
            println!("{}", mov[i].iter().map(|m| m.as_ref().map(|s| s.dirs[0]).unwrap_or('#')).collect::<String>());
        }

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 'E' {
                    return mov[i][j].as_ref().map(|m| m.score).unwrap_or_default() as i64;
                }
            }
        }

        0
    }

    fn part_two(&self) -> i64 {

        #[derive(Clone, Debug)]
        struct Best {
            scores: [i64;4],
            is_on_path: bool,
        }

        impl Display for Best {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("^ {} > {} v {} < {}", self.scores[0], self.scores[1], self.scores[2], self.scores[3]))
            }
        }

        impl Best {
            fn new(dir: char, score: i64) -> Self {
                let mut scores = [score + 1000;4];
                scores[Self::index(&dir)] = score;

                Self {
                    scores, is_on_path: false
                }
            }

            fn score(&self, dir: &char) -> i64 {
                self.scores[Self::index(dir)]
            }

            fn index(dir: &char) -> usize {
                match dir {
                    '^' => 0,
                    '>' => 1,
                    'v' => 2,
                    '<' => 3,
                    _ => unreachable!()
                }
            }

            fn best(&self) -> Vec<char> {
                let best_score = *self.scores.iter().min().unwrap();
                vec!['^', '>', 'v', '<'].into_iter().filter(|d| self.score(d) == best_score).collect::<Vec<_>>()
            }

            fn from(&mut self, dir: &char, score: i64) -> bool {
                let dirs = ['^', '>', 'v', '<'];

                let mut changed = false;
                for d in &dirs {
                    if d == dir && self.scores[Self::index(d)] > score {
                        self.scores[Self::index(d)] = score;
                        changed = true;
                    } else if self.scores[Self::index(d)] > score + 1000 {
                        self.scores[Self::index(d)] = score + 1000;
                        changed = true;
                    }
                }

                changed
            }
        }
        
        let mut map = vec!();
        let mut mov: Vec<Vec<Option<Best>>> = vec!();

        for line in &self.input {
            map.push(line.chars().collect::<Vec<_>>());
            mov.push(vec![None; line.len()]);
        }

        let change = &mut vec!();
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 'S' {
                    mov[i][j] = Some(Best::new('>', 0));
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
                        let score = mov[i][j].as_ref().unwrap().score(d) + 1;

                        if mov[ni][nj].is_none() {
                            mov[ni][nj] = Some(Best::new(*d, score));
                            if map[ni][nj] != 'E' {
                                new_change.push((ni, nj))
                            }
                        } else if mov[ni][nj].as_mut().unwrap().from(d, score) {
                            if map[ni][nj] != 'E' {
                                new_change.push((ni, nj))
                            }
                        }
                    }
                }
            }

            *change = new_change;
        }

        let change = &mut vec!();
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 'E' {
                    for d in mov[i][j].as_ref().unwrap().best() {
                        change.push((i, j, d));
                    }
                }
            }
        }

        let mut res = 0;

        while change.len() > 0 {
            let mut new_change = vec!();

            let dirs = HashMap::from([('^', (1, 0)), ('>', (0, -1)), ('v', (-1, 0)), ('<', (0, 1))]);

            for (i, j, d) in change.iter() {
                let (i, j) = (*i, *j);
                mov[i][j].as_mut().unwrap().is_on_path = true;
                let (di, dj) = dirs.get(d).unwrap();
                let (ni, nj) = ((i as i64 + di) as usize, (j as i64 + dj) as usize);
                if map[ni][nj] != '#' {
                    if !mov[ni][nj].as_ref().unwrap().is_on_path {
                        for dd in &vec!['^', '>', 'v', '<'] {
                            if dd == d && mov[i][j].as_ref().unwrap().score(d) == mov[ni][nj].as_ref().unwrap().score(d) + 1 {
                                new_change.push((ni, nj, *dd));
                            } else if dd != d && mov[i][j].as_ref().unwrap().score(dd) == mov[ni][nj].as_ref().unwrap().score(d) + 1001 {
                                new_change.push((ni, nj, *dd));
                            }
                        }
                    }
                }
            }

            *change = new_change;
        }

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                print!("{}", match &mov[i][j] {
                    Some(best) => {
                        if best.is_on_path {
                            format!("{:>6} ", best.scores.iter().min().unwrap())
                        } else {
                            "       ".to_string()
                        }
                    },
                    _ => "###### ".to_string()
                });
            }
            println!("");
        }


        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if mov[i][j].is_some() && mov[i][j].as_ref().unwrap().is_on_path {
                    res += 1;
                }
            }
        }

        res
    }
        
}

crate::test!(
    7036,
    "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",

    64,
    "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
);
