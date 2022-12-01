use aoc::read_to_chars;
// use aoc::read_lines;

pub struct AoC2015_02 {
    data: Vec<char>,
}

impl AoC2015_02 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2015_02 {
    fn parse(&mut self) {
        self.data = read_to_chars("input/2022-02.txt");
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2015, 2)
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
