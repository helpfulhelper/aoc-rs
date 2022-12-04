use itertools::Itertools;
use std::ops::Range;
//use aoc::read_to_chars;
//use aoc::numbers;
use aoc::read_lines;

pub struct AoC2022_04 {
    data: Vec<Vec<u32>>,
}

impl AoC2022_04 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

fn contains(line: Vec<u32>) -> bool {
    let (lst, lend, rst, rend) = line.iter().next_tuple().unwrap();
    // let lst = line[0];
    // let lend = line[1];
    // let rst = line[2];
    // let rend = line[3];

    if rst >= lst && rend <= lend {
        return true;
    } else if lst >= rst && lend <= rend {
        return true;
    }
    false
}

fn overlap(line: Vec<u32>) -> bool {
    let (lst, lend, rst, rend) = line.iter().next_tuple().unwrap();
    // let lst = line[0];
    // let lend = line[1];
    // let rst = line[2];
    // let rend = line[3];

    // let (lst, lend, rst, rend) = line
    //     .clone()
    //     .into_iter()
    //     .take(4)
    //     .collect::<(u32, u32, u32, u32)>();
    if rst >= lst && rst <= lend {
        return true;
    } else if lst >= rst && lst <= rend {
        return true;
    } else if lend >= rst && lend <= rend {
        return true;
    } else if rend >= lst && rend <= lend {
        return true;
    }
    false
}

impl crate::Runner for AoC2022_04 {
    fn parse(&mut self) {
        for lines in read_lines("../input/2022/04.txt") {
            self.data.push(
                lines
                    .split(|x| x == '-' || x == ',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            );
        }
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
