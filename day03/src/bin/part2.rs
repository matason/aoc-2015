use std::fs;

fn main() {
    let input: String = fs::read_to_string("./day03-input.txt").expect("Should be able to read the file");
    println!("{}", run(input))
}

fn run(_input: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, run(String::from("")));
    }
}
