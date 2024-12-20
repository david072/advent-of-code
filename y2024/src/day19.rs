const INPUT: &str = include_str!("../day19.txt");

fn is_possible(design: &str, towels: &[&str]) -> bool {
    if design.is_empty() {
        return true;
    }

    towels.iter().any(|towel| {
        if design.starts_with(towel) {
            is_possible(&design[towel.len()..], towels)
        } else {
            false
        }
    })
}

fn part1(designs: &str, towels: &[&str]) {
    let possible_designs = designs
        .trim()
        .lines()
        .filter(|design| is_possible(design, &towels))
        .count();
    println!("possible designs: {possible_designs}");
}

fn part2(designs: &str, towels: &[&str]) {
    #[derive(Default)]
    struct TrieNode {
        end_of_word: bool,
        // 5 possible characters in the alphabet
        children: [Option<Box<TrieNode>>; 5],
    }

    fn color_to_index(c: char) -> usize {
        match c {
            'w' => 0,
            'u' => 1,
            'b' => 2,
            'r' => 3,
            'g' => 4,
            _ => unreachable!(),
        }
    }

    impl TrieNode {
        fn insert(&mut self, s: &str) {
            if s.is_empty() {
                self.end_of_word = true;
                return;
            }

            let idx = color_to_index(s.chars().next().unwrap());
            if self.children[idx].is_none() {
                self.children[idx] = Some(Box::new(TrieNode::default()));
            }

            self.children[idx].as_mut().unwrap().insert(&s[1..]);
        }
    }

    let mut root = Box::new(TrieNode::default());
    for towel in towels {
        let s = towel.chars().rev().collect::<String>();
        root.insert(&s);
    }

    let count = designs
        .trim()
        .lines()
        .map(|design| {
            // list of possible ways to make each substring 0..i, 0 <= i < design.len() of design
            let mut counts = vec![0usize; design.len()];
            let design = design.chars().collect::<Vec<_>>();
            // go through all substrings 0..i, 0 <= i < design.len() of design
            for i in 0..design.len() {
                let mut node = &root;
                // traverse the trie, going backwards through the substring
                for j in (0..=i).rev() {
                    if let Some(child) = &node.children[color_to_index(design[j])] {
                        node = child;
                    } else {
                        break;
                    }

                    // if we found a word in the trie, either
                    // 1. the entire substring is in the trie, in which case we found another way to
                    //    make this substring
                    // 2. or we have a part of the end of the substring in the trie, in which case
                    //    add the possible ways to make the string up to the start of the current
                    //    substring. Don't add 1 here, as we haven't found a new way to make the
                    //    substring and are just continuing the possible ways from the prior
                    //    substring with the word we found in the trie.
                    if node.end_of_word {
                        counts[i] += if j > 0 { counts[j - 1] } else { 1 };
                    }
                }
            }

            // the number of possible ways to make this design is how many ways there are to make the
            // substring 0..design.len()
            counts[design.len() - 1]
        })
        .sum::<usize>();
    println!("number of possible ways: {count}");
}

fn main() {
    let (towels, designs) = INPUT.split_once("\n\n").unwrap();
    let towels = towels.trim().split(", ").collect::<Vec<_>>();

    part1(designs, &towels);
    part2(designs, &towels);
}
