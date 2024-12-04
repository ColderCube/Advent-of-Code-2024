use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let lines = reader.lines().flatten().collect_vec();
        let mut first_array = BinaryHeap::<u64>::new();
        let mut second_array = BinaryHeap::<u64>::new();
        // let mut answer: u64 = 0;
        for line in lines {
            let (first, second) = line.split("   ").collect_tuple().unwrap();
            let first_n = first.parse::<u64>().unwrap();
            let second_n = second.parse::<u64>().unwrap();
            first_array.push(first_n);
            second_array.push(second_n);
        }

        let answer: u64 = first_array
            .into_sorted_vec()
            .into_iter()
            .zip(second_array.into_sorted_vec().into_iter())
            .map(|x| x.0.abs_diff(x.1))
            .into_iter()
            .sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let (l, r): (Vec<u64>, Vec<u64>) = reader.lines().flatten().map(|input| {
            let (first, second) = input.split("   ").collect_tuple().unwrap();
            (first.parse::<u64>().unwrap(), second.parse::<u64>().unwrap())
        }).into_iter().map(|(a,b)| (a,b)).unzip();
        let mut freq: HashMap<u64, u64> = HashMap::new();
        r.iter().for_each(|r| *freq.entry(*r).or_insert(0) += 1);
        Ok(l.iter().filter_map(|l| freq.get(l).map(|f| l * f)).sum())
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
