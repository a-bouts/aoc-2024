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
            let line = line.split(": ").collect::<Vec<_>>();
            let test = line[0].parse::<f64>().unwrap();
            let numbers = line[1].split(" ").map(|v| v.parse::<f64>().unwrap()).collect::<Vec<_>>();

            println!("");
            if is_valid(test, numbers) {
                res += test as i64;
            }
        }

        res
    }

    fn part_two(&self) -> i64 {    

        let mut res = 0;

        for line in &self.input {
            let line = line.split(": ").collect::<Vec<_>>();
            let test = line[0].parse::<f64>().unwrap();
            let numbers = line[1].split(" ").map(|v| v.parse::<f64>().unwrap()).collect::<Vec<_>>();

            println!("");
            if is_valid_2(test, numbers) {
                res += test as i64;
            }
        }

        res
    }
        
}

fn is_valid(test: f64, numbers: Vec<f64>) -> bool {

    println!("test {test} {:?}", numbers);

    if test == 0.0 && numbers.len() == 0 {
        return true
    }
    if test == 0.0 || test.fract() != 0.0 || numbers.len() == 0 {
        return false
    }

    println!("try -");
    if is_valid(test - numbers[numbers.len()-1], numbers[0..numbers.len()-1].to_vec()) {
        return true
    }

    println!("try /");
    if is_valid(test / numbers[numbers.len()-1], numbers[0..numbers.len()-1].to_vec()) {
        return true
    }

    false
}

fn is_valid_2(test: f64, numbers: Vec<f64>) -> bool {

    println!("test {test} {:?}", numbers);

    if test == 0.0 && numbers.len() == 0 {
        return true
    }
    if test == 0.0 || test.fract() != 0.0 || numbers.len() == 0 {
        return false
    }

    println!("try -");
    if is_valid_2(test - numbers[numbers.len()-1], numbers[0..numbers.len()-1].to_vec()) {
        return true
    }

    println!("try /");
    if is_valid_2(test / numbers[numbers.len()-1], numbers[0..numbers.len()-1].to_vec()) {
        return true
    }

    println!("try ||");
    if test.to_string().ends_with(&numbers[numbers.len()-1].to_string()) {
        let mut test = test.to_string();
        test.truncate(test.to_string().len() - numbers[numbers.len()-1].to_string().len());
        match test.parse::<f64>() {
            Ok(test) => {
                if is_valid_2(test, numbers[0..numbers.len()-1].to_vec()) {
                    return true
                }
            }
            Err(_) => {},
        }
    }

    false
}

crate::test!(
    3749,
    "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",

    11387,
    "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
);
