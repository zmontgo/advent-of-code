use crate::read_file;

pub fn day_two() {
    let input = read_file("src/dec_2/input.txt".to_string());
    let input = input.trim();

    let mut safe_count: u64 = 0;
    let mut dampened_safe_count: u64 = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|el| el.parse().unwrap())
            .collect();

        if is_safe(&levels) {
            safe_count += 1;
            dampened_safe_count += 1;
        } else if is_dampened_safe(&levels) {
            dampened_safe_count += 1;
        }
    }

    println!("Safe count: {}", safe_count);
    println!("Dampened safe count: {}", dampened_safe_count);
}

fn is_safe(levels: &[i32]) -> bool {
    let mut increasing = None; // None until we determine direction

    for level in levels.windows(2) {
        let diff = level[1] - level[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        match increasing {
            Some(true) if diff < 0 => return false, // Was increasing, but now decreasing
            Some(false) if diff > 0 => return false, // Was decreasing, but now increasing
            None => increasing = Some(diff > 0),    // Determine direction
            _ => {}
        }
    }

    true
}

fn is_dampened_safe(levels: &[i32]) -> bool {
    // Remove the ith element
    for i in 0..levels.len() {
        // This avoids cloning the entire vector only to then make the O(n-i) operation for the call to remove
        let new_levels = &levels[..i]
            .iter()
            .chain(&levels[i + 1..])
            .copied()
            .collect::<Vec<_>>();

        if is_safe(new_levels) {
            return true;
        }
    }

    false
}
