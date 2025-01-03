fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("Input file not found");

    let mut nice = 0;
    let mut nice_v2 = 0;

    for line in input.lines() {
        if is_nice(line) {
            nice += 1;
        }
        if is_nice_v2(line) {
            nice_v2 += 1;
        }
    }

    println!("How many strings are nice? {}", nice);
    println!("   Version Two...          {}", nice_v2);
}

fn is_nice_v2(line: &str) -> bool {
    non_overlapping_pair(line) && repeating_letter_one_separator(line)
}

fn non_overlapping_pair(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() - 2 {
        let a = chars[i];
        let b = chars[i + 1];
        for j in i + 2..chars.len() - 1 {
            let c = chars[j];
            let d = chars[j + 1];
            if a == c && b == d {
                return true;
            }
        }
    }
    false
}

fn repeating_letter_one_separator(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() - 2 {
        let a = chars[i];
        let z = chars[i + 2];
        if a == z {
            return true;
        }
    }
    false
}

fn is_nice(line: &str) -> bool {
    assert!(line.is_ascii());
    if vowel_count(line) < 3 {
        return false;
    }
    if !double_letter(line) {
        return false;
    }
    if forbidden_pair(line) {
        return false;
    }
    true
}

fn vowel_count(line: &str) -> usize {
    let mut n = 0;
    for c in line.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => n += 1,
            _ => {}
        }
    }
    n
}

fn double_letter(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

fn forbidden_pair(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() - 1 {
        let a = chars[i];
        let b = chars[i + 1];
        if a == 'a' && b == 'b' {
            return true;
        }
        if a == 'c' && b == 'd' {
            return true;
        }
        if a == 'p' && b == 'q' {
            return true;
        }
        if a == 'x' && b == 'y' {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_one() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn version_two() {
        assert!(is_nice_v2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_v2("xxyxx"));
        assert!(!is_nice_v2("uurcxstgmygtbstg"));
        assert!(!is_nice_v2("ieodomkazucvgmuy"));
    }
}
