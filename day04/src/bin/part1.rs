use::std::str;
use::{md5, md5::Digest};

fn main() {
    let input: &str = "ckczppom";
    println!("{}", run(input))
}

fn run(input: &str) -> i32 {
    let value: String = String::from(input);
    let mut lowest: i32 = 0;

    for n in 0..1000000 {
        let mut seed = value.clone();
        seed.push_str(n.to_string().as_str());

        let digest: Digest = md5::compute(seed);

        if format!("{:x}", digest).starts_with("00000") {
            lowest = n;
            break;
        }
    }

    lowest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(609043, run("abcdef"));
    }
}
