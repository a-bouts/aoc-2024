use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
        
        struct Door {
            opp: String,
            in1_name: String,
            in1: Rc<RefCell<isize>>,
            in2_name: String,
            in2: Rc<RefCell<isize>>,
            out_name: String,
            out: Rc<RefCell<isize>>,
            done:bool
        }

        impl Door {
            fn new(opp: &str, in1: &str, in2: &str, out: &str, values: &mut HashMap<String, Rc<RefCell<isize>>>) -> Self {
                let in1_name = in1.to_string();
                let in2_name = in2.to_string();
                let out_name = out.to_string();

                let in1 = values.entry(in1_name.clone()).or_insert(Rc::new(RefCell::new(-1))).clone();
                let in2 = values.entry(in2_name.clone()).or_insert(Rc::new(RefCell::new(-1))).clone();
                let out = values.entry(out_name.clone()).or_insert(Rc::new(RefCell::new(-1))).clone();
                
                Self {
                    opp: opp.to_string(),
                    in1_name,
                    in1,
                    in2_name,
                    in2,
                    out_name,
                    out,
                    done: false,
                }
            }

            fn eval(&mut self) -> bool {
                if *self.in1.borrow() >= 0 && *self.in2.borrow() >= 0 {
                    if self.opp == "AND" {
                        *self.out.borrow_mut() = *self.in1.borrow() & *self.in2.borrow();
                    } else if self.opp == "OR" {
                        *self.out.borrow_mut() = *self.in1.borrow() | *self.in2.borrow();
                    } else if self.opp == "XOR" {
                        *self.out.borrow_mut() = *self.in1.borrow() ^ *self.in2.borrow();
                    }
                    println!("{}({}) {} {}({}) -> {}({})", self.in1_name, *self.in1.borrow(), self.opp, self.in2_name, *self.in2.borrow(), self.out_name, *self.out.borrow());
                    self.done = true;
                    return true
                }
                false
            }
        }


        fn result(filter: &str, values: &HashMap<String, Rc<RefCell<isize>>>) -> Option<isize> {
            let mut outs = values.keys().filter(|k| k.starts_with(filter)).collect::<Vec<_>>();
            outs.sort();
            outs.reverse();
    
            let mut res = 0;
            for out in outs {
                let val = *values.get(out).unwrap().borrow();
                if val < 0 {
                    return None
                }
                res *= 2;
                res += val;
            }

            Some(res)
        }

        let values = &mut HashMap::new();
        let mut doors = vec![];
        for line in &self.input {
            let s = line.split(" ").collect::<Vec<_>>();
            if s.len() == 2 {
                values.insert(s[0].trim_end_matches(":").to_string(), Rc::new(RefCell::new(s[1].parse::<isize>().unwrap())));
            } else if s.len() == 5 {
                doors.push(Door::new(s[1], s[0], s[2], s[4], values));
            }
        }

        let mut change = true;
        while change {
            change = false;
            for door in doors.iter_mut() {
                if door.done {
                    continue
                }
                if door.eval() {
                    change = true
                }
            }
        }

        let res = result("z", &values).unwrap();

        Ok(res as i64)
    }

    fn part_two(&self) -> Result<i64> {    
        
        #[derive(Clone)]
        struct Door {
            opp: String,
            in1_name: String,
            in1: Rc<RefCell<isize>>,
            in2_name: String,
            in2: Rc<RefCell<isize>>,
            out_name: String,
            out: Rc<RefCell<isize>>,
            done:bool
        }

        impl Door {
            fn new(opp: &str, in1: &str, in2: &str, out: &str, values: &mut HashMap<String, Rc<RefCell<isize>>>) -> Self {
                let in1_name = in1.to_string();
                let in2_name = in2.to_string();
                let out_name = out.to_string();

                let in1 = values.entry(in1_name.clone()).or_insert(Rc::new(RefCell::new(-1))).clone();
                let in2 = values.entry(in2_name.clone()).or_insert(Rc::new(RefCell::new(-1))).clone();
                let out = values.entry(out_name.clone()).or_insert(Rc::new(RefCell::new(-1))).clone();
                
                Self {
                    opp: opp.to_string(),
                    in1_name,
                    in1,
                    in2_name,
                    in2,
                    out_name,
                    out,
                    done: false,
                }
            }

            fn eval(&mut self) -> bool {
                if *self.in1.borrow() >= 0 && *self.in2.borrow() >= 0 {
                    if self.opp == "AND" {
                        *self.out.borrow_mut() = *self.in1.borrow() & *self.in2.borrow();
                    } else if self.opp == "OR" {
                        *self.out.borrow_mut() = *self.in1.borrow() | *self.in2.borrow();
                    } else if self.opp == "XOR" {
                        *self.out.borrow_mut() = *self.in1.borrow() ^ *self.in2.borrow();
                    }
                    self.done = true;
                    return true
                }
                false
            }
        }


        fn result(filter: &str, values: &HashMap<String, Rc<RefCell<isize>>>) -> Option<isize> {
            let mut outs = values.keys().filter(|k| k.starts_with(filter)).collect::<Vec<_>>();
            outs.sort();
            outs.reverse();
    
            let mut res = 0;
            for out in outs {
                let val = *values.get(out).unwrap().borrow();
                if val < 0 {
                    return None
                }
                res *= 2;
                res += val;
            }

            Some(res)
        }

        for x in 0..44 {
            for y in 0..44 {

                let values: &mut HashMap<String, Rc<RefCell<isize>>> = &mut HashMap::new();
                let mut doors = vec![];
                for line in &self.input {
                    let s = line.split(" ").collect::<Vec<_>>();
                    if s.len() == 2 {
                        //values.insert(s[0].trim_end_matches(":").to_string(), Rc::new(RefCell::new(s[1].parse::<isize>().unwrap())));
                        if s[0].starts_with("x") && s[0].trim_end_matches(":").trim_start_matches("x").parse::<isize>().unwrap() <= x {
                            values.insert(s[0].trim_end_matches(":").to_string(), Rc::new(RefCell::new(1_isize)));
                        } else if s[0].starts_with("y") && s[0].trim_end_matches(":").trim_start_matches("y").parse::<isize>().unwrap() <= y {
                            values.insert(s[0].trim_end_matches(":").to_string(), Rc::new(RefCell::new(1_isize)));
                        } else {
                            values.insert(s[0].trim_end_matches(":").to_string(), Rc::new(RefCell::new(0_isize)));
                        }
        
                    } else if s.len() == 5 {
                        doors.push(Door::new(s[1], s[0], s[2], s[4], values));
                        // println!("{} -> {}_{}_{};", s[0], s[1], s[0], s[2]);
                        // println!("{} -> {}_{}_{};", s[2], s[1], s[0], s[2]);
                        // println!("{}_{}_{} -> {};", s[1], s[0], s[2], s[4]);
                    }
                }
        
                let mut change = true;
                while change {
                    change = false;
                    for door in doors.iter_mut() {
                        if door.done {
                            continue
                        }
                        if door.eval() {
                            change = true
                        }
                    }
                }
        
                let xx = result("x", &values).unwrap();

                let yy = result("y", &values).unwrap();

                let zz = result("z", &values).unwrap();

                if zz != xx + yy {
                    println!("fail ({x}, {y}) {xx} + {yy} = {zz}");
                    return Ok(0)
                }
            }
        }
        
        // values.keys().filter(|k| k.starts_with("x")).for_each(|k| {
        //     println!("{} [style=filled,color=blue];", k)
        // });

        // values.keys().filter(|k| k.starts_with("y")).for_each(|k| {
        //     println!("{} [style=filled,color=green];", k)
        // });

        // values.keys().filter(|k| k.starts_with("z")).for_each(|k| {
        //     println!("{} [style=filled,color=red];", k)
        // });


        // let x = result("x", &values).unwrap();

        // let y = result("y", &values).unwrap();

        // println!("{x} + {y} = {}", x + y);

// ntr XOR gcc -> bfq
// vgg OR pph -> z27

// mdg XOR jss -> hmt
// jss AND mdg -> z18

// x39 XOR y39 -> fjp
// y39 AND x39 -> bng

// fqh XOR ctc -> hkh
// y31 AND x31 -> z31


bfq,bng,fjp,hkh,hmt,z18,z27,z31

//  1111111111111111111111111110
// 10111111111111111111111111110

// 110011101101110101001101110011110010110100011
// 101011011010011011100111111111111111011001101

//101111 1 00 10000 1 0000 1101011 10 011110010001110000
//101111 1 00 10000 1 0000 1101011 10 011110010001110000

//101111 0 00 01111 1 1111 1101011 01 011110010001110000


// y16 AND x16 -> bpn

// hdg AND qpj -> dhg
// dhg OR bpn -> pch

// y17 XOR x17 -> pbr
// pbr AND pch -> rrb

// y17 AND x17 -> dpj
// rrb OR dpj -> mdg
// jss AND mdg -> z18

        // let mut change = true;
        // while change {
        //     change = false;
        //     for door in doors.iter_mut() {
        //         if door.done {
        //             continue
        //         }
        //         if door.eval() {
        //             change = true
        //         }
        //     }
        // }

        // let res = result("z", &values).unwrap();

        Ok(0)
    
    }
        
}

crate::test!(
    2024,
    "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",

    0,
    "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
);
