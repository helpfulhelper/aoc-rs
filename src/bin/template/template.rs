use aoc::read_to_chars;
// use aoc::read_lines;

pub struct AoCYYYY_DD {
    data: Vec<char>,
}

impl AoCYYYY_DD {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoCYYYY_DD {
    fn parse(&mut self) {
        self.data = read_to_chars("../input/YYYY/DD.txt");
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (YYYY, DD)
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output("0".to_string())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("0".to_string())
    }
}
