use std::cmp;

pub fn min_of_three(a: i32, b: i32, c: i32) -> i32 {
    cmp::min(a, cmp::min(b, c))
}