use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let lines = read_lines("input.txt");
    // numbers and a list of numbers that
    // must come before it
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        if line.chars().any(|ch| ch == '|') {
            let (left, right) = line
                .split_once("|")
                .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
                .unwrap();
            match rules.get_mut(&right) {
                Some(comes_after) => {
                    comes_after.push(left);
                }
                None => {
                    rules.insert(right, vec![left]);
                }
            }
        } else {
            let numbers: Vec<u32> = line
                .split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            updates.push(numbers);
        }
    }
    println!("Part 1: {}", part1(&updates, &rules));
    println!("Part 2: {}", part2(&updates, &rules));
}

fn part1(updates: &Vec<Vec<u32>>, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut sum = 0;
    for numbers in updates {
        if is_sorted(&numbers, &rules) {
            sum += numbers[numbers.len() / 2];
        }
    }
    sum
}

fn part2(updates: &Vec<Vec<u32>>, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut sum = 0;
    for numbers in updates {
        if !is_sorted(&numbers, &rules) {
            sum += sort_numbers(numbers.clone(), &rules)[numbers.len() / 2];
        }
    }
    sum
}

fn is_sorted(numbers: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, num) in numbers.iter().enumerate() {
        if let Some(num_comes_after) = rules.get(num) {
            for j in i..numbers.len() {
                if num_comes_after.iter().any(|num| num == &numbers[j]) {
                    return false;
                }
            }
        }
    }
    true
}

fn sort_numbers(mut numbers: Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    while !is_sorted(&numbers, &rules) {
        for i in 0..numbers.len() {
            let num = numbers[i];
            if let Some(numbers_that_must_come_before) = rules.get(&num) {
                for j in i..numbers.len() {
                    for number in numbers_that_must_come_before.iter() {
                        if *number == numbers[j] {
                            numbers.swap(i, j);
                        }
                    }
                }
            }
        }
    }
    numbers
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
