fn main() {
    let input = include_str!("../input6.txt").replace("\r\n", "\n");

    // part 1
    let answer: usize = input
        .split("\n\n")
        .map(|group| {
            let mut chars = group.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
            chars.sort();
            chars.dedup();
            chars.iter().count()
        })
        .sum();
    println!("{}", answer);
}
