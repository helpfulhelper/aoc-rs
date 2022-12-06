use aoc::read_lines;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res,
    sequence::separated_pair, IResult,
};
use std::str::FromStr;

pub struct AoC2022_04 {
    data: Vec<Assignments>,
}

struct Assignments {
    left: (u32, u32),
    right: (u32, u32),
}

impl Assignments {
    fn parse(s: &str) -> Self {
        let (_, (p1, p2)) = separated_pair(
            separated_pair(parse_num, tag("-"), parse_num),
            tag(","),
            separated_pair(parse_num, tag("-"), parse_num),
        )(s)
        .unwrap();
        let (left, right) = if p1.0 < p2.0 { (p1, p2) } else { (p2, p1) };
        Assignments { left, right }
    }
    fn fully_contains(a: &&Assignments) -> bool {
        (a.left.0 <= a.right.0 && a.right.1 <= a.left.1)
            || (a.right.0 <= a.left.0 && a.left.1 <= a.right.1)
    }
    fn has_overlap(a: &&Assignments) -> bool {
        a.left.1 >= a.right.0
    }
}

fn parse_num(s: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(s)
}

impl AoC2022_04 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2022_04 {
    fn parse(&mut self) {
        for lines in read_lines("../input/2022/04.txt") {
            self.data.push(Assignments::parse(&lines));
        }
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2022, 04)
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.data.iter().filter(Assignments::fully_contains).count())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.data.iter().filter(Assignments::has_overlap).count())
    }
}
