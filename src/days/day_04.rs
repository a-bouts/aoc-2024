use crate::days::Day as D;

pub(crate) struct Day {
    input: Vec<String>
}

fn search_for(search: Vec<char>, puzzle: &Vec<Vec<char>>, i: i64, j: i64, dir: &(i64, i64)) -> bool {

    if search.len() == 0 {
        return true
    }

    if i < 0 || i >= puzzle.len() as i64 || j < 0 || j >= puzzle[0].len() as i64 {
        return false
    }

    if puzzle[i as usize][j as usize] == search[0] {
        return search_for(search[1..].to_vec(), puzzle, i + dir.0, j + dir.1, dir)
    }
    
    false
}

impl D for Day {
    fn new(input: Vec<String>) -> Self {
        Self {
            input
        }
    }

    fn part_one(&self) -> i64 {
        
        let mut puzzle: Vec<Vec<char>> = vec![];
        for line in &self.input {
            puzzle.push(line.chars().collect::<Vec<char>>());
        }

        let mut res = 0;

        let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

        for i in 0..puzzle.len() {
            for j in 0..puzzle.len() {
                for dir in &dirs {
                    if search_for("XMAS".chars().collect(), &puzzle, i as i64, j as i64, dir) {
                        res += 1;
                    }
                }
                
            }
        }

        res
    }

    fn part_two(&self) -> i64 {    

        let mut puzzle: Vec<Vec<char>> = vec![];
        for line in &self.input {
            puzzle.push(line.chars().collect::<Vec<char>>());
        }

        let mut res = 0;

        for i in 1..puzzle.len() - 1 {
            for j in 1..puzzle.len() - 1 {

                if puzzle[i][j] != 'A' {
                    continue
                }

                if (puzzle[i-1][j-1] == 'M' && puzzle[i+1][j+1] == 'S' || puzzle[i-1][j-1] == 'S' && puzzle[i+1][j+1] == 'M')
                && (puzzle[i-1][j+1] == 'M' && puzzle[i+1][j-1] == 'S' || puzzle[i-1][j+1] == 'S' && puzzle[i+1][j-1] == 'M') {
                    res += 1;
                }
                
            }
        }

        res
    }
        
}

crate::test!(
    18,
    "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",

    9,
    "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
);
