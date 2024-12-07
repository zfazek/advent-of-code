fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut result1 = 0;
    let mut result2 = 0;
    for line in input.lines() {
        let (value, nums) = line.split_once(": ").unwrap();
        let value = value.parse::<i64>().unwrap();
        let nums = nums
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        result1 += first(value, &nums).signum() * value;
        result2 += second(value, &nums).signum() * value;
    }
    println!("{result1}");
    println!("{result2}");
}

fn first(r: i64, ns: &Vec<i64>) -> i64 {
    if ns[0] > r {
        return 0;
    }
    if ns.len() == 1 {
        if ns[0] == r {
            return 1;
        } else {
            return 0;
        }
    }
    let mut nums1 = ns.clone();
    let mut nums2 = ns.clone();
    nums1[1] = ns[0] + ns[1];
    nums2[1] = ns[0] * ns[1];
    nums1.remove(0);
    nums2.remove(0);
    first(r, &nums1) + first(r, &nums2)
}

fn second(r: i64, ns: &Vec<i64>) -> i64 {
    if ns[0] > r {
        return 0;
    }
    if ns.len() == 1 {
        if ns[0] == r {
            return 1;
        } else {
            return 0;
        }
    }
    let mut nums1 = ns.clone();
    let mut nums2 = ns.clone();
    let mut nums3 = ns.clone();
    nums1[1] = ns[0] + ns[1];
    nums2[1] = ns[0] * ns[1];
    nums3[1] = (ns[0].to_string() + &ns[1].to_string())
        .parse::<i64>()
        .unwrap();
    nums1.remove(0);
    nums2.remove(0);
    nums3.remove(0);
    second(r, &nums1) + second(r, &nums2) + second(r, &nums3)
}
