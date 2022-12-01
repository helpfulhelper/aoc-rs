// use aoc::read_to_chars;
use aoc::numbers;

pub struct AoC2015_02 {
    data: Vec<Present>,
}

pub struct Present(u32, u32, u32);

impl Present {
    fn new(l: u32, w: u32, h: u32) -> Self {
        Self(l, w, h)
    }

    fn surface_area(&self) -> u32 {
        2 * self.0 * self.1 + 2 * self.1 * self.2 + 2 * self.2 * self.0
    }

    fn extra(&self) -> u32 {
        self.0 * self.1
    }

    fn ribbon(&self) -> u32 {
        2 * self.0 + 2 * self.1 + self.0 * self.1 * self.2
    }
}

impl AoC2015_02 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2015_02 {
    fn parse(&mut self) {
        let mut data = numbers("input/2015/02.txt", 'x');
        for d in &mut data {
            d.sort();
            self.data.push(Present::new(d[0], d[1], d[2]))
        }
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2015, 2)
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(
            self.data
                .iter()
                .map(|p| p.surface_area() + p.extra())
                .sum::<u32>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.data.iter().map(|p| p.ribbon()).sum::<u32>())
    }
}
