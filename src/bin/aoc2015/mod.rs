use crate::{Runner, Selector};

mod aoc2015_01;
mod aoc2015_02;
mod aoc2015_03;
mod aoc2015_04;
// mod aoc2015_05;
// mod aoc2015_06;
// mod aoc2015_07;
// mod aoc2015_08;
// mod aoc2015_09;
// mod aoc2015_10;
// mod aoc2015_11;
// mod aoc2015_12;
// mod aoc2015_13;
// mod aoc2015_14;
// mod aoc2015_15;
// mod aoc2015_16;
// mod aoc2015_17;
// mod aoc2015_18;
// mod aoc2015_19;
// mod aoc2015_20;
// mod aoc2015_21;
// mod aoc2015_22;
// mod aoc2015_23;
// mod aoc2015_24;
// mod aoc2015_25;

use aoc2015_01::*;
use aoc2015_02::*;
use aoc2015_03::*;
use aoc2015_04::*;
// use aoc2015_05::*;
// use aoc2015_06::*;
// use aoc2015_07::*;
// use aoc2015_08::*;
// use aoc2015_09::*;
// use aoc2015_10::*;
// use aoc2015_11::*;
// use aoc2015_12::*;
// use aoc2015_13::*;
// use aoc2015_14::*;
// use aoc2015_15::*;
// use aoc2015_16::*;
// use aoc2015_17::*;
// use aoc2015_18::*;
// use aoc2015_19::*;
// use aoc2015_20::*;
// use aoc2015_21::*;
// use aoc2015_22::*;
// use aoc2015_23::*;
// use aoc2015_24::*;
// use aoc2015_25::*;

pub fn run_2015(which: Selector) {
    let mut day01 = AoC2015_01::new();
    let mut day02 = AoC2015_02::new();
    let mut day03 = AoC2015_03::new();
    let mut day04 = AoC2015_04::new();
    // let mut day05 = AoC2015_05::new();
    // let mut day06 = AoC2015_06::new();
    // let mut day07 = AoC2015_07::new();
    // let mut day08 = AoC2015_08::new();
    // let mut day09 = AoC2015_09::new();
    // let mut day10 = AoC2015_10::new();
    // let mut day11 = AoC2015_11::new();
    // let mut day12 = AoC2015_12::new();
    // let mut day13 = AoC2015_13::new();
    // let mut day14 = AoC2015_14::new();
    // let mut day15 = AoC2015_15::new();
    // let mut day16 = AoC2015_16::new();
    // let mut day17 = AoC2015_17::new();
    // let mut day18 = AoC2015_18::new();
    // let mut day19 = AoC2015_19::new();
    // let mut day20 = AoC2015_20::new();
    // let mut day21 = AoC2015_21::new();
    // let mut day22 = AoC2015_22::new();
    // let mut day23 = AoC2015_23::new();
    // let mut day24 = AoC2015_24::new();
    // let mut day25 = AoC2015_25::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02,
        &mut day03,
        &mut day04,
        // &mut day05,
        // &mut day06,
        // &mut day07,
        // &mut day08,
        // &mut day09,
        // &mut day10,
        // &mut day11,
        // &mut day12,
        // &mut day13,
        // &mut day14,
        // &mut day15,
        // &mut day16,
        // &mut day17,
        // &mut day18,
        // &mut day19,
        // &mut day20,
        // &mut day21,
        // &mut day22,
        // &mut day23,
        // &mut day24,
        // &mut day25,
    ];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            crate::run_solution(*d);
        }
        Selector::All => {
            for d in days {
                crate::run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            crate::run_solution(*d);
        }
    }
}
