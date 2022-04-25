use rand::Rng;
use std::fs;
use crate::Sets;

pub fn parse_triangle(input: &str) -> Vec<Vec<u32>> {
    let split = input
        .split(|c: char| c.is_whitespace())
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut i = 0;
    let mut count = 0;
    let mut result = vec![];
    while count < split.len() {
        result.push(vec![]);
        for _ in 0..=i {
            result[i].push(split[count]);
            count += 1;
        }
        i += 1;
    }

    result
}

pub fn generate_sets() -> (i32, Vec<Vec<i32>>) {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(15..100);

    let sets_count = rng.gen_range(n * 1.5 as i32..n * 3) as usize;
    let mut sets = vec![vec![]; sets_count];

    for i in 0..sets_count {
        let size = rng.gen_range(2..5 as i32);
        for _ in 0..size {
            let mut rand = rng.gen_range(0..n);
            while sets[i].contains(&rand) {
                rand = rng.gen_range(0..n);
            }

            sets[i].push(rand);
        }
    }

    (n, sets)
    /*let mut result = format!("{} {}\n", n, sets_count);
    for set in sets {
        result.push_str(&format!("{} ", &set.len().to_string()));
        for num in set {
            result.push_str(&format!("{} ", &num.to_string()))
        }
        result.push('\n');
    }

    fs::write("sets.txt", result);*/
}

pub trait Unite<T> {
    fn unite(&self, other: &Self) -> Self;
}

impl<T: Clone + PartialEq> Unite<Vec<T>> for Vec<T> {
    fn unite(&self, other: &Self) -> Self {
        if self.len() == 0 {
            return other.clone()
        } else if other.len() == 0 {
            return self.clone()
        }

        let mut res = vec![];
        for i in self {
            if !res.contains(i) {
                res.push(i.clone())
            }
        }

        for i in other {
            if !res.contains(i) {
                res.push(i.clone())
            }
        }

        res
    }
}

pub fn fold_sets(sets: &Sets) -> Vec<i32> {
    sets
        .into_iter()
        .fold(
            vec![],
            |acc, elem| acc.unite(&elem)
        )
}

pub fn is_cover(n: i32, sets: &Sets) -> bool {
    fold_sets(sets).len() == n as usize
}

pub fn find_redundant(n: i32, cover: &Sets) -> Option<usize> {
    for i in 0..cover.len() {
        let a = cover
            .into_iter()
            .take(i)
            .fold(
                vec![],
                |acc, elem| acc.unite(elem)
            );

        let b = cover
            .into_iter()
            .skip(i + 1)
            .fold(
                vec![],
                |acc, elem| acc.unite(elem)
            );

        let res = a.unite(&b);
        if res.len() == n as usize {
            return Some(i);
        }
    }

    None
}