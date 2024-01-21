use std::fs;

fn main() {
    let input: String = fs::read_to_string("./day05-input.txt").expect("Should be able to read the file");
    println!("{}", run(input))
}

fn run(input: String) -> usize {
    let filtered = input
        .lines()
        .filter_map(|line| {
            if line.chars().enumerate().any(|c| {
                line.chars().nth(c.0 + 2).is_some_and(|t| t == c.1)
            }
            ) {
                Some(line)
            } else {
                None
            }
        }).filter_map(|line| {
            if line.chars().enumerate().any(|c| {
                line.get(c.0..c.0 + 2).is_some_and(|v| line.matches(v).count() > 1)
            }
            ) {
                Some(line)
            } else {
                None
            }
        });
        
    filtered.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, run(String::from("qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy")));
    }
}
