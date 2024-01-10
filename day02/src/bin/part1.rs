use std::fs;

fn main() {
    let input: String = fs::read_to_string("./day02-input.txt").expect("Should be able to read the file");
    println!("{}", run(input))
}

fn run(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut d: Vec<_> = line
                .split('x')
                .map(|value| {
                    value.parse::<i32>().expect("Should be a parseable i32")
                })
                .collect();
            d.sort();

            (d[0] * d[1]) + (2 * (d[0] * d[1])) + (2 * (d[1] * d[2])) + (2 * (d[0] * d[2]))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        assert_eq!(101, run(String::from("2x3x4\n1x1x10")));
    }
}
