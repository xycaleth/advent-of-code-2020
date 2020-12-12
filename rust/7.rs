fn main() {
    let input = include_str!("../input7.txt");

    let relations: Vec<(String, String)> = input
        .lines()
        .filter(|line| !line.contains("no other bags"))
        .flat_map(|line| {
            let mut words = line.split_ascii_whitespace();
            let container = format!("{} {}", words.next().unwrap(), words.next().unwrap());

            words.next(); // bags
            words.next(); // contain

            let mut pairs = Vec::new();
            while let Some(count) = words.next() {
                let _count: usize = count.parse().unwrap();
                let contained = format!("{} {}", words.next().unwrap(), words.next().unwrap());

                pairs.push((contained, container.clone()));

                words.next(); // bag(s)
            }

            pairs
        })
        .collect();

    let mut containers: Vec<String> = vec![String::from("shiny gold")];
    loop {
        let new_containers: Vec<String> = containers
            .iter()
            .map(|needle| {
                relations
                    .iter()
                    .filter(|(contained, _)| contained == needle)
                    .map(|(_, container)| container.clone())
                    .collect::<Vec<String>>()
            })
            
            .collect();

        if new_containers.is_empty() {
            break;
        } else {
            containers = new_containers;
        }
    }

    println!("shiny gold is contained by {} containers", containers.len());
}
