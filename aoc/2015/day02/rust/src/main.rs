fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("Input file not found");
    let mut dimensions: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split('x')
            .map(|s| s.parse::<i32>().expect("Expected i32"))
            .collect();
        assert_eq!(nums.len(), 3);
        dimensions.push(nums);
    }

    let mut total = 0;
    for nums in dimensions.iter() {
        total += calc_area(nums);
    }

    println!(
        "The elves should order {} square feet of wrapping paper",
        total
    );

    let mut total = 0;
    for nums in dimensions.iter() {
        total += calc_length(nums);
    }

    println!("The elves should order {} feet of ribbon", total);
}

fn calc_area(nums: &[i32]) -> i32 {
    assert_eq!(nums.len(), 3);
    let l = nums[0];
    let w = nums[1];
    let h = nums[2];
    let lw = l * w;
    let lh = l * h;
    let wh = w * h;
    let min = (lw.min(lh)).min(wh);
    2 * lw + 2 * lh + 2 * wh + min
}

fn calc_length(nums: &[i32]) -> i32 {
    assert_eq!(nums.len(), 3);
    let l = nums[0];
    let w = nums[1];
    let h = nums[2];
    let lw = 2 * l + 2 * w;
    let lh = 2 * l + 2 * h;
    let wh = 2 * w + 2 * h;
    let min = (lw.min(lh)).min(wh);
    let vol = l * w * h;
    min + vol
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_calc() {
        let nums = vec![2, 3, 4];
        assert_eq!(calc_area(&nums), 58);
        let nums = vec![1, 1, 10];
        assert_eq!(calc_area(&nums), 43);
    }

    #[test]
    fn length_calc() {
        let nums = vec![2, 3, 4];
        assert_eq!(calc_length(&nums), 34);
        let nums = vec![1, 1, 10];
        assert_eq!(calc_length(&nums), 14);
    }
}
