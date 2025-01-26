fn main() {
    let input = std::fs::read_to_string("../05.txt").unwrap();
    let n1 = input.lines().filter(|&line| is_nice1(line)).count();
    let n2 = input.lines().filter(|&line| is_nice2(line)).count();
    println!("{}", n1);
    println!("{}", n2);
}

fn is_nice1(line: &str) -> bool {
    if line.chars().filter(|c| is_vowel(*c)).count() < 3 {
        return false;
    }
    let list: Vec<u8> = line.as_bytes().to_vec();
    let double_letter = (0..list.len() - 1).any(|i| list[i] == list[i + 1]);
    if !double_letter {
        return false;
    }
    if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy") {
        return false;
    }
    return true;
}

fn is_nice2(line: &str) -> bool {
    let list: Vec<u8> = line.as_bytes().to_vec();
    (0..list.len() - 2).any(|i| list[i] == list[i + 2])
        && (0..list.len() - 1).any(|i| line[i + 2..line.len()].contains(&line[i..i + 2]))
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
