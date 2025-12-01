use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Div, Rem};
use nom::{Finish, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{value};
use nom::sequence::pair;
use anyhow::{anyhow, Result};
use nom::error::Error;

fn parse_rotation(rot_str: &str) -> Result<i64> {
    let (_, (rot_dir, rot_val)) = pair(
        alt((
            value(-1i32, tag::<&str, &str, Error<_>>("L")),
            value(1i32, tag("R")),
        )),
        nom::character::complete::u32,
    ).parse_complete(rot_str).finish().map_err(|err| anyhow!("Err: {err:?}"))?;

    Ok(rot_dir as i64 * rot_val as i64)
}

fn solve_part1(rot_strs: impl Iterator<Item = String>) -> Result<isize> {
    const DIAL_MAX: isize = 100;

    let mut pos = 50;
    let mut counter = 0;

    for rot in rot_strs.map(|rot_str| parse_rotation(rot_str.as_str())) {
        let rot = (rot? as isize).rem_euclid(DIAL_MAX);
        pos = (pos + rot).rem_euclid(DIAL_MAX);

        counter += if pos == 0 { 1 } else { 0 };
    }

    Ok(counter)
}

fn solve_part2(rot_strs: impl Iterator<Item = String>) -> Result<isize> {
    const DIAL_MAX: isize = 100;

    let mut pos: isize = 50;
    let mut counter: isize = 0;

    for rot_res in rot_strs.map(|rot_str| parse_rotation(rot_str.as_str())) {
        let (rot_k, rot_r) = {
            let unbounded_rot = rot_res? as isize;
            (unbounded_rot.div(DIAL_MAX), unbounded_rot.rem(DIAL_MAX))
        };

        // account for full-revolutions
        counter += rot_k.abs();

        // check wrap-around if applying the rotation remainder in space [1, ..., DIAL_MAX - 1]
        counter += if pos == 0 { 0 } else { (pos - 1 + rot_r).div_euclid(DIAL_MAX - 1).abs() };

        pos = (pos + rot_r).rem_euclid(DIAL_MAX);
    }

    Ok(counter)
}

pub fn part_1(data: File) -> Result<isize> {
    let lines = BufReader::new(data).lines().map(Result::unwrap);
    solve_part1(lines)
}

pub fn part_2(data: File) -> Result<isize> {
    let lines = BufReader::new(data).lines().map(Result::unwrap);
    solve_part2(lines)
}