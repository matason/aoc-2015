use std::fs;
use regex::Regex;
// struct Light {
//     x: u8,
//     y: u8,
//     on: bool,
// }

fn main() {
    let input: String = fs::read_to_string("./day06-input.txt").expect("Should be able to read the file");
    println!("{}", run(input))
}

// 0,0     -------     0,999
// 999,0   -------   999,999
fn run(input: String) -> usize {
    let mut lights: Vec<u32> = vec![0; 1000000];

    let _: Vec<_> = input
        .lines()
        .map(|line| {
            let re =  Regex::new(r"(\d+),{1}(\d+) through (\d+),{1}(\d+)").unwrap();
            let matches = re.captures(line).unwrap();

            let start_x = matches.get(3).unwrap().as_str().parse::<usize>();
            println!("{:#?}", start_x);

            if line.starts_with("turn on") {
                lights.splice(0..999, vec![1; 1000]);
            }
            else if line.starts_with("turn off") {

            }
            else if line.starts_with("toggle") {

            }
        }).collect();

    lights.iter().filter(|light| **light == 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(2, run(String::from("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500")));
        assert_eq!(1000000, run(String::from("turn on 0,0 through 999,999")));
    }
}
