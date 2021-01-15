fn main() {
    let contents = std::fs::read_to_string("../02.txt").unwrap();
    let re = regex::Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let lines = contents.lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let min = cap[1].parse::<usize>().unwrap();
        let max = cap[2].parse::<usize>().unwrap();
        let character = cap[3].chars().next().unwrap();
        let password = cap[4].to_string();
        (character, min, max, password)
    });
    let counter1 = lines
        .clone()
        .filter(|line| {
            let (character, min, max, password) = line;
            (min..=max).contains(&&password.chars().filter(|c| c == character).count())
        })
        .count();
    let counter2 = lines
        .filter(|line| {
            let (character, min, max, password) = line;
            (&password.chars().nth(min - 1).unwrap() == character)
                != (&password.chars().nth(max - 1).unwrap() == character)
        })
        .count();
    println!("{} {}", counter1, counter2);
}
