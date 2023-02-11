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


fn main() {
    let mut rng = thread_rng();
    let n: i32 = rng.gen_range(1..3);
    let k: i32 = rng.gen_range(1..10);
    let t = 4..40;
    let universe: HashSet<i32> = t
        .map(|x| k * x^n)
        .collect();

    println!("1 задание (4)");
    dbg!(&universe);

    let a: HashSet<i32> = (0..20)
        .map(|_| rng.choose(&universe))
        .collect();

    println!("2 задание (2)");
    dbg!(&a);



        
}
