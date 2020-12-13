fn main() {
    let input = include_str!("../input6.txt").replace("\r\n", "\n");

    let groups: Vec<&str> = input.split("\n\n").collect();

    // part 1
    let answer: usize = groups
        .iter()
        .map(|group| {
            let mut chars = group.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
            chars.sort();
            chars.dedup();
            chars.iter().count()
        })
        .sum();
    println!("{}", answer);

    // part 2
    let answer: usize = groups
        .iter()
        .map(|group| {
            ('a'..='z')
                .filter(|c| group.lines().all(|line| line.contains(*c)))
                .count()
        })
        .sum();
    println!("{}", answer);
}
