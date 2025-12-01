seq!(N in 1..=1 {
   mod day_~N;
});

use seq_macro::seq;
use std::str::FromStr;
use std::time::Instant;
use std::{env, fs};

fn main() {
    let day = env::args()
        .nth(1)
        .map(|day_str| usize::from_str(day_str.as_str()).unwrap());

    seq!(N in 1..=1 {
        if day.is_none_or(|day| day == N) {
            let now = Instant::now();
            let sol_1 = day_~N::part_1(fs::File::open(concat!("./resources/day_", stringify!(N), "_data.txt")).unwrap()).unwrap();
            let sol_2 = day_~N::part_2(fs::File::open(concat!("./resources/day_", stringify!(N), "_data.txt")).unwrap()).unwrap();
            println!(
                concat!("Day ", stringify!(N), ": Part 1 = {}; Part 2 = {}; Took {:.2}ms;"),
                sol_1, sol_2, now.elapsed().as_secs_f32() * 1000.0f32
            );
        }
    });
}
