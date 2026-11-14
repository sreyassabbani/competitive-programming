use std::{
    error::Error,
    fs,
    io::{self, prelude::*},
};

pub fn part1() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("2025/inputs/day01.txt")?;
    let reader = io::BufReader::new(file);

    // dial position
    let mut dial = 50;

    // actual password
    let mut pass = 0;

    let mut l = 0;

    for line in reader.lines() {
        let line = line?;
        let change = line[1..].parse::<i32>()? % 100;

        if line.starts_with('R') {
            dial += change;
            dial = dial % 100;
        } else {
            dial -= change;
        }

        while dial < 0 {
            dial += 100;
        }

        println!("line {l}, dial: {dial}");

        l += 1;

        if dial == 0 {
            pass += 1;
        }
    }

    println!("password: {pass}");

    Ok(())
}

pub fn part2() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("2025/inputs/day01.txt")?;
    let reader = io::BufReader::new(file);

    // dial position
    let mut dial = 50;

    // actual password
    let mut pass = 0;

    let mut l = 0;

    for line in reader.lines() {
        let line = line?;
        let change = line[1..].parse::<i32>()?;

        if line.starts_with('R') {
            pass += num_passes(dial, change);
            dial += change;
        } else {
            pass += num_passes(dial, -change);
            dial -= change;
        }

        dial %= 100;

        if dial < 0 {
            dial += 100;
        }

        println!("line {l}, dial: {dial}, passes: {pass}");

        l += 1;
    }

    println!("password: {pass}");

    Ok(())
}
// R90
// R1000
// L30

fn num_passes(dial_pos: i32, change: i32) -> i32 {
    // note how if dial_pos == 0, then it does NOT cross
    // however if dial_pos + change == 0, then it DOES cross

    let mut n = 0;

    if dial_pos + change > 99 {
        n += (dial_pos + change) / 100;
    }

    if dial_pos + change < 0 {
        n += 1 - ((dial_pos + change) / 100);
    }

    if dial_pos + change == 0 {
        n += 1;
    }

    if dial_pos == 0 {
        n -= 1;
    }

    return n.max(0);
}

// fn crosses(dial_pos: i32, change: i32) -> bool {
//     // note how if dial_pos == 0, then it does NOT cross
//     // however if dial_pos + change == 0, then it DOES cross

//     if dial_pos + change > 99 {
//         return true;
//     }

//     if dial_pos + change < 0 {
//         return true;
//     }

//     return false;
// }
