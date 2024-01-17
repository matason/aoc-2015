use std::fs;

fn main() {
    let input: String = fs::read_to_string("./day05-input.txt").expect("Should be able to read the file");
    println!("{}", run(input))
}

fn run(input: String) -> usize {
    input
        .lines()
        .filter_map(|line| {
            if (line.contains("ab") == false && line.contains("cd") == false && line.contains("pq") == false && line.contains("xy") == false) &&
                (line.contains("aa") || line.contains("bb") || line.contains("cc") || line.contains("dd") ||
                line.contains("ee") || line.contains("ff") || line.contains("gg") || line.contains("hh") ||
                line.contains("ii") || line.contains("jj") || line.contains("kk") || line.contains("ll") ||
                line.contains("mm") || line.contains("nn") || line.contains("oo") || line.contains("pp") ||
                line.contains("qq") || line.contains("rr") || line.contains("ss") || line.contains("tt") ||
                line.contains("uu") || line.contains("vv") || line.contains("ww") || line.contains("xx") ||
                line.contains("yy") || line.contains("zz")) &&
                (line.replace("a", "").as_str()
                    .replace("e", "").as_str()
                    .replace("i", "").as_str()
                    .replace("o", "").as_str()
                    .replace("u", "").as_str()
                    .len() + 3 <= line.len())
            {
                Some(line)
            } else {
                None
            }
        }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_words() {
        assert_eq!(2, run(String::from("ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb")));
    }
}
