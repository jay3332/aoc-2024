#![feature(never_type)]
#![allow(unused_variables)]

use std::collections::HashMap;

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &'static str) -> Output<u32> {
    let mut lines = input.lines();
    let mut a = Vec::<u32>::with_capacity(lines.clone().count());
    let mut b = Vec::<u32>::with_capacity(lines.count());

    for line in input.lines() {
        let (first, second) = line.split_once("   ").unwrap();
        a.push(first.parse().unwrap());
        b.push(second.parse().unwrap());
    }

    a.sort_unstable();
    b.sort_unstable();

    Ok(a.into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum())
}

pub fn part_2(input: &'static str) -> Output<u32> {
    let mut lines = input.lines();
    let mut a = Vec::<u32>::with_capacity(lines.clone().count());
    let mut b = HashMap::<u32, u32>::with_capacity(lines.count());

    for line in input.lines() {
        let (first, second) = line.split_once("   ").unwrap();
        a.push(first.parse().unwrap());
        *b.entry(second.parse().unwrap()).or_insert(0) += 1;
    }

    Ok(a.into_iter().map(|c| c * *b.get(&c).unwrap_or(&0)).sum())
}
