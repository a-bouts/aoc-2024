use std::fs::read_to_string;
use anyhow::Result;

crate::day!(day_01, "day-1");
crate::day!(day_02, "day-2");
crate::day!(day_03, "day-3");
crate::day!(day_04, "day-4");
crate::day!(day_05, "day-5");
crate::day!(day_06, "day-6");
crate::day!(day_07, "day-7");
crate::day!(day_08, "day-8");
crate::day!(day_09, "day-9");
crate::day!(day_10, "day-10");
crate::day!(day_11, "day-11");
crate::day!(day_12, "day-12");
crate::day!(day_13, "day-13");
crate::day!(day_14, "day-14");
crate::day!(day_15, "day-15");
crate::day!(day_16, "day-16");
crate::day!(day_17, "day-17");
crate::day!(day_18, "day-18");
crate::day!(day_19, "day-19");
crate::day!(day_20, "day-20");
crate::day!(day_21, "day-21");
crate::day!(day_22, "day-22");
crate::day!(day_23, "day-23");
crate::day!(day_24, "day-24");
crate::day!(day_25, "day-25");

pub(crate) trait Day {
    fn new(input: Vec<String>) -> Self where Self: Sized;

    fn from_file(filename: &str) -> Self where Self: Sized {
        Self::new(
            read_to_string(filename) 
                .unwrap()
                .lines()
                .map(String::from)
                .collect()
        )
    }

    #[allow(dead_code)]
    fn from_sample(sample: &str) -> Self where Self: Sized {
        Self::new(
            sample 
                .lines()
                .map(String::from)
                .collect()
        )
    }

    fn run_part_one(&self) {
        match self.part_one() {
            Ok(res) => {
                println!("\nResult : [{res}]\n");
            }
            Err(e) => {
                println!("/nError : {:?}", e);
            }
        }
    }

    fn part_one(&self) -> Result<i64>;

    fn run_part_two(&self) {
        match self.part_two() {
            Ok(res) => {
                println!("\nResult : [{res}]\n");
            }
            Err(e) => {
                println!("/nError : {:?}", e);
            }
        }
    }

    fn part_two(&self) -> Result<i64>;
}

#[macro_export]
macro_rules! day {
    ($mod:ident, $feature:expr) => {
        #[cfg(feature = $feature)]
        pub(crate) mod $mod;
        #[cfg(not(feature = $feature))]
        pub(crate) mod $mod {
            use anyhow::Result;

            use crate::days::Day as D;

            pub(crate) struct Day {
            }

            impl D for Day {
                fn new(_input: Vec<String>) -> Self {
                    Self {
                    }
                }

                fn part_one(&self) -> Result<i64> {

                    todo!()
                }

                fn part_two(&self) -> Result<i64> {    
                
                    todo!()
                }       
            }
        }
    };
}

#[macro_export]
macro_rules! test {
    ($p1_out:expr, $p1_in:expr, $p2_out:expr, $p2_in:expr) => {
        #[cfg(test)]
        mod tests {

            use anyhow::Result;

            use super::*;

            #[test]
            fn part_one() -> Result<()> {

                let exo: Day = Day::from_sample(
                    indoc::indoc! { $p1_in }
                );

                assert_eq!($p1_out, exo.part_one()?);
                Ok(())
            }

            #[test]
            fn part_two() -> Result<()> {

                let exo: Day = Day::from_sample(
                    indoc::indoc! { $p2_in }
                );

                assert_eq!($p2_out, exo.part_two()?);
                Ok(())
            }
        }
    }
}