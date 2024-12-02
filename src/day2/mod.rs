pub fn part_one() -> usize {
    let input = include_str!("input.txt");

    let reports: Vec<Vec<i32>> = input
        .lines()
        .filter_map(|line| {
            let nums = line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();
            Some(nums)
        })
        .collect();

    let mut num_safe = 0;

    for report in reports {
        if is_safe(&report) {
            num_safe += 1;
        }
    }

    num_safe
}

pub fn part_two() -> usize {
    let input = include_str!("input.txt");

    let reports: Vec<Vec<i32>> = input
        .lines()
        .filter_map(|line| {
            let nums = line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();
            Some(nums)
        })
        .collect();

    reports
        .iter()
        .filter(|report| dampened_is_safe(report))
        .count()
}

fn is_safe(report: &Vec<i32>) -> bool {
    report.is_sorted_by(|a, b| a < b && a.abs_diff(*b) <= 3)
        || report.is_sorted_by(|a, b| a > b && a.abs_diff(*b) <= 3)
}

fn dampened_is_safe(report: &Vec<i32>) -> bool {
    if is_safe(&report) {
        return true;
    }

    for skip in 0..report.len() {
        let levels_iter = &report
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != skip)
            .map(|(_, level)| *level)
            .collect();
        if is_safe(levels_iter) {
            return true;
        }
    }

    false
}
