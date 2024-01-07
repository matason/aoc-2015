use std::fs;

fn main() {
    let input = fs::read_to_string("./day02-input.txt").expect("Should be able to read the file");
   println!("{}", run(input))
}

fn run(input: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        assert_eq!(0, run(String::from("")));
    }
}
