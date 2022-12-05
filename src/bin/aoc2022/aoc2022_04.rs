use aoc::read_lines;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::separated_pair as sp;
use nom::IResult;
use std::str::FromStr;

pub struct AoC2022_04 {
    data: Vec<Vec<u32>>,
}

struct Assignments {
    left: (u32, u32),
    right: (u32, u32),
}

impl Assignments {
    fn parse(s: &str) -> Self {
        let (_, (mut p1, mut p2)) =
            sp(sp(pn, tag("-"), pn), tag(","), sp(pn, tag("-"), pn))(s).unwrap();
        let (left, right) = if p1.0 < p2.0 { (p1, p2) } else { (p2, p1) };
        Assignments { left, right }
    }
}

fn pn(s: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(s)
}

impl AoC2022_04 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

fn contains(line: Vec<u32>) -> bool {
    let (lst, lend, rst, rend) = line.iter().next_tuple().unwrap();

    if lst <= rst && rend <= lend {
        return true;
    } else if rst <= lst && lend <= rend {
        return true;
    }
    false
}

fn overlap(line: Vec<u32>) -> bool {
    let (lst, lend, rst, rend) = line.iter().next_tuple().unwrap();

    if lst <= rst && rst <= lend {
        return true;
    } else if rst <= lst && lst <= rend {
        return true;
    } else if rst <= lend && lend <= rend {
        return true;
    } else if lst <= rend && rend <= lend {
        return true;
    }
    false
}

impl crate::Runner for AoC2022_04 {
    fn parse(&mut self) {
        // for lines in read_lines("../input/2022/04.txt") {
        //     self.data.push(
        //         lines
        //             .split(|x| x == '-' || x == ',')
        //             .map(|n| n.parse::<u32>().unwrap())
        //             .collect::<Vec<u32>>(),
        //     );
        // }
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2022, 04)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;
        for l in &self.data {
            if contains(l.to_vec()) {
                total += 1;
            }
        }
        crate::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;
        for l in &self.data {
            if overlap(l.to_vec()) {
                total += 1;
            }
        }
        crate::output(total)
    }
}
