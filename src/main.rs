#[cfg(test)]
extern crate indoc;

use std::env;
use std::time::Instant;
use humantime::format_duration;

use days::Day;

mod days;
#[cfg(feature = "default")]
mod tools;

fn day(day: i32) -> Box<dyn Day> {
    match day {
        1  => Box::new(days::day_01::Day::from_file("inputs/day-01")),
        2  => Box::new(days::day_02::Day::from_file("inputs/day-02")),
        3  => Box::new(days::day_03::Day::from_file("inputs/day-03")),
        4  => Box::new(days::day_04::Day::from_file("inputs/day-04")),
        5  => Box::new(days::day_05::Day::from_file("inputs/day-05")),
        6  => Box::new(days::day_06::Day::from_file("inputs/day-06")),
        7  => Box::new(days::day_07::Day::from_file("inputs/day-07")),
        8  => Box::new(days::day_08::Day::from_file("inputs/day-08")),
        9  => Box::new(days::day_09::Day::from_file("inputs/day-09")),
        10 => Box::new(days::day_10::Day::from_file("inputs/day-10")),
        11 => Box::new(days::day_11::Day::from_file("inputs/day-11")),
        12 => Box::new(days::day_12::Day::from_file("inputs/day-12")),
        13 => Box::new(days::day_13::Day::from_file("inputs/day-13")),
        14 => Box::new(days::day_14::Day::from_file("inputs/day-14")),
        15 => Box::new(days::day_15::Day::from_file("inputs/day-15")),
        16 => Box::new(days::day_16::Day::from_file("inputs/day-16")),
        17 => Box::new(days::day_17::Day::from_file("inputs/day-17")),
        18 => Box::new(days::day_18::Day::from_file("inputs/day-18")),
        19 => Box::new(days::day_19::Day::from_file("inputs/day-19")),
        20 => Box::new(days::day_20::Day::from_file("inputs/day-20")),
        21 => Box::new(days::day_21::Day::from_file("inputs/day-21")),
        22 => Box::new(days::day_22::Day::from_file("inputs/day-22")),
        23 => Box::new(days::day_23::Day::from_file("inputs/day-23")),
        24 => Box::new(days::day_24::Day::from_file("inputs/day-24")),
        25 => Box::new(days::day_25::Day::from_file("inputs/day-25")),
        _ => todo!()
    }
}

fn d() -> i32 {
    if cfg!(feature = "day-25") {
        25
    } else if cfg!(feature = "day-24") {
        24
    } else if cfg!(feature = "day-23") {
        23
    } else if cfg!(feature = "day-22") {
        22
    } else if cfg!(feature = "day-21") {
        21
    } else if cfg!(feature = "day-20") {
        20
    } else if cfg!(feature = "day-19") {
        19
    } else if cfg!(feature = "day-18") {
        18
    } else if cfg!(feature = "day-17") {
        17
    } else if cfg!(feature = "day-16") {
        16
    } else if cfg!(feature = "day-15") {
        15
    } else if cfg!(feature = "day-14") {
        14
    } else if cfg!(feature = "day-13") {
        13
    } else if cfg!(feature = "day-12") {
        12
    } else if cfg!(feature = "day-11") {
        11
    } else if cfg!(feature = "day-10") {
        10
    } else if cfg!(feature = "day-9") {
        9
    } else if cfg!(feature = "day-8") {
        8
    } else if cfg!(feature = "day-7") {
        7
    } else if cfg!(feature = "day-6") {
        6
    } else if cfg!(feature = "day-5") {
        5
    } else if cfg!(feature = "day-4") {
        4
    } else if cfg!(feature = "day-3") {
        3
    } else if cfg!(feature = "day-2") {
        2
    } else if cfg!(feature = "day-1") {
        1
    } else {
        0
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let d = d();
    let part = args.get(1).map(|arg| arg.parse::<i32>().expect("/!\\ The exercice part must be 1 or 2")).expect("/!\\ The exercice part must be provided as argument");

    println!("\nDay {d} part {part}");

    let day = day(d);

    let now = Instant::now();

    match part {
        1 => day.run_part_one(),
        2 => day.run_part_two(),
        _ => println!("/!\\ The exercice part must be 1 or 2")
    }

    println!("Duration : {}", format_duration(now.elapsed()));
}
