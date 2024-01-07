use std::fs;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

fn main() {
    let input = fs::read_to_string("./day01-input.txt").expect("Should be able to read the file");
    println!("{}", run(input))
}

fn run(input: String) -> i32 {
    return input.chars().enumerate().fold_while(0, |mut acc, c| {
        if '(' == c.1 { acc += 1 } else if ')' == c.1 { acc -= 1 } else { acc += 0 }
        if acc < 0 { Done(c.0 as i32 + 1) } else { Continue(acc) }
    }).into_inner();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        assert_eq!(5, run(String::from("()())")));
    }
}
