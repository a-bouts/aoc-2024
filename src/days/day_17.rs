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
        
        let mut a = self.input[0].split(" ").collect::<Vec<_>>()[2].parse::<i64>().unwrap();
        let mut b = self.input[1].split(" ").collect::<Vec<_>>()[2].parse::<i64>().unwrap();
        let mut c = self.input[2].split(" ").collect::<Vec<_>>()[2].parse::<i64>().unwrap();

        let program = self.input[4].split(" ").collect::<Vec<_>>()[1].split(",").map(|c| c.parse::<i64>().unwrap()).collect::<Vec<_>>();

        let mut ptr = 0;

        let mut res: Vec<i64> = vec![];

        while ptr < program.len() {

            println!("");
            println!("ptr: {ptr}");
            println!("a: {a}");
            println!("b: {b}");
            println!("c: {c}");
            println!("{}", res.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));

            let instruction = program[ptr];
            let operand = program[ptr + 1];
            let combo = match operand {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => a,
                5 => b,
                6 => c,
                _ => -1,
            };

            println!("instruction {instruction} literal {operand} combo {combo}");

            match instruction {
                0 => {
                    println!("a = {a} / 2_i64.pow({combo} as u32)");
                    a = a / 2_i64.pow(combo as u32);
                }
                1 => {
                    println!("b = {b} ^ {operand}");
                    b = b ^ operand;
                }
                2 => {
                    println!("b = {combo} % 8");
                    b = combo % 8;
                }
                3 => {
                    if a != 0 {
                        println!("ptr = {operand}");
                        ptr = operand as usize;
                        continue;
                    }
                }
                4 => {
                    println!("b = {b} ^ {c}");
                    b = b ^ c;
                }
                5 => {
                    println!("output {combo} % 8");
                    res.push(combo % 8);
                }
                6 => {
                    println!("b = {a} / 2_i64.pow({combo} as u32)");
                    b = a / 2_i64.pow(combo as u32);
                }
                7 => {
                    println!("c = {a} / 2_i64.pow({combo} as u32)");
                    c = a / 2_i64.pow(combo as u32);
                }
                _ => unreachable!()
            }

            ptr += 2;
        }

        println!("");
        println!("ptr: {ptr}");
        println!("a: {a}");
        println!("b: {b}");
        println!("c: {c}");
        println!("{}", res.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));

        0
    }

    fn part_two(&self) -> i64 {    
        let mut a = self.input[0].split(" ").collect::<Vec<_>>()[2].parse::<i64>().unwrap();
        let mut b = self.input[1].split(" ").collect::<Vec<_>>()[2].parse::<i64>().unwrap();
        let mut c = self.input[2].split(" ").collect::<Vec<_>>()[2].parse::<i64>().unwrap();

        let program = self.input[4].split(" ").collect::<Vec<_>>()[1].split(",").map(|c| c.parse::<i64>().unwrap()).collect::<Vec<_>>();


        let mut a_init = 0;
        loop {
            if a_init % 1000000 == 0 {
                println!("{a_init}");
            }

            a = a_init;

            let mut ptr = 0;
            let mut res: Vec<i64> = vec![];

            let mut ok = true;
            while ptr < program.len() {

                let instruction = program[ptr];
                let operand = program[ptr + 1];
                let combo = match operand {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => a,
                    5 => b,
                    6 => c,
                    _ => -1,
                };
        
                match instruction {
                    0 => {
                        a = a / 2_i64.pow(combo as u32);
                    }
                    1 => {
                        b = b ^ operand;
                    }
                    2 => {
                        b = combo % 8;
                    }
                    3 => {
                        if a != 0 {
                            ptr = operand as usize;
                            continue;
                        }
                    }
                    4 => {
                        b = b ^ c;
                    }
                    5 => {
                        res.push(combo % 8);
                        if res.len() > program.len() || res[res.len() - 1] != program[res.len() - 1] {
                            ok = false;
                            break;
                        }
                    }
                    6 => {
                        b = a / 2_i64.pow(combo as u32);
                    }
                    7 => {
                        c = a / 2_i64.pow(combo as u32);
                    }
                    _ => unreachable!()
                }
    
                ptr += 2;
            }

            if ok && format!("Program: {}", res.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")) == self.input[4].to_string() {
                return a_init;
            }
            a_init += 1;
        }

        0
    }
        
}

crate::test!(
    0,
    "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
",

    117440,
    "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
);
