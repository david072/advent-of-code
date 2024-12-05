use std::collections::HashMap;

const INPUT: &str = include_str!("../day5.txt");

fn is_valid_order(update: &str, required_pages: &HashMap<u32, Vec<u32>>) -> Option<u32> {
    let pages = update
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    for (i, p) in pages.iter().enumerate() {
        if !required_pages.contains_key(&p) {
            continue;
        }

        // check that we have already printed the required pages
        if !required_pages[&p]
            .iter()
            .filter(|p| pages.contains(p))
            .all(|p| pages[0..i].contains(p))
        {
            return None;
        }
    }

    Some(pages[pages.len() / 2])
}

fn part1(updates: &str, required_pages: &HashMap<u32, Vec<u32>>) {
    let middle_pages_sum: u32 = updates
        .lines()
        .filter_map(|l| is_valid_order(l, &required_pages))
        .sum();
    println!("middle page sum: {middle_pages_sum}");
}

fn part2(updates: &str, required_pages: &HashMap<u32, Vec<u32>>) {
    let mut middle_pages_sum = 0;
    for update in updates.lines() {
        if is_valid_order(update, required_pages).is_some() {
            continue;
        }

        let mut pages = update
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mut new_pages = vec![];

        /// Recursively insert the required pages (if not already inserted) before inserting `page` into `new_pages`
        fn add(
            page: u32,
            pages: &mut Vec<u32>,
            new_pages: &mut Vec<u32>,
            required_pages: &HashMap<u32, Vec<u32>>,
        ) {
            if !required_pages.contains_key(&page) {
                new_pages.push(page);
                return;
            }

            for rp in &required_pages[&page] {
                if let Some(idx) = pages.iter().position(|p| *p == *rp) {
                    add(pages.remove(idx), pages, new_pages, required_pages);
                }
            }

            new_pages.push(page);
        }

        while !pages.is_empty() {
            add(pages.remove(0), &mut pages, &mut new_pages, required_pages);
        }

        middle_pages_sum += new_pages[new_pages.len() / 2];
    }

    println!("middle page sum: {middle_pages_sum}");
}

fn main() {
    let mut required_pages = HashMap::<u32, Vec<u32>>::new();
    let (rules, updates) = INPUT.split_once("\n\n").unwrap();

    for r in rules.lines() {
        let mut parts = r.split('|').map(|n| n.parse::<u32>().unwrap());
        let (p1, p2) = (parts.next().unwrap(), parts.next().unwrap());
        required_pages
            .entry(p2)
            .and_modify(|v| v.push(p1))
            .or_insert(vec![p1]);
    }

    part1(updates, &required_pages);
    part2(updates, &required_pages);
}
