use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../input.txt");
    let mut boxes = Vec::new();
    for line in input.lines() {
        let mut tokens = line.split(',');
        let x = tokens.next().unwrap().parse::<i64>().unwrap();
        let y = tokens.next().unwrap().parse::<i64>().unwrap();
        let z = tokens.next().unwrap().parse::<i64>().unwrap();
        boxes.push((x, y, z));
    }
    let mut pairs = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let a = boxes[i];
            let b = boxes[j];
            let d =
                (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1) + (a.2 - b.2) * (a.2 - b.2);
            pairs.push((d, a, b));
        }
    }
    pairs.sort();
    let mut circuits: Vec<BTreeSet<(i64, i64, i64)>> = Vec::new();
    for i in 0..pairs.len() {
        let (_, a, b) = pairs[i];
        if let Some(idx1) = find(&circuits, &a)
            && let Some(idx2) = find(&circuits, &b)
            && idx1 != idx2
        {
            circuits[idx1] = circuits[idx1].union(&circuits[idx2]).cloned().collect();
            circuits.remove(idx2);
        } else if let Some(idx) = find(&circuits, &a) {
            circuits[idx].insert(b);
        } else if let Some(idx) = find(&circuits, &b) {
            circuits[idx].insert(a);
        } else {
            let mut boxes = BTreeSet::new();
            boxes.insert(a);
            boxes.insert(b);
            circuits.push(boxes);
        }
        if i == 999 {
            let mut cc = circuits.iter().map(|x| x.len()).collect::<Vec<_>>();
            cc.sort_by(|a, b| b.cmp(a));
            let result1 = cc.iter().take(3).product::<usize>();
            println!("{result1}");
        }
        if circuits.len() == 1 && circuits[0].len() == boxes.len() {
            println!("{}", pairs[i].1.0 * pairs[i].2.0);
            break;
        }
    }
}

fn find(circuits: &[BTreeSet<(i64, i64, i64)>], p: &(i64, i64, i64)) -> Option<usize> {
    (0..circuits.len()).find(|&i| circuits[i].contains(p))
}
