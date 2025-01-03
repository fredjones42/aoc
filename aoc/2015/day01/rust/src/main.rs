fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("could not read file");
    println!("Santa goes to floor {}", count_open_paren(&input));
    println!("Santa enters basement at {}", pos_extra_rparen(&input));
}

fn count_open_paren(input: &str) -> i32 {
    let mut n = 0;
    for c in input.chars() {
        match c {
            '(' => n += 1,
            ')' => n -= 1,
            _ => panic!("unexpected char"),
        }
    }
    n
}

fn pos_extra_rparen(input: &str) -> usize {
    let mut n: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => n += 1,
            ')' => n -= 1,
            _ => panic!("unexpected char"),
        }
        if n == -1 {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_paren_count() {
        assert_eq!(count_open_paren("(())"), 0);
        assert_eq!(count_open_paren("()()"), 0);
        assert_eq!(count_open_paren("((("), 3);
        assert_eq!(count_open_paren("(()(()("), 3);
        assert_eq!(count_open_paren("))((((("), 3);
        assert_eq!(count_open_paren("())"), -1);
        assert_eq!(count_open_paren("))("), -1);
        assert_eq!(count_open_paren(")))"), -3);
        assert_eq!(count_open_paren(")())())"), -3);
    }

    #[test]
    fn extra_rparen_pos() {
        assert_eq!(pos_extra_rparen(")"), 1);
        assert_eq!(pos_extra_rparen("()())"), 5);
    }
}
