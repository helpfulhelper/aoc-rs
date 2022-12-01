use aoc::read_to_chars;
// use aoc::read_lines;

pub struct AoC2022_01 {
    data: Vec<char>,
}

impl AoC2022_01 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2022_01 {
    fn parse(&mut self) {
        self.data = read_to_chars("input/2022-01.txt");
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2022, 1)
    }

    fn part1(&mut self) -> Vec<String> {
        vec!["0".to_string()]
        // crate::output(self.data.iter())
    }

    fn part2(&mut self) -> Vec<String> {
        // crate::output(self.data.iter())
        vec!["0".to_string()]
    }
}