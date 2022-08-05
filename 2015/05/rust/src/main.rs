use substring::Substring;

fn main() {
    let input = std::fs::read_to_string("../05.txt").unwrap();
    let mut n1:i64 = 0;
    let mut n2:i64 = 0;
    for line in input.lines() {
        if is_nice1(&line) {
            n1 += 1;
        }
        if is_nice2(&line) {
            n2 += 1;
        }
    }
    println!("{}", n1);
    println!("{}", n2);
}


fn is_nice1(line: &str) -> bool {
    let number_of_vowels = line.chars().filter(|c| is_vowel(*c)).count();
    if number_of_vowels < 3 {
        return false;
    }
    let mut double_letter = false;
    let list: Vec<u8> = line.as_bytes().to_vec();
    for i in 0..list.len() - 1 {
        let c1 = list[i];
        let c2 = list[i + 1];
        if c1 == c2 {
            double_letter = true;
            break;
        }
    }
    if !double_letter {
        return false;
    }
    if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy") {
        return false;
    }
    return true;
}

fn is_nice2(line: &str) -> bool {
    let mut first = false;
    let list: Vec<u8> = line.as_bytes().to_vec();
    for i in 0..list.len() - 1 {
        let s = line.substring(i, i + 2);
        if line.substring(i + 2, line.len()).contains(s) {
            first = true;
            break;
        }
    }
    let mut second = false;
    for i in 0..list.len() - 2 {
        let c1 = list[i];
        let c2 = list[i + 2];
        if c1 == c2 {
            second = true;
            break;
        }
    }
    return first && second;
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
