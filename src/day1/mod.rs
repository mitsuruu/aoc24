use counter::Counter;

pub fn part_one() -> i32 {
    let input = include_str!("input.txt");

    let (mut col1, mut col2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok());
            Some((nums.next()?, nums.next()?))
        })
        .unzip();

    col1.sort();
    col2.sort();

    col1.into_iter().zip(col2).map(|l| (l.0 - l.1).abs()).sum()
}

pub fn part_two() -> usize {
    let input = include_str!("input.txt");

    let (col1, col2): (Counter<_>, Counter<_>) = input
        .lines()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok());
            Some((nums.next()?, nums.next()?))
        })
        .unzip();

    col1.into_iter().map(|f| f.0 * f.1 * col2[&f.0]).sum()
}
