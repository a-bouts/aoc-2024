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

        fn resolve(paths: &HashMap<(char, char), Vec<String>>, code: &String, n: usize, cache: &mut Vec<HashMap<String, usize>>, is_in_cache: &mut i64, is_not_in_cache: &mut i64) -> usize {

            if cache[n].contains_key(code) {
                *is_in_cache += 1;
                return *cache[n].get(code).unwrap();
            }
            *is_not_in_cache += 1;

            let mut res = 0;

            let codes = code.chars().collect::<Vec<_>>();

            let mut prev = 'A';
            for i in 0..code.len() {
                let ps: &Vec<String> = paths.get(&(prev, codes[i])).unwrap();

                if n > 0 {
                    let mut min = -1;
                    let mut min_p = "";
                    for p in ps {
                        let len = resolve(paths, p, n - 1, cache, is_in_cache, is_not_in_cache) as isize;
                        if min < 0 || len < min {
                            min = len;
                            min_p = p;
                        }
                    }
                    
                    //let ps = ps.iter().map(|p| resolve(paths, p, n - 1, cache, is_in_cache, is_not_in_cache)).collect::<Vec<_>>();

                    //let min = &mut ps.iter().min_by(|a, b| a.iter().map(|a| a.len()).sum::<usize>().cmp(&b.iter().map(|b| b.len()).sum::<usize>())).unwrap().clone();
                    res += min as usize;
                } else {
                    let min = ps.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap();
                    res += min.len();
                }

                prev = codes[i];
            }

            cache[n].entry(code.clone()).or_insert(res.clone());

            println!("\n");
            println!("{is_in_cache} {is_not_in_cache}");
            for i in 0..cache.len() {
                println!("{i} : {}", cache[i].len());
            }

            res
        }
        
        let cache = &mut vec![];
        for _ in 0..26 {
            cache.push(HashMap::new());
        }
        let is_in_cache = &mut 0;
        let is_not_in_cache = &mut 0;

        let mut res = 0;
        for code in &self.input {
            println!("\n{}", code);

            let len = resolve(&paths, &code, 25, cache, is_in_cache, is_not_in_cache);

            res += code.trim_end_matches(|c| c == 'A').parse::<i64>().unwrap() * len as i64;
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
