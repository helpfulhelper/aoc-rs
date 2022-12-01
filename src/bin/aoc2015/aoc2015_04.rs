use aoc::read_to_chars;
// use aoc::read_lines;

pub struct AoC2015_04 {
    data: Vec<char>,
}

impl AoC2015_04 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2015_04 {
    fn parse(&mut self) {
        self.data = read_to_chars("../input/2015/04.txt");
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2015, 04)
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output("0".to_string())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("0".to_string())
    }
}
