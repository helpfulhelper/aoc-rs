// use aoc::read_to_chars;
use aoc::read_lines;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct AoC2022_03 {
    data: Vec<char>,
    badges: Vec<char>,
    scores: HashMap<char, usize>,
}

impl AoC2022_03 {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            badges: Vec::new(),
            scores: gen_scores(),
        }
    }
}

fn gen_scores() -> HashMap<char, usize> {
    let mut temp: HashMap<char, usize> = HashMap::new();
    for (score, ch) in gen_alpha().iter().enumerate() {
        temp.insert(ch.to_owned(), score + 1);
    }
    temp
}

fn gen_alpha() -> Vec<char> {
    let alpha: Vec<char> = (b'a'..=b'z')
        .map(|c| c as char)
        .chain((b'A'..=b'Z').map(|c| c as char))
        .collect();
    return alpha.to_owned();
}

impl crate::Runner for AoC2022_03 {
    fn parse(&mut self) {
        let lines = read_lines("../input/2022/03.txt");
        for l in lines {
            let (left, right) = l.split_at(l.len() / 2);
            let lset: HashSet<char> = left.chars().collect();
            let rset: HashSet<char> = right.chars().collect();
            let dup = lset.intersection(&rset).collect::<String>();
            self.data.push(dup.chars().nth(0).unwrap());
        }

        let lines2 = read_lines("../input/2022/03.txt");
        let trio: Vec<Vec<String>> = lines2.chunks(3).map(|x| x.to_vec()).collect();
        for t in trio {
            let a: HashSet<char> = t[0].chars().collect();
            let b: HashSet<char> = t[1].chars().collect();
            let c: HashSet<char> = t[2].chars().collect();
            let d1: HashSet<char> = a.intersection(&b).collect::<String>().chars().collect();
            let d2 = d1.intersection(&c).collect::<String>();
            self.badges.push(d2.chars().nth(0).unwrap());
        }

        // print!("{:?}", gen_alpha());
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2022, 03)
    }

    fn part1(&mut self) -> Vec<String> {
        let total: usize = self.data.iter().map(|c| self.scores.get(c).unwrap()).sum();
        crate::output(total.to_string())
    }

    fn part2(&mut self) -> Vec<String> {
        let total: usize = self
            .badges
            .iter()
            .map(|c| self.scores.get(c).unwrap())
            .sum();
        crate::output(total.to_string())
    }
}
