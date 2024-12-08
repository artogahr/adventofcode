use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let f = BufReader::new(File::open("input.txt").unwrap());

    let lab_map: Vec<Vec<char>> = f.lines().map(|l| l.unwrap().chars().collect()).collect();
    for x in 0..lab_map.len() {
        for y in 0..lab_map[0].len() {
            match lab_map[x as usize][y as usize] {
                '^' | '<' | '>' | 'v' => {
                    println!(
                        "Part 1: {} steps",
                        part1(&lab_map, (x as usize, y as usize))
                    );
                    std::thread::sleep(Duration::from_secs(1));
                    println!(
                        "Part 2: {} steps",
                        part2(&lab_map, (x as usize, y as usize))
                    );
                }
                _ => continue,
            }
        }
    }
}

fn part1(lab_map: &Vec<Vec<char>>, pos: (usize, usize)) -> u32 {
    let mut lab_map = lab_map.clone();
    let mut steps = 0;
    let mut pos = pos;
    let mut direction = lab_map[pos.0 as usize][pos.1 as usize];
    while pos.0 > 0
        && pos.0 < lab_map.len() as usize - 1
        && pos.1 > 0
        && pos.1 < lab_map[0].len() as usize - 1
    {
        print_map(&lab_map, pos, steps);
        match direction {
            '^' => {
                let new_pos = lab_map[pos.0 - 1][pos.1];
                if new_pos == '#' {
                    direction = '>';
                    lab_map[pos.0][pos.1] = direction;
                    continue;
                }
                if new_pos == '.' || new_pos == 'X' {
                    if new_pos == 'X' {
                        steps -= 1
                    };
                    lab_map[pos.0][pos.1] = 'X';
                    pos = (pos.0 - 1, pos.1);
                    lab_map[pos.0][pos.1] = direction;
                }
            }
            '>' => {
                let new_pos = lab_map[pos.0][pos.1 + 1];
                if new_pos == '#' {
                    direction = 'v';
                    lab_map[pos.0][pos.1] = direction;
                    continue;
                }
                if new_pos == '.' || new_pos == 'X' {
                    if new_pos == 'X' {
                        steps -= 1
                    };
                    lab_map[pos.0][pos.1] = 'X';
                    pos = (pos.0, pos.1 + 1);
                    lab_map[pos.0][pos.1] = direction;
                }
            }
            'v' => {
                let new_pos = lab_map[pos.0 + 1][pos.1];
                if new_pos == '#' {
                    direction = '<';
                    lab_map[pos.0][pos.1] = direction;
                    continue;
                }
                if new_pos == '.' || new_pos == 'X' {
                    if new_pos == 'X' {
                        steps -= 1
                    };
                    lab_map[pos.0][pos.1] = 'X';
                    pos = (pos.0 + 1, pos.1);
                    lab_map[pos.0][pos.1] = direction;
                }
            }
            '<' => {
                let new_pos = lab_map[pos.0][pos.1 - 1];
                if new_pos == '#' {
                    direction = '^';
                    lab_map[pos.0][pos.1] = direction;
                    continue;
                }
                if new_pos == '.' || new_pos == 'X' {
                    if new_pos == 'X' {
                        steps -= 1
                    };
                    lab_map[pos.0][pos.1] = 'X';
                    pos = (pos.0, pos.1 - 1);
                    lab_map[pos.0][pos.1] = direction;
                }
            }
            _ => {}
        }
        steps += 1;
    }
    println!("Found an exit!");
    steps + 1
}

fn print_map(lab_map: &Vec<Vec<char>>, pos: (usize, usize), steps: u32) {
    print!("{esc}[1;1H", esc = 27 as char);
    println!("Steps taken: {steps}");
    for x in 0..lab_map.len() {
        for y in 0..lab_map[0].len() {
            print!("{} ", lab_map[x][y]);
        }
        print!("\n");
    }
    std::thread::sleep(Duration::from_millis(10));
}

fn part2(lab_map: &Vec<Vec<char>>, pos: (usize, usize)) -> u32 {
    todo!();
}
