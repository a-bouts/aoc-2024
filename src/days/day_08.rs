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
        
        let mut map: Vec<Vec<char>> = vec!();
        let mut antinodes = vec!();
        for line in &self.input {
            map.push(line.chars().collect::<Vec<_>>());
            antinodes.push(vec!['.'; line.len()]);
        }

        let mut antennas = HashMap::new();
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] != '.' {
                    antennas.entry(map[i][j]).or_insert(vec!()).push(Point {i, j});
                }
            }
        }

        for frequency in antennas.keys() {

            for a1 in antennas.get(frequency).unwrap() {
                for a2 in antennas.get(frequency).unwrap() {
                    if a1 == a2 {
                        continue
                    }

                    match a1.resonnate(a2, (map.len(), map[0].len())) {
                        Some(r) => antinodes[r.i][r.j] = '#',
                        _ => {}
                    }

                    match a2.resonnate(a1, (map.len(), map[0].len())) {
                        Some(r) => antinodes[r.i][r.j] = '#',
                        _ => {}
                    }
                }
            }

        }


        println!();
        for i in 0..map.len() {
            println!("{} {}", antinodes[i].iter().collect::<String>(), map[i].iter().collect::<String>());
        }    

        antinodes.iter().map(|l| l.iter().filter(|c| c == &&'#').count() as i64).sum()
    }

    fn part_two(&self) -> i64 {    
        let mut map: Vec<Vec<char>> = vec!();
        let mut antinodes = vec!();
        for line in &self.input {
            map.push(line.chars().collect::<Vec<_>>());
            antinodes.push(vec!['.'; line.len()]);
        }

        let mut antennas = HashMap::new();
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] != '.' {
                    antennas.entry(map[i][j]).or_insert(vec!()).push(Point {i, j});
                }
            }
        }

        for frequency in antennas.keys() {

            for a1 in antennas.get(frequency).unwrap() {
                for a2 in antennas.get(frequency).unwrap() {
                    if a1 == a2 {
                        continue
                    }

                    for r in a1.resonnate_all(a2, (map.len(), map[0].len())) {
                        antinodes[r.i][r.j] = '#'
                    }

                    for r in a2.resonnate_all(a1, (map.len(), map[0].len())) {
                        antinodes[r.i][r.j] = '#'
                    }
                }
            }

        }


        println!();
        for i in 0..map.len() {
            println!("{} {}", antinodes[i].iter().collect::<String>(), map[i].iter().collect::<String>());
        }    

        antinodes.iter().map(|l| l.iter().filter(|c| c == &&'#').count() as i64).sum()
    }

}

#[derive(PartialEq)]
struct Point {
    i: usize,
    j: usize,
}

impl Point {
    fn resonnate(&self, other: &Point, (height, width): (usize, usize)) -> Option<Self> {

        let di = other.i as i64 - self.i as i64;
        let dj = other.j as i64 - self.j as i64;

        if other.i as i64 + di < 0 || other.i as i64 + di >= height as i64
        || other.j as i64 + dj < 0 || other.j as i64 + dj >= width as i64 {
            None
        } else {
            Some(Point {i: (other.i as i64 + di) as usize, j: (other.j as i64 + dj) as usize})
        }
    }

    fn resonnate_all(&self, other: &Point, (height, width): (usize, usize)) -> Vec<Self> {

        let di = other.i as i64 - self.i as i64;
        let dj = other.j as i64 - self.j as i64;

        let mut res = vec!();
        let mut new_i = other.i as i64;
        let mut new_j = other.j as i64;
        while new_i >= 0 && new_i < height as i64
        && new_j >= 0 && new_j < width as i64 {
            res.push(Point{i: new_i as usize, j: new_j as usize});
            new_i += di;
            new_j += dj;
        }

        res
    }

}

crate::test!(
    14,
    "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",

    34,
    "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
);
