use std::collections::VecDeque;

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
        
        let mut res = 0;

        for line in &self.input {
            let mut map = line.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<VecDeque<_>>();

            let mut id_front = 0_i64;
            let mut id_back = map.len() as i64 / 2;
            let mut pos = 0;

            let mut is_file = true;
            while map.len() > 0 {

                let size = map.pop_front().unwrap();

                if is_file {
                    //println!("front {size}");

                    for _ in 0..size {
                        //println!("{id_front} * {pos}");
                        res += id_front * pos;
                        pos += 1;
                    }
                    id_front += 1;

                } else {
                    //println!("back {size}");

                    let mut size_back = map.pop_back().unwrap_or_default();
                    //println!("pop back {size_back} !");
                    for _ in 0..size {
                        if size_back > 0 {
                            //println!("{id_back} * {pos}");
                            res += id_back * pos;
                            size_back -= 1;
                        }

                        if size_back == 0 {
                            size_back = map.pop_back().unwrap_or_default();
                            size_back = map.pop_back().unwrap_or_default();
                            //println!("pop back {size_back} !");
                            id_back -= 1;
                        }
                        
                        pos += 1;
                    }
                    if size_back > 0 {
                        map.push_back(size_back);
                    }
                }

                is_file = !is_file;
            }
        }

        res
    }

    fn part_two(&self) -> i64 {    

        #[derive(Debug)]
        struct Sector {
            id: Option<i64>,
            size: usize,
            start: usize,
        }
        
        let mut res = 0;

        for line in &self.input {
            let map = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>();

            let mut disk = vec![];
            let mut id: i64 = 0_i64;
            let mut pos = 0_usize;

            let mut is_file = true;
            for size in map.into_iter() {
                if is_file {
                    disk.push(Sector {id: Some(id), size, start: pos});
                    id += 1;
                } else {
                    disk.push(Sector {id: None, size, start: pos});
                }
                pos += size;
                is_file = !is_file;
            }

            for a in (0..disk.len()).rev() {
                if disk[a].id.is_some() {
                    for b in 0..a {
                        if disk[b].id.is_some() || disk[b].size < disk[a].size {
                            continue
                        }
                        let free = Sector {id: None, size: disk[b].size - disk[a].size, start: disk[b].start + disk[a].size};
                        disk[b].id = disk[a].id;
                        disk[b].size = disk[a].size;
                        disk[a].id = None;
                        if free.size > 0 {
                            disk.insert(b + 1, free);
                        }
                        break;
                    }
                }
            }

            let mut pos = 0;
            for s in disk.iter() {
                for _ in 0..s.size {
                    match s.id {
                        Some(id) => {
                            res += pos * id;
                        },
                        _ => {} 
                    }
                    pos += 1;
                }
            }
        }

        res
    }
        
}

crate::test!(
    1928,
    "
2333133121414131402",

    2858,
    "
2333133121414131402"
);
