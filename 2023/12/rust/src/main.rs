const N: i32 = 1;

fn main() {
    let input = include_str!("../../inputa.txt");
    let mut ans1: usize = 0;
    for line in input.lines() {
        let mut vs = Vec::new();
        let (record, groups) = line.split_once(" ").unwrap();
        println!("{}", record);
        let gso = groups
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut gs = Vec::new();
        for _ in 0..N {
            for &g in gso.iter() {
                gs.push(g);
            }
        }
        let recordo = record.replace('.', " ");
        let mut record: String = "".to_string();
        for i in 0..N {
            for c in recordo.chars() {
                record.push(c);
            }
            if i < N - 1 {
                record.push('?');
            }
        }
        println!("{}", record);
        println!("{:?}", gs);
        let mut l = Vec::new();
        for c in record.chars() {
            if c != '?' {
                if vs.is_empty() {
                    l.push(c);
                    vs.push(l.clone());
                } else {
                    for v in vs.iter_mut() {
                        v.push(c);
                    }
                }
            } else {
                if vs.is_empty() {
                    let mut l1 = l.clone();
                    l.push(' ');
                    l1.push('#');
                    vs.push(l.clone());
                    vs.push(l1);
                } else {
                    let mut vs1 = Vec::new();
                    for v in vs.iter_mut() {
                        let mut l1 = v.clone();
                        v.push(' ');
                        l1.push('#');
                        vs1.push(l1);
                    }
                    for v in vs1 {
                        vs.push(v);
                    }
                }
            }
        }
        //println!("{:?}", vs);
        println!("{}", vs.len());
        'foo: for v in vs {
            let s: String = v.into_iter().collect();
            //println!("{}", s);
            let ss = s.split_ascii_whitespace().collect::<Vec<_>>();
            //println!("{:?}", ss);
            if ss.len() != gs.len() {
                continue;
            }
            for i in 0..ss.len() {
                if ss[i].len() != gs[i] {
                    continue 'foo;
                }
            }
            ans1 += 1;
        }
        println!("{}", ans1);
    }
    println!("{}", ans1);
}
