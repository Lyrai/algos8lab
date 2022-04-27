use std::cmp::max;
use std::fs;
use rand::Rng;
use crate::misc::*;
use crate::cover::Cover;

type Sets = Vec<Vec<i32>>;

mod misc;
mod item;
mod cover;

fn main() {
    let t = fs::read_to_string("triangle.txt").unwrap();
    let r = parse_triangle(&t);
    println!("Max sum is {}", triangle_max_sum(r));
    let mut sets = generate_sets();
    let mut cover = greedy_cover(sets.0, &sets.1);
    while let None = cover {
        sets = generate_sets();
        cover = greedy_cover(sets.0, &sets.1);
    }

    let (n, sets) = sets;

    if let Some(cover) = cover {
        let len = cover.len();
        let mut cover_union = fold_sets(&cover);
        cover_union.sort();
        println!("n: {}, len: {}, {:?}", n, len, cover_union);
        let genetic = genetic(n, cover, sets);
        let mut genetic_folded = fold_sets(&genetic);
        genetic_folded.sort();
        println!("n: {}, len: {}, {:?}", n, genetic.len(), genetic_folded);
    } else {
        println!("No cover");
    }

    println!("Knapsack max cost {}", knapsack(150));
}

fn triangle_max_sum(mut triangle: Vec<Vec<u32>>) -> u32 {
    let n = triangle.len();

    for i in (0..=n - 2).rev() {
        for j in 0..=i {
            triangle[i][j] += max(triangle[i + 1][j], triangle[i + 1][j + 1]);
        }
    }

    triangle[0][0]
}

fn greedy_cover(n: i32, sets: &Sets) -> Option<Sets> {
    let mut result = Cover {
        union: vec![],
        sets: vec![]
    };
    let n = n as usize;

    let mut i = 0;
    while result.union.len() < n {
        let mut union = vec![];
        let mut max = result.union.len();
        let mut flag = false;
        let mut max_set = &vec![];
        for set in sets {
            let u = result.union.unite(&set);
            if u.len() > max {
                max = u.len();
                union = u;
                max_set = set;
                flag = true;
            }
        }

        if !flag {
            return None;
        }

        result.union = union;
        result.sets.push(max_set.to_vec());
        i += 1;
    }

    Some(result.sets)
}

fn genetic(n: i32, mut approx: Sets, sets: Sets) -> Sets {
    let mut rand = rand::thread_rng();
    let mut ended = false;

    while !ended {
        ended = true;

        //fill
        let mut population = {
            let mut res = vec![];
            for _ in 0..50 {
                res.push(approx.clone())
            }

            res
        };

        //mutate
        for specie in population.iter_mut() {
            let len = specie.len();
            let idx1 = rand.gen_range(0..len);
            let idx2 = rand.gen_range(0..sets.len());
            specie[idx1] = sets[idx2].clone();
        }

        //fit
        for mut specie in population {
            if is_cover(n, &specie) {
                if let Some(idx) = find_redundant(n, &specie) {
                    specie.remove(idx);
                    approx = specie;
                    ended = false;
                    println!("Reduced len");
                    break;
                } else {
                    continue;
                }
            }
        }
    };

    approx
}

fn knapsack(max_capacity: usize) -> u32 {
    let weights = vec![
        7,  0,  30, 22, 80, 94, 11, 81, 70, 64, 59, 18, 0,  36, 3,  8,  15,
        42, 9,  0,  42, 47, 52, 32, 26, 48, 55, 6,  29, 84, 2,  4,  18, 56,
        7,  29, 93, 44, 71, 3,  86, 66, 31, 65, 0,  79, 20, 65, 52, 13
    ];
    let costs = vec![
        360, 83, 59,  130, 431, 67, 230, 52,  93,  125, 670, 892, 600,
        38,  48, 147, 78,  256, 63, 17,  120, 164, 432, 35,  92,  110,
        22,  42, 50,  323, 514, 28, 87,  73,  78,  15,  26,  78,  210,
        36,  85, 189, 274, 43,  33, 10,  19,  389, 276, 312
    ];

    //result[0][..] == 0
    //result[..][0] == 0
    let mut result = vec![vec![0u32; max_capacity + 1]; weights.len() + 1];

    for i in 0..weights.len() {
        for j in 1..=max_capacity {
            if weights[i] > j {
                result[i + 1][j] = result[i][j];
            }
            else
            {
                let prev = result[i][j];
                let remaining_cost = costs[i] + result[i][j - weights[i]];
                result[i + 1][j] = max(prev, remaining_cost);
            }
        }
    }

    *result
        .last()
        .unwrap()
        .last()
        .unwrap()
}