use std::collections::HashMap;

const DAY1: &str = include_str!("../day1.txt");

fn part1(mut col1: Vec<i32>, mut col2: Vec<i32>) {
    col1.sort();
    col2.sort();

    let dist: i32 = col1
        .iter()
        .zip(col2.iter())
        .map(|(n1, n2)| (n1 - n2).abs())
        .sum();
    println!("Distance: {dist}");
}

fn part2(mut col1: Vec<i32>, mut col2: Vec<i32>) {
    fn count(v: &[i32]) -> HashMap<i32, i32> {
        let mut res = HashMap::new();
        for el in v {
            res.entry(*el).and_modify(|c| *c += 1).or_insert(1);
        }
        res
    }

    let col1 = count(&col1);
    let col2 = count(&col2);

    let similarity: i32 = col1
        .into_iter()
        .map(|(k, v)| v * k * *col2.get(&k).unwrap_or(&0))
        .sum();
    println!("Similarity: {similarity}");
}

fn main() {
    let (col1, col2) = DAY1
        .lines()
        .map(|l| {
            let mut parts = l.split("   ").map(|n| n.parse::<i32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect::<(Vec<i32>, Vec<i32>)>();
    part1(col1.clone(), col2.clone());
    part2(col1, col2);
}
