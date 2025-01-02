fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("could not read file");
    println!("{}", calc_floor(&input));
    println!("{}", calc_position(&input));
}

fn calc_floor(input: &str) -> i32 {
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

fn calc_position(input: &str) -> usize {
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
