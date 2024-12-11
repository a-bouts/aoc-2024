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

        fn nb_trail(map: &mut Vec<Vec<i64>>, (i, j): (usize, usize), height: i64) -> i64 {

            // println!("nb_trail ({i}, {j}) {height}");

            let mut res = 0;

            let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

            for (d_i, d_j) in dirs.iter() {
                let (i, j) = (i as i64 + d_i, j as i64 + d_j);
                if i >= 0 && i < map.len() as i64 && j >= 0 && j < map[0].len() as i64 {
                    let (i, j) = (i as usize, j as usize);

                    let d = match (d_i, d_j) {
                        (1, 0) => "bas",
                        (0, 1) => "droite",
                        (-1, 0) => "haut",
                        (0, -1) => "gauche",
                        _ => unreachable!()
                    };

                    if map[i][j] == height + 1 {
                        // println!("goto {d}");
                        res += if height + 1 == 9 {
                            // println!("goal");
                            map[i][j] = -1;
                            1
                        } else {
                            nb_trail(map, (i, j), height + 1)
                        };
                    }
                }
            }

            res
        }
        
        let mut map = vec!();
        for line in &self.input {
            map.push(line.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<_>>());
        }

        let mut res = 0;
        for i in 0..map.len() {
            for j in 0..map[i].len() {

                if map[i][j] == 0 {

                    let map = &mut map.clone();

                    res += nb_trail(map, (i, j), 0);

                }

            }
        }
        
        res
    }

    fn part_two(&self) -> i64 {    
        fn nb_trail(map: &mut Vec<Vec<i64>>, (i, j): (usize, usize), height: i64) -> i64 {

            // println!("nb_trail ({i}, {j}) {height}");

            let mut res = 0;

            let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

            for (d_i, d_j) in dirs.iter() {
                let (i, j) = (i as i64 + d_i, j as i64 + d_j);
                if i >= 0 && i < map.len() as i64 && j >= 0 && j < map[0].len() as i64 {
                    let (i, j) = (i as usize, j as usize);

                    let d = match (d_i, d_j) {
                        (1, 0) => "bas",
                        (0, 1) => "droite",
                        (-1, 0) => "haut",
                        (0, -1) => "gauche",
                        _ => unreachable!()
                    };

                    if map[i][j] == height + 1 {
                        // println!("goto {d}");
                        res += if height + 1 == 9 {
                            // println!("goal");
                            1
                        } else {
                            nb_trail(map, (i, j), height + 1)
                        };
                    }
                }
            }

            res
        }
        
        let mut map = vec!();
        for line in &self.input {
            map.push(line.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<_>>());
        }

        let mut res = 0;
        for i in 0..map.len() {
            for j in 0..map[i].len() {

                if map[i][j] == 0 {

                    let map = &mut map.clone();

                    res += nb_trail(map, (i, j), 0);

                }

            }
        }
        
        res
    }
        
}

crate::test!(
    36,
    "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",

    81,
    "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
);
