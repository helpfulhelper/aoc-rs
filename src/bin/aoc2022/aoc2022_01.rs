use aoc::read_lines2;

pub struct AoC2022_01 {
    data: Vec<u32>,
}

impl AoC2022_01 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2022_01 {
    fn parse(&mut self) {
        let temp = read_lines2("input/2022-01.txt");
        let mut t: Vec<u32> = Vec::new();
        for x in temp {
            if let Ok(y) = x.parse::<u32>() {
                t.push(y)
            } else {
                self.data.push(t.iter().sum());
                t.clear();
            };
        }

        self.data.sort();
    }

    fn name(&self) -> (usize, usize) {
        (2022, 1)
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.data.last().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(
            self.data.pop().unwrap() + self.data.pop().unwrap() + self.data.pop().unwrap(),
        )
    }
}
