use std::{fs, collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

fn main() {
    let input: String = fs::read_to_string("./day03-input.txt").expect("Should be able to read the file");
    println!("{}", run(input))
}

fn run(input: String) -> i32 {
    let start: Point = Point::new(0, 0);
    let mut visited: HashMap<Point, i32> = HashMap::from([(start, 1)]);

    let _moves: Point = input.chars().fold(start, |location: Point, direction: char| {
        let next: Point = match direction {
            '^' => Point::new(location.x, location.y + 1),
            '>' => Point::new(location.x + 1, location.y),
            'v' => Point::new(location.x, location.y - 1),
            '<' => Point::new(location.x - 1, location.y),
            _ => Point::new(0, 0), // Hmmm...
        };

        match visited.get(&next) {
            Some(count) => { visited.insert(next, count + 1); }
            None => { visited.insert(next, 1); }
        }

        next.clone()
    });

    visited.iter().count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, run(String::from("^v^v^v^v^v")));
    }
}
