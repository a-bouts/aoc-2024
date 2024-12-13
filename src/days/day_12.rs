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

        fn regionize(map: &mut Vec<Vec<char>>, (i, j): (usize, usize)) {
            let change = &mut vec![(i, j)];
            let c = map[i][j];
            map[i][j] = '#';

            println!("regionize {c}, ({i}, {j})");

            while change.len() > 0 {
                let mut new_change = vec![];

                for (i, j) in change.iter() {
                    let (i, j) = (*i, *j);
                    if i > 0 && map[i-1][j] == c {
                        map[i-1][j] = '#';
                        new_change.push((i-1, j));
                    }
                    if j > 0 && map[i][j-1] == c {
                        map[i][j-1] = '#';
                        new_change.push((i, j-1));
                    }
                    if i < map.len()-1 && map[i+1][j] == c {
                        map[i+1][j] = '#';
                        new_change.push((i+1, j));
                    }
                    if j < map[i].len()-1 && map[i][j+1] == c {
                        map[i][j+1] = '#';
                        new_change.push((i, j+1));
                    }
                }

                *change = new_change;
            }
        }

        fn area(map: &Vec<Vec<char>>) -> i64 {
            let mut res = 0;
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] == '#' {
                        res += 1;
                    }
                }
            }
            res
        }

        fn perimeter(map: &Vec<Vec<char>>) -> i64 {
            let mut res = 0;

            for i in 0..map.len() {
                let mut is_plot = false;
                let mut nb_switch = 0;
                for j in 0..map[i].len() {
                    if !is_plot && map[i][j] == '#'
                    || is_plot && map[i][j] != '#' {
                        nb_switch += 1;
                        is_plot = !is_plot;
                    }
                    if j == map[i].len() - 1 && map[i][j] == '#' {
                        nb_switch += 1;
                    }
                }
                res += nb_switch;
            }

            for j in 0..map[0].len() {
                let mut is_plot = false;
                let mut nb_switch = 0;
                for i in 0..map.len() {
                    if !is_plot && map[i][j] == '#'
                    || is_plot && map[i][j] != '#' {
                        nb_switch += 1;
                        is_plot = !is_plot;
                    }
                    if i == map.len() - 1 && map[i][j] == '#' {
                        nb_switch += 1;
                    }
                }
                res += nb_switch;
            }
            res
        }

        fn clean(map: &mut Vec<Vec<char>>) {
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] == '#' {
                        map[i][j] = '.';
                    }
                }
            }
        }
        
        let mut map = vec!();
        for line in &self.input {
            map.push(line.chars().collect::<Vec<_>>());
        }

        let mut res = 0;

        let mut nb_plots = (map.len() * map[0].len()) as i64;

        let map = &mut map;
        while nb_plots > 0 {
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] != '.' {
                        regionize(map, (i,j));

                        let a = area(&map);
                        let p = perimeter(&map);

                        res += a * p;
                        nb_plots -= a;

                        println!("{a} * {p}");
                        //for l in map.iter() {
                        //    println!("{}", l.iter().collect::<String>());
                        //}

                        clean(map);
                        
                    }
                }
            }
        }

        res
    }

    fn part_two(&self) -> i64 {    

        fn regionize(map: &mut Vec<Vec<char>>, (i, j): (usize, usize)) {
            let change = &mut vec![(i, j)];
            let c = map[i][j];
            map[i][j] = '#';

            println!("regionize {c}, ({i}, {j})");

            while change.len() > 0 {
                let mut new_change = vec![];

                for (i, j) in change.iter() {
                    let (i, j) = (*i, *j);
                    if i > 0 && map[i-1][j] == c {
                        map[i-1][j] = '#';
                        new_change.push((i-1, j));
                    }
                    if j > 0 && map[i][j-1] == c {
                        map[i][j-1] = '#';
                        new_change.push((i, j-1));
                    }
                    if i < map.len()-1 && map[i+1][j] == c {
                        map[i+1][j] = '#';
                        new_change.push((i+1, j));
                    }
                    if j < map[i].len()-1 && map[i][j+1] == c {
                        map[i][j+1] = '#';
                        new_change.push((i, j+1));
                    }
                }

                *change = new_change;
            }
        }

        fn area(map: &Vec<Vec<char>>) -> i64 {
            let mut res = 0;
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] == '#' {
                        res += 1;
                    }
                }
            }
            res
        }

        fn perimeter(map: &Vec<Vec<char>>) -> i64 {
            let mut res = 0;

            for i in 0..map.len() {
                let mut is_plot = false;
                let mut nb_switch = 0;
                for j in 0..map[i].len() {
                    if !is_plot && map[i][j] == '#' {
                        if i == 0 || map[i-1][j] != '#' || j > 0 && map[i-1][j-1] == '#' {
                            println!("1 - border : ({i}, {j}) : {is_plot}");
                            nb_switch += 1;
                        }
                        is_plot = !is_plot;
                    } else if is_plot && map[i][j] != '#' {
                        if i == 0 || map[i-1][j] == '#' || j > 0 && map[i-1][j-1] != '#' {
                            println!("2 - border : ({i}, {j}) : {is_plot}");
                            nb_switch += 1;
                        }
                        is_plot = !is_plot;
                    }
                    if j == map[i].len() - 1 && map[i][j] == '#' {
                        if i == 0 || map[i-1][j] != '#' {
                            println!("3 - border : ({i}, {j}) : {is_plot}");
                            nb_switch += 1;
                        }
                    }
                }
                res += nb_switch;
            }

            for j in 0..map[0].len() {
                let mut is_plot = false;
                let mut nb_switch = 0;
                for i in 0..map.len() {
                    if !is_plot && map[i][j] == '#' {
                        if j == 0 || map[i][j-1] != '#' || i > 0 && map[i-1][j-1] == '#' {
                            println!("4 - border : ({i}, {j}) : {is_plot}");
                            nb_switch += 1;
                        }
                        is_plot = !is_plot;
                    } else if is_plot && map[i][j] != '#' {
                        if j == 0 || map[i][j-1] == '#' || i > 0 && map[i-1][j-1] != '#' {
                            println!("5 - border : ({i}, {j}) : {is_plot}");
                            nb_switch += 1;
                        }
                        is_plot = !is_plot;
                    }
                    if i == map.len() - 1 && map[i][j] == '#' {
                        if j == 0 || map[i][j-1] != '#' {
                            println!("6 - border : ({i}, {j}) : {is_plot}");
                            nb_switch += 1;
                        }
                    }
                }
                res += nb_switch;
            }
            res
        }

        fn clean(map: &mut Vec<Vec<char>>) {
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] == '#' {
                        map[i][j] = '.';
                    }
                }
            }
        }
        
        let mut map = vec!();
        for line in &self.input {
            map.push(line.chars().collect::<Vec<_>>());
        }

        let mut res = 0;

        let mut nb_plots = (map.len() * map[0].len()) as i64;

        let map = &mut map;
        while nb_plots > 0 {
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] != '.' {
                        regionize(map, (i,j));

                        let a = area(&map);
                        let p = perimeter(&map);

                        res += a * p;
                        nb_plots -= a;

                        println!("{a} * {p}");
                        //for l in map.iter() {
                        //   println!("{}", l.iter().collect::<String>());
                        //}

                        clean(map);
                        
                    }
                }
            }
        }

        res

    }
        
}

crate::test!(
    1930,
    "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",

    1206,
    "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
);
