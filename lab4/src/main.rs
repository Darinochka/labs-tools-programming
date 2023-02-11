// 4 2 6 4 2
// 1 Формула {x: t[a, b], x=K*tn} (n = 1 или 2)
//
// 2 Случайным образом (случайным образом выбираются индексы
// универсального множества)
//
// 3 Элементы, которые кратны K1 или K2
//
// 4  /(AvB) и /A|\B
//
// 5 Четные по значению элементы
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let n: i32 = rng.gen_range(1..3);
    let k: i32 = rng.gen_range(1..10);
    let t = 4..40;

    println!("1 задание (4)");

    let universe: HashSet<i32> = t
        .map(|x| k * x^n)
        .collect();

    dbg!(&universe);

    println!("2 задание (2)");
    
    let a: HashSet<i32> = (0..20)
        .map(|_| *(universe.iter().choose(&mut rng).unwrap()))
        .collect();

    dbg!(&a);

    let k1: i32 = rng.gen_range(1..10);
    let k2: i32 = rng.gen_range(1..10);
    println!("3 задание (6)\nk1 = {}, k2 = {}", k1, k2);

    let b: HashSet<i32> = universe
        .iter()
        .map(|&x| x)
        .filter(|x| x & k1 == 0 && x & k2 == 0)
        .collect();

    dbg!(&b);

    println!("4 задание (4)");

    let union_ab: HashSet<i32> = a.union(&b)
        .map(|&x| x)
        .collect();

    let diff_union_ab: HashSet<&i32> = universe.difference(&union_ab).collect();

    let diff_a: HashSet<&i32> = universe.difference(&a).collect();
    let diff_b: HashSet<&i32> = universe.difference(&b).collect();

    let inter_diff_ab: HashSet<&i32> = diff_a.intersection(&diff_b)
        .map(|&x| x)
        .collect();

    dbg!(&inter_diff_ab);
    assert_eq!(diff_union_ab, inter_diff_ab);
}
