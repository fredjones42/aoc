fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("Input file not found");

    let answer = find_answer_5(&input);
    println!("The answer to Part One is: {}", answer);

    let answer = find_answer_6(&input);
    println!("The answer to Part Two is: {}", answer);
}

fn find_answer_5(secret_key: &str) -> i32 {
    let mut n = 0;
    loop {
        let s = format!("{}{}", secret_key, n);
        let digest = md5::compute(s.as_bytes());
        if format!("{:x}", digest)[..5] == *"00000" {
            return n;
        }
        n += 1;
    }
}

fn find_answer_6(secret_key: &str) -> i32 {
    let mut n = 0;
    loop {
        let s = format!("{}{}", secret_key, n);
        let digest = md5::compute(s.as_bytes());
        if format!("{:x}", digest)[..6] == *"000000" {
            return n;
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(find_answer_5("abcdef"), 609043);
        assert_eq!(find_answer_5("pqrstuv"), 1048970);
    }
}
