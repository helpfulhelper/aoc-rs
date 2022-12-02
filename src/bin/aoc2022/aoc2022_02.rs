//use aoc::read_to_chars;
use aoc::read_lines;

#[derive(Default)]
pub struct AoC2022_02 {
    data: Vec<(String, String)>,
}

impl AoC2022_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Copy, Clone)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone)]
pub enum Results {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

use Move::{Paper, Rock, Scissors};
use Results::{Draw, Lose, Win};

impl Move {
    pub fn parse(c: &str) -> Move {
        match c {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("BAD!"),
        }
    }
}

impl Results {
    pub fn parse(c: &str) -> Results {
        match c {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("BAD!"),
        }
    }
}

fn round(e: Move, m: Move) -> Results {
    match (e, m) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => Win,
        (Rock, Scissors) => Lose,
        (Paper, Rock) => Lose,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Lose,
        (Scissors, Scissors) => Draw,
    }
}

fn fixed(e: Move, m: Results) -> Move {
    match (e, m) {
        (Rock, Lose) => Scissors,
        (Rock, Draw) => Rock,
        (Rock, Win) => Paper,
        (Paper, Lose) => Rock,
        (Paper, Draw) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Lose) => Paper,
        (Scissors, Draw) => Scissors,
        (Scissors, Win) => Rock,
    }
}

impl crate::Runner for AoC2022_02 {
    fn parse(&mut self) {
        let lines = read_lines("../input/2022/02.txt");
        for l in lines {
            let (e, m) = l.split_once(' ').unwrap();
            self.data.push((e.to_string(), m.to_string()));
        }
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2022, 02)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut match_score = 0;
        let mut hand_score = 0;
        for d in &self.data {
            let e = Move::parse(&d.0);
            let m = Move::parse(&d.1);
            hand_score += m as i32;
            match_score += round(e, m) as i32;
        }
        let total = match_score + hand_score;
        crate::output(total.to_string())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut match_score = 0;
        let mut hand_score = 0;
        for d in &self.data {
            let e = Move::parse(&d.0);
            let m = Results::parse(&d.1);
            hand_score += fixed(e, m) as i32;
            match_score += m as i32;
        }
        let total = match_score + hand_score;
        crate::output(total.to_string())
    }
}
