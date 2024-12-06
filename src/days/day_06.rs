use crate::days::Day as D;
use std::collections::HashMap;

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
        let mut map = vec![];
        for line in &self.input {
            map.push(line.chars().collect::<Vec<_>>());
        }

        let dirs = HashMap::from([
            ('^', (-1, 0)),
            ('>', (0, 1)),
            ('<', (0, -1)),
            ('v', (1, 0))
        ]);

        let mut guard: (i64, i64) = (0, 0);

        for i in 0..map.len() {
            for j in 0..map.len() {
                if map[i][j] != '.' && map[i][j] != '#' {
                    guard = (i as i64, j as i64);
                    break
                }
            }
        }

        loop {

            if guard.0 < 0 || guard.0 >= map.len() as i64
            || guard.1 < 0 || guard.1 >= map[0].len() as i64 {
                break
            }

            let dir = dirs.get(&map[guard.0 as usize][guard.1 as usize]).unwrap();
            
            let next = (guard.0 + dir.0, guard.1 + dir.1);

            if next.0 >= 0 && next.0 < map.len() as i64
            && next.1 >= 0 && next.1 < map[0].len() as i64 {
                if map[next.0 as usize][next.1 as usize] == '#' {
                    map[guard.0 as usize][guard.1 as usize] = match map[guard.0 as usize][guard.1 as usize] {
                        '^' => '>',
                        '>' => 'v',
                        'v' => '<',
                        '<' => '^',
                        c => c,
                    };
                    continue
                }
                map[next.0 as usize][next.1 as usize] = map[guard.0 as usize][guard.1 as usize];
            }
            
            map[guard.0 as usize][guard.1 as usize] = 'x';
            guard = next;
        }

        map.iter().map(|l| l.iter().filter(|c| c == &&'x').count() as i64).sum()
        
    }

    fn part_two(&self) -> i64 {
        let dirs: HashMap<char, (i64, i64, u8)> = HashMap::from([
            ('^', (-1, 0, 2)),
            ('>', (0, 1, 4)),
            ('v', (1, 0, 8)),
            ('<', (0, -1, 16)),
        ]);
        
        let mut dir = dirs.get(&'^').unwrap();
        let mut guard: (i64, i64) = (0, 0);

        let mut map = vec![];
        for (i, line) in self.input.iter().enumerate() {
            map.push(line.chars().enumerate().map(|(j, c)| {
                match c {
                    '#' => 64,
                    '.' => 0,
                    c => {
                        dir = dirs.get(&c).unwrap();
                        guard = (i as i64, j as i64);
                        0
                    }
                }
            }).collect::<Vec<_>>());
        }


        let mut res = 0;

    
        for i in 0..map.len() {
            for j in 0..map[i].len() {

                if map[i][j] == 64 {
                   continue
                }

                let mut map = map.clone();
                let mut dir = &dir.clone();
                let mut guard = guard.clone();
        
                map[i][j] = 64;


                let mut obstructed: bool = false;
                loop {

                    if guard.0 < 0 || guard.0 >= map.len() as i64
                    || guard.1 < 0 || guard.1 >= map[0].len() as i64 {
                        break
                    }
                    
                    let next = (guard.0 + dir.0, guard.1 + dir.1);
        
                    if next.0 >= 0 && next.0 < map.len() as i64
                    && next.1 >= 0 && next.1 < map[0].len() as i64 {
                        if map[next.0 as usize][next.1 as usize] == 64 {
                            dir = match dir.2 {
                                2 => dirs.get(&'>').unwrap(),
                                4 => dirs.get(&'v').unwrap(),
                                8 => dirs.get(&'<').unwrap(),
                                16 => dirs.get(&'^').unwrap(),
                                _ => unreachable!(),
                            };
                            continue
                        }
                    }
        
                    if map[guard.0 as usize][guard.1 as usize] & dir.2 == dir.2 {
                        obstructed = true;
                        break
                    }
                    
                    map[guard.0 as usize][guard.1 as usize] |= dir.2;
                    guard = next;
                }
        
                if obstructed {
                    res += 1;
                }    
    
            }
        }

        res
    }
        
}

crate::test!(
    41,
    "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",

    6,
    "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
);
