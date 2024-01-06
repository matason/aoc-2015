use std::{fs};

fn main() {
    let input = fs::read_to_string("./day01-input.txt").expect("Should be able to read the file");
   println!("{}", run(input))
}

fn run(input: String) -> i32 {
    let up: i32 = input.match_indices("(").count() as i32;
    let down: i32 = input.match_indices(")").count() as i32;
    up - down
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        assert_eq!(3, run(String::from("))(((((")));
    }
}
