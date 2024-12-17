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
        let b = self.input[1].split(" ").collect::<Vec<_>>()[2].parse::<i64>().unwrap();
        let c = self.input[2].split(" ").collect::<Vec<_>>()[2].parse::<i64>().unwrap();

        let program = self.input[4].split(" ").collect::<Vec<_>>()[1].split(",").map(|c| c.parse::<i64>().unwrap()).collect::<Vec<_>>();

        fn search(a_init: i64, b_init: i64, c_init: i64, program: &Vec<i64>, index: usize) -> bool {

            println!("{index} {a_init}");


            let mut cpt = 0;

            while index == program.len() - 1 || cpt < 8 {
                let mut a = a_init + cpt;
                let mut b = b_init;
                let mut c = c_init;
    
                let mut ptr = 0;    
                let mut ok = false;
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
                            if combo % 8 == program[index] {
                                ok = true;
                            }
                            break;
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
                if ok {
                    if index == 0 {
                        println!("{a}");
                        return true
                    } else if search(a << 3, b_init, c_init, program, index - 1) {
                        return true
                    }
                } 

                cpt += 1;
            }

            false
        }


        search(0, b, c, &program, program.len() - 1);

        0
    }
        
}

crate::test!(
    0,
    "
Register A: 90938893795561
Register B: 0
Register C: 0

Program: 2,4,1,6,7,5,4,6,1,4,5,5,0,3,3,0
",

    117440,
    "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
);
