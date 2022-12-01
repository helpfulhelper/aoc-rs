use aoc::read_to_chars;
// use aoc::numbers;
// use aoc::read_lines;

use std::collections::HashSet;

pub struct AoC2015_03 {
    data: Vec<char>,
}

impl AoC2015_03 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2015_03 {
    fn parse(&mut self) {
        self.data = read_to_chars("input/2015/03.txt");
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2015, 3)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut grid = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        for c in &self.data {
            grid.insert((x, y));
            match c {
                '^' => y += 1,
                'v' => y -= 1,
                '<' => x -= 1,
                '>' => x += 1,
                _ => panic!("invalid char '{c}'"),
            }
        }
        grid.insert((x, y));
        crate::output(grid.len())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("0".to_string())
    }
}
