use std::collections::HashMap;

fn main() {
    let mut ans1: usize = 0;
    let input = include_str!("../../input.txt");
    let mut hashtable: HashMap<usize, Vec<(&str, &str)>> = HashMap::new();
    'foo: for token in input.trim().split(',') {
        let ans = hash(token);
        ans1 += ans;
        if token.contains('-') {
            let t = &token[..token.len() - 1];
            let key = hash(t);
            if let Some(vs) = hashtable.get_mut(&key) {
                let mut idx = 0;
                let mut found = false;
                for i in 0..vs.len() {
                    if t == vs[i].0 {
                        idx = i;
                        found = true;
                        break;
                    }
                }
                if found {
                    vs.remove(idx);
                }
            }
        } else {
            let (a, b) = token.split_once('=').unwrap();
            let key = hash(a);
            if let Some(vs) = hashtable.get_mut(&key) {
                for v in vs.iter_mut() {
                    if a == v.0 {
                        *v = (a, b);
                        continue 'foo;
                    }
                }
                vs.push((a, b));
            } else {
                hashtable.insert(key, vec![(a, b)]);
            }
        }
    }
    //println!("{:?}", ht);
    println!("{}", ans1);
    let mut ans2: usize = 0;
    for e in hashtable {
        for (i, v) in e.1.iter().enumerate() {
            ans2 += (e.0 + 1) * (i + 1) * v.1.parse::<usize>().unwrap();
        }
    }
    println!("{}", ans2);
}

fn hash(s: &str) -> usize {
    let mut ans = 0;
    for c in s.chars() {
        let a = c as usize;
        ans += a;
        ans *= 17;
        ans %= 256;
    }
    ans
}
