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

        fn compute_path(map: &Vec<Vec<char>>, from: (usize, usize), to: (usize, usize)) -> Vec<Vec<char>> {

            let dirs = HashMap::from([('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]);

            let mut ds = vec![];

            if from.0 < to.0 {
                ds.push('v');
            } else if from.0 > to.0 {
                ds.push('^');
            }
            
            if from.1 < to.1 {
                ds.push('>');
            } else if from.1 > to.1 {
                ds.push('<');
            }

            let mut res = vec![];
            for d in &ds {
                let dir = dirs.get(&d).unwrap();

                let (i, j) = (from.0 as isize, from.1 as isize);
                let (ni, nj) = ((i + dir.0) as usize, (j + dir.1) as usize);

                if (to.0 as isize - ni as isize)*(to.0 as isize - ni as isize) + (to.1 as isize - nj as isize)*(to.1 as isize - nj as isize) >= (to.0 as isize - from.0 as isize)*(to.0 as isize - from.0 as isize) + (to.1 as isize - from.1 as isize)*(to.1 as isize - from.1 as isize) 
                || map[ni][nj] == '#' {
                    continue
                }

                if ni == to.0 && nj == to.1 {
                    return vec![vec![*d, 'A']];
                }

                for r in compute_path(map, (ni, nj), to) {
                    let mut r = r.clone();
                    r.insert(0, *d);
                    res.push(r);
                }

            }

            res
        }

        fn compute_paths(map: &Vec<Vec<char>>) -> HashMap<(char, char), Vec<Vec<char>>> {

            let mut res = HashMap::new();

            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] == '#' {
                        continue
                    }
                    for k in 0..map.len() {
                        for l in 0..map[i].len() {
                            if map[k][l] == '#' {
                                continue
                            }
                            if i == k && j == l  {
                                res.insert((map[i][j], map[k][l]), vec![vec!['A']]);
                                continue
                            }

                            res.insert((map[i][j], map[k][l]), compute_path(map, (i, j), (k, l)));
                        }
                    }        
                }
            }

            res
        }

        let map = vec![
            vec!['#','#', '#', '#', '#'],
            vec!['#','7', '8', '9', '#'],
            vec!['#', '4', '5', '6', '#'],
            vec!['#', '1', '2', '3', '#'],
            vec!['#', '#', '0', 'A', '#'],
            vec!['#', '#', '#', '#', '#'],
        ];

        let mut paths = compute_paths(&map);

        let map = vec![
            vec!['#','#', '#', '#', '#'],
            vec!['#','#', '^', 'A', '#'],
            vec!['#', '<', 'v', '>', '#'],
            vec!['#', '#', '#', '#', '#'],
        ];

        for path in compute_paths(&map).into_iter() {
            paths.insert(path.0, path.1);
        }

        fn resolve(paths: &HashMap<(char, char), Vec<Vec<char>>>, code: &Vec<char>) -> Vec<Vec<char>> {
            let res: &mut Vec<Vec<char>> = &mut vec![];

            let mut prev = 'A';
            for i in 0..code.len() {
                let ps: &Vec<Vec<char>> = paths.get(&(prev, code[i])).unwrap();

                let mut new_res = vec![];
                for p in ps {
                    if res.len() == 0 {
                        new_res.push(p.clone());
                    } else {    
                        for r in res.iter() {
                            let mut r = r.clone();
                            r.append(&mut p.clone());
                            new_res.push(r);
                        }
                    }
                }
                prev = code[i];
                *res = new_res;
            }

            res.clone()
        }
        
        let mut res = 0;
        for line in &self.input {
            let code: Vec<char> = line.chars().collect::<Vec<_>>();
            let mut min = 10000;

            println!("\n{}", code.iter().collect::<String>());
            for code in resolve(&paths, &code) {
                for code in resolve(&paths, &code) {
                    for code in resolve(&paths, &code) {
                        if code.len() <= min {
                            min = code.len();
                        }
                    }
                }
            }

            println!("{min}");

            res += code[0..code.len() - 1].iter().collect::<String>().parse::<i64>().unwrap() * min as i64;
        }

        Ok(res)
    }

    fn part_two(&self) -> Result<i64> {    

        fn compute_path(map: &Vec<Vec<char>>, from: (usize, usize), to: (usize, usize)) -> Vec<Vec<char>> {

            let dirs = HashMap::from([('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]);

            let mut ds = vec![];

            if from.0 < to.0 {
                ds.push('v');
            } else if from.0 > to.0 {
                ds.push('^');
            }
            
            if from.1 < to.1 {
                ds.push('>');
            } else if from.1 > to.1 {
                ds.push('<');
            }

            let mut res = vec![];
            for d in &ds {
                let dir = dirs.get(&d).unwrap();

                let (i, j) = (from.0 as isize, from.1 as isize);
                let (ni, nj) = ((i + dir.0) as usize, (j + dir.1) as usize);

                if (to.0 as isize - ni as isize)*(to.0 as isize - ni as isize) + (to.1 as isize - nj as isize)*(to.1 as isize - nj as isize) >= (to.0 as isize - from.0 as isize)*(to.0 as isize - from.0 as isize) + (to.1 as isize - from.1 as isize)*(to.1 as isize - from.1 as isize) 
                || map[ni][nj] == '#' {
                    continue
                }

                if ni == to.0 && nj == to.1 {
                    return vec![vec![*d, 'A']];
                }

                for r in compute_path(map, (ni, nj), to) {
                    let mut r = r.clone();
                    r.insert(0, *d);
                    res.push(r);
                }

            }

            res
        }

        fn compute_paths(map: &Vec<Vec<char>>) -> HashMap<(char, char), Vec<String>> {

            let mut res = HashMap::new();

            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] == '#' {
                        continue
                    }
                    for k in 0..map.len() {
                        for l in 0..map[i].len() {
                            if map[k][l] == '#' {
                                continue
                            }
                            if i == k && j == l  {
                                res.insert((map[i][j], map[k][l]), vec!["A".to_string()]);
                                continue
                            }

                            res.insert((map[i][j], map[k][l]), compute_path(map, (i, j), (k, l)).iter().map(|r| r.iter().collect::<String>()).collect::<Vec<_>>());
                        }
                    }        
                }
            }

            res
        }

        let map = vec![
            vec!['#','#', '#', '#', '#'],
            vec!['#','7', '8', '9', '#'],
            vec!['#', '4', '5', '6', '#'],
            vec!['#', '1', '2', '3', '#'],
            vec!['#', '#', '0', 'A', '#'],
            vec!['#', '#', '#', '#', '#'],
        ];

        let mut paths = compute_paths(&map);

        let map = vec![
            vec!['#','#', '#', '#', '#'],
            vec!['#','#', '^', 'A', '#'],
            vec!['#', '<', 'v', '>', '#'],
            vec!['#', '#', '#', '#', '#'],
        ];

        for path in compute_paths(&map).into_iter() {
            paths.insert(path.0, path.1);
        }

        fn resolve(paths: &HashMap<(char, char), Vec<String>>, code: &String, n: usize, cache: &mut HashMap<usize, HashMap<String, Vec<String>>>) -> Vec<String> {

            if cache.contains_key(&n) && cache.get(&n).unwrap().contains_key(code) {
                return cache.get(&n).unwrap().get(code).unwrap().clone();
            }

            let res: &mut Vec<String> = &mut vec![];

            let codes = code.chars().collect::<Vec<_>>();

            let mut prev = 'A';
            for i in 0..code.len() {
                let ps: &Vec<String> = paths.get(&(prev, codes[i])).unwrap();

                if n > 0 {
                    let ps = ps.iter().map(|p| resolve(paths, p, n - 1, cache)).collect::<Vec<_>>();

                    let min = &mut ps.iter().min_by(|a, b| a.iter().map(|a| a.len()).sum::<usize>().cmp(&b.iter().map(|b| b.len()).sum::<usize>())).unwrap().clone();
                    res.append(min);
                } else {
                    let min = ps.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap();
                    res.push(min.clone());
                }

                prev = codes[i];
            }

            cache.entry(n).or_insert(HashMap::new()).entry(code.clone()).or_insert(res.clone());
            res.clone()
        }
        
        let cache = &mut HashMap::new();
        let mut res = 0;
        for code in &self.input {
            println!("\n{}", code);

            let codes = resolve(&paths, &code, 25, cache);

            res += code.trim_end_matches(|c| c == 'A').parse::<i64>().unwrap() * codes.iter().map(|c| c.len()).sum::<usize>() as i64;
        }

        Ok(res)
    }
        
}

crate::test!(
    126384,
    "
029A
980A
179A
456A
379A",

    126384,
    "
029A
980A
179A
456A
379A"
);
