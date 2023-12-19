use regex::Regex;
use std::cmp;
use std::collections::BTreeMap;

const N: i32 = 4000;

struct Solution {
    result: Vec<(i32, i32)>,
    ws: BTreeMap<String, Vec<(String, String)>>,
}

impl Solution {
    fn new() -> Self {
        let result = vec![(0, N + 1), (0, N + 1), (0, N + 1), (0, N + 1)];
        let ws = BTreeMap::new();
        Solution { result, ws }
    }

    fn foo(&mut self, input: &str) {
        let (workflows, _ratings) = input.split_once("\n\n").unwrap();
        let re = Regex::new(r"^(.*)\{(.*)}").unwrap();
        for w in workflows.lines() {
            let name = re.captures(w).unwrap().get(1).unwrap().as_str();
            let workflow = re.captures(w).unwrap().get(2).unwrap().as_str();
            let rules = workflow.split(',');
            let mut vs = Vec::new();
            for rule in rules {
                if let Some((cond, target)) = rule.split_once(':') {
                    //println!("{} {}", cond, target);
                    vs.push((cond.to_string(), target.to_string()));
                } else {
                    let cond = "y>0";
                    let target = rule;
                    //println!("{} {}", cond, target);
                    vs.push((cond.to_string(), target.to_string()));
                }
            }
            self.ws.insert(name.to_string(), vs);
        }
        //println!("{:?}", ws);
        let mut ans: i64 = 0;
        let re = Regex::new(r"\{x=(.*),m=(.*),a=(.*),s=(.*)}").unwrap();
        for r in _ratings.lines() {
            let x = re
                .captures(r)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let m = re
                .captures(r)
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let a = re
                .captures(r)
                .unwrap()
                .get(3)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let s = re
                .captures(r)
                .unwrap()
                .get(4)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            //println!("{x} {m} {a} {s}: {}", x + m + a + s);
            let r = self.iter1("in", &[x, m, a, s]);
            if r {
                let sum = x + m + a + s;
                ans += sum as i64;
            }
        }
        println!("{}", ans);
        let mut vs: Vec<String> = Vec::new();
        self.iter2("in", &mut vs);
        println!("{:?}", self.result);
        let mut ans: usize = 1;
        for r in &self.result {
            let sum = 4000.min(r.0 + N - r.1 + 1);
            println!("{sum}");
            ans *= sum as usize;
        }
        println!("{}", ans);
    }

    fn iter2(&mut self, key: &str, rs: &mut Vec<String>) {
        //print!("{key} ");
        if ["A", "R"].contains(&key) {
            if key == "A" {
                println!("{:?} {}", rs, key);
                println!("{:?}", self.result);
            }
            return;
        }
        let rules = self.ws.get(key).unwrap().clone();
        //println!("{key}: {:?}", rules);
        for (cond, target) in &rules {
            let c = cond.chars().next().unwrap();
            let lt = cond.chars().nth(1).unwrap() == '<';
            let v = cond[2..].to_string().parse::<i32>().unwrap();
            let mut idx = 0;
            match c {
                'x' => {
                    idx = 0;
                }
                'm' => {
                    idx = 1;
                }
                'a' => {
                    idx = 2;
                }
                's' => {
                    idx = 3;
                }
                _ => {}
            }
            if c != 'y' {
                if lt {
                    let max = v - 1;
                    self.result[idx].0 = cmp::max(max, self.result[idx].0);
                } else {
                    let min = v + 1;
                    self.result[idx].1 = cmp::min(min, self.result[idx].1);
                }
            }
            rs.push(key.to_string());
            self.iter2(target, rs);
            rs.pop();
        }
    }

    fn iter1(&self, key: &str, nums: &[i32]) -> bool {
        let rules = self.ws.get(key).unwrap();
        //println!("{key}: {:?} {:?}", nums, rules);
        for (cond, target) in rules {
            let c = cond.chars().next().unwrap();
            let lt = cond.chars().nth(1).unwrap() == '<';
            let v = cond[2..].to_string().parse::<i32>().unwrap();
            let mut idx = 0;
            match c {
                'x' => {
                    idx = 0;
                }
                'm' => {
                    idx = 1;
                }
                'a' => {
                    idx = 2;
                }
                's' => {
                    idx = 3;
                }
                _ => {}
            }
            let n = nums[idx];
            if lt && n < v || !lt && n > v {
                if target == "A" {
                    return true;
                }
                if target == "R" {
                    return false;
                }
                return self.iter1(target, nums);
            }
        }
        false
    }
}

fn main() {
    let input = include_str!("../../inputa.txt");
    let mut my = Solution::new();
    my.foo(input);
}
