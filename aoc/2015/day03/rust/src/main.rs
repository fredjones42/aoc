use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("Input file not found");

    let mut grid: HashMap<(i32, i32), usize> = HashMap::with_capacity(input.len());
    let mut i = 0;
    let mut j = 0;

    grid.insert((i, j), 1);

    for c in input.chars() {
        match c {
            '^' => j += 1,
            'v' => j -= 1,
            '>' => i += 1,
            '<' => i -= 1,
            _ => panic!("unexpected char"),
        }
        let visits = grid.entry((i, j)).or_insert(0);
        *visits += 1;
    }

    println!("{} houses received at least one present.", grid.len());

    grid.clear();

    let mut i = 0;
    let mut j = 0;
    let mut ir = 0;
    let mut jr = 0;

    grid.insert((i, j), 2);

    for (k, c) in input.chars().enumerate() {
        match k % 2 {
            0 => {
                match c {
                    '^' => j += 1,
                    'v' => j -= 1,
                    '>' => i += 1,
                    '<' => i -= 1,
                    _ => panic!("unexpected char"),
                };
                let visits = grid.entry((i, j)).or_insert(0);
                *visits += 1;
            }
            1 => {
                match c {
                    '^' => jr += 1,
                    'v' => jr -= 1,
                    '>' => ir += 1,
                    '<' => ir -= 1,
                    _ => panic!("unexpected char"),
                };
                let visits = grid.entry((ir, jr)).or_insert(0);
                *visits += 1;
            }
            2_usize.. => unreachable!(),
        }
    }

    println!("With Robo-Santa...");
    println!("{} houses received at least one present.", grid.len());
}
