use std::collections::BTreeSet;

fn main() {
    let boxes = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let mut tokens = line.split(',');
            let x = tokens.next().unwrap().parse::<i64>().unwrap();
            let y = tokens.next().unwrap().parse::<i64>().unwrap();
            let z = tokens.next().unwrap().parse::<i64>().unwrap();
            (x, y, z)
        })
        .collect::<Vec<_>>();
    let mut pairs = Vec::new();
    for (i, &a) in boxes.iter().enumerate() {
        for &b in boxes.iter().skip(i + 1) {
            pairs.push((
                (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1) + (a.2 - b.2) * (a.2 - b.2),
                a,
                b,
            ));
        }
    }
    pairs.sort();
    let mut circuits: Vec<BTreeSet<(i64, i64, i64)>> = boxes
        .iter()
        .map(|p| [*p].into_iter().collect::<BTreeSet<_>>())
        .collect();
    let target = if boxes.len() == 20 { 9 } else { 999 };
    for (i, &p) in pairs.iter().enumerate() {
        let (_, a, b) = p;
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
        }
        if i == target {
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
