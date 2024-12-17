use std::collections::HashMap;

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

        fn can_mov_to(map: &mut Vec<Vec<char>>, from: (i64, i64), dir: (i64, i64)) -> bool {
            match map[(from.0 + dir.0) as usize][(from.1 + dir.1) as usize] {
                '.' => {
                    map[(from.0 + dir.0) as usize][(from.1 + dir.1) as usize] = map[from.0 as usize][from.1 as usize];
                    map[from.0 as usize][from.1 as usize] = '.';
                    true
                },
                '#' => {
                    false
                }
                'O' => {
                    if can_mov_to(map, (from.0 + dir.0, from.1 + dir.1), dir) {
                        map[(from.0 + dir.0) as usize][(from.1 + dir.1) as usize] = map[from.0 as usize][from.1 as usize];
                        map[from.0 as usize][from.1 as usize] = '.';
                        true
                    } else {
                        false
                    }
                },
                _ => unreachable!()
            }
        }

        fn mov(map: &mut Vec<Vec<char>>, robot: (i64, i64), command: &char) -> (i64, i64) {

            let dirs = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);

            let dir = dirs[command];

            if can_mov_to(map, robot, dir) {
                (robot.0 + dir.0, robot.1 + dir.1)
            } else {
                (robot.0, robot.1)
            }
        }
        
        
        let mut map = vec!();
        let mut commands = vec!();
        let mut robot = (0, 0);

        for line in &self.input {
            if line.starts_with("#") {
                map.push(line.chars().collect::<Vec<_>>());
            } else {
                commands.push(line.chars().collect::<Vec<_>>());
            }
        }

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == '@' {
                    robot = (i as i64, j as i64);
                }
            }
        }

        for command in commands.iter().flatten() {
            robot = mov(&mut map, robot, command);
        }

        for i in 0..map.len() {
            println!("{}", map[i].iter().collect::<String>());
        }    

        let mut res = 0;
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 'O' {
                    res += 100 * i as i64 + j as i64;
                }
            }
        }

        res
    }

    fn part_two(&self) -> i64 {    
        fn mov_one(map: &mut Vec<Vec<char>>, from: (i64, i64), command: &char, mov: bool) -> bool {
            let dirs = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);
            let dir = dirs[command];

            let can = match (map[(from.0 + dir.0) as usize][(from.1 + dir.1) as usize], command) {
                ('.', _) => {
                    true
                },
                ('#', _) => {
                    false
                }
                (_, '<') | (_, '>') => {
                    mov_one(map, (from.0 + dir.0, from.1 + dir.1), command, mov)
                }
                ('[', _) => {
                    mov_two(map, (from.0 + dir.0, from.1 + dir.1), command, mov)
                }
                (']', _) => {
                    mov_two(map, (from.0 + dir.0, from.1 + dir.1 - 1), command, mov)
                }
                _ => unreachable!()
            };
            if can {
                if mov {
                    map[(from.0 + dir.0) as usize][(from.1 + dir.1) as usize] = map[from.0 as usize][from.1 as usize];
                    map[from.0 as usize][from.1 as usize] = '.';
                }
                true
            } else {
                false
            }
        }

        fn mov_two(map: &mut Vec<Vec<char>>, from: (i64, i64), command: &char, mov: bool) -> bool {

            let dirs = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);
            let dir = dirs[command];

            let can = match (map[(from.0 + dir.0) as usize][(from.1 + dir.1) as usize], map[(from.0 + dir.0) as usize][(from.1 + dir.1 + 1) as usize]) {
                ('.', '.') => {
                    true
                },
                ('#', _) | (_, '#') => {
                    false
                }
                ('[', ']') => {
                    mov_two(map, (from.0 + dir.0, from.1 + dir.1), command, mov)
                }
                (']', '.') => {
                    mov_two(map, (from.0 + dir.0, from.1 + dir.1 - 1), command, mov)
                }
                ('.', '[') => {
                    mov_two(map, (from.0 + dir.0, from.1 + dir.1 + 1), command, mov)
                }
                (']', '[') => {
                    mov_two(map, (from.0 + dir.0, from.1 + dir.1 - 1), command, mov)
                    && mov_two(map, (from.0 + dir.0, from.1 + dir.1 + 1), command, mov)
                }
                (a, b) => {
                    println!("{a}, {b}");
                    unreachable!()
                }
            };
            if can {
                if mov {
                    map[(from.0 + dir.0) as usize][(from.1 + dir.1) as usize] = map[from.0 as usize][from.1 as usize];
                    map[(from.0 + dir.0) as usize][(from.1 + dir.1) as usize + 1] = map[from.0 as usize][from.1 as usize + 1];
                    map[from.0 as usize][from.1 as usize] = '.';
                    map[from.0 as usize][from.1 as usize + 1] = '.';    
                }

                true
            } else {
                false
            }
        }


        fn mov(map: &mut Vec<Vec<char>>, robot: (i64, i64), command: &char) -> (i64, i64) {

            let dirs = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);

            let dir = dirs[command];

            if mov_one(map, robot, command, false) {
                mov_one(map, robot, command, true);
                (robot.0 + dir.0, robot.1 + dir.1)
            } else {
                (robot.0, robot.1)
            }
        }
        
        
        let mut map = vec!();
        let mut commands = vec!();
        let mut robot = (0, 0);

        for line in &self.input {
            if line.starts_with("#") {
                map.push(line.chars().map(|c| {
                    match c {
                        '#' => ['#', '#'],
                        'O' => ['[', ']'],
                        '.' => ['.', '.'],
                        '@' => ['@', '.'],
                        _ => unreachable!()
                    }
                }).flatten().collect::<Vec<_>>());
            } else {
                commands.push(line.chars().collect::<Vec<_>>());
            }
        }

        for i in 0..map.len() {
            println!("{}", map[i].iter().collect::<String>());
        }  
    
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == '@' {
                    robot = (i as i64, j as i64);
                }
            }
        }

        for command in commands.iter().flatten() {
            robot = mov(&mut map, robot, command);

            // println!("{command}");
            // for i in 0..map.len() {
            //     println!("{}", map[i].iter().collect::<String>());
            // }        
        }

        for i in 0..map.len() {
            println!("{}", map[i].iter().collect::<String>());
        }    

        let mut res = 0;
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == '[' {
                    res += 100 * i as i64 + j as i64;
                }
            }
        }

        res
    }
        
}

crate::test!(
    10092,
    "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",

    9021,
    "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
);
