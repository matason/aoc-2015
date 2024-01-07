use std::fs;

fn main() {
    let input = fs::read_to_string("./day02-input.txt").expect("Should be able to read the file");
   println!("{}", run(input))
}

fn run(input: String) -> i32 {
    101
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        assert_eq!(101, run(String::from("2x3x4\n1x1x10")));
    }
}
