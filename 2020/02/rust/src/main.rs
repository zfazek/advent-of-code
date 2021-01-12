use regex::Regex;

fn main() {
    let contents = std::fs::read_to_string("../02.txt").unwrap();
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
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
        .filter_map(|line| {
            let (character, min, max, password) = line;
            if (&min..&(max + 1)).contains(&&password.chars().filter(|&c| c == character).count()) {
                Some(())
            } else {
                None
            }
        })
        .count();
    let counter2 = lines
        .filter_map(|line| {
            let (character, min, max, password) = line;
            if (&password.chars().nth(min - 1).unwrap() == &character)
                != (&password.chars().nth(max - 1).unwrap() == &character)
            {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("{} {}", counter1, counter2);
}

