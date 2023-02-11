use std::vec::Vec;
use rand::Rng;
use std::time::Instant;
use std::process::exit;

fn alg_with_sort(vec: &Vec<i32>) -> i32 {
    let mut vec_new: Vec<i32> = vec.to_vec();
    vec_new.sort();

    return vec_new[0] + vec_new[1];
}

fn alg_with_iter(vec: &Vec<i32>) -> Option<i32> {
    if vec.len() < 2 {
        return None;
    }

    let mut idx_min1: usize = 0;
    let mut idx_min2: usize = 1;

    for i in 2..vec.len() {
        if vec[i] < vec[idx_min1] {
            idx_min2 = idx_min1;
            idx_min1 = i;
        }
        else if vec[i] < vec[idx_min2] {
            idx_min2 = i;
        }
    }

    return Some(vec[idx_min1] + vec[idx_min2]);
}

fn find_max_positive(vec: &Vec<i32>) -> Option<&i32> {    
    let result: Option<&i32> = vec
        .iter()
        .filter(|p| p.is_positive())
        .max();
    
    return result;
}

fn divide_num_half(vec: &Vec<i32>, n: usize, m: usize) -> Vec<i32> {
    let mut vec_new: Vec<i32> = vec.to_vec();
    for i in n..m {
        vec_new[i] /= 2;
    }

    return vec_new;
}

fn find_sum_btw_neg(vec: &Vec<i32>) -> Option<i32> {
    let mut first: bool = true;
    let mut idx_left: usize = 0;
    let mut idx_right: usize = 0;
    let mut result: i32 = 0;

    for (i, el) in vec.iter().enumerate() {
        if *el < 0 {
            if first {
                first = false;
                idx_left = i;
            }
            else {
                idx_right = i;
            }
        }
    }

    if idx_left < idx_right {
        for i in idx_left+1..idx_right {
            result += vec[i];
        }

        return Some(result);
    }
    else {
        return None;
    } 
}

fn sort_first_half(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec.to_vec();
    let half: usize = vec.len() / 2;
    new_vec[..half].sort_by(|a, b| b.cmp(a));

    return new_vec;
}

fn custom_order(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec.to_vec();
    let mut left: usize = 0;
    let mut right: usize = 0;
 
    while right < new_vec.len() {
        if new_vec[right] != 0 {
            new_vec[left] = new_vec[right];
            left += 1;
        }
        right += 1;
    }
    
    while left < new_vec.len() {
        new_vec[left] = 0;
        left += 1;
    }

    return new_vec;
}

fn delete_el_less_k(vec: &Vec<i32>, k: i32) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();

    for el in vec.iter() {
        if el.abs() >= k {
            new_vec.push(*el);
        }
    }

    return new_vec;
}

fn main() {
    let mut rng = rand::thread_rng();
    let vals: Vec<i32> = (0..40).map(|_| rng.gen_range(-256..256)).collect();
    let vals: Vec<i32> = vec![];
    let vals: Vec<i32> = vec![4, 3, 1, 0, 5, 2];
    let vals: Vec<i32> = vec![-2, -5];
    let vals: Vec<i32> = vec![-3, -6, -1, 0, 2, 4, -2, 0];

    println!("Vector: \n{:?}", vals);
    
    if vals.len() == 0 {
        eprintln!("Vector is empty");
        exit(1)
    }

    // First task

    let start = Instant::now();
    let result_sort: i32 = alg_with_sort(&vals);
    let duration = start.elapsed();
    
    println!("Algorithm with sort returns {}", result_sort);
    println!("Time: {:?}", duration);

    let start = Instant::now();
    let result_iter: Option<i32> = alg_with_iter(&vals);
    let duration = start.elapsed();
    
    match result_iter {
        Some(result_iter) => println!("Algorithm with iterations returns {}", result_iter),
        None              => println!("Vector length is less than 2"),
    }
    println!("Time: {:?}", duration);

    // Second task
    println!("Finding max positive element if condifion will be true...");
    let mut check_zero: bool = true;

    for (i, item) in vals.iter().enumerate() {
        if *item == 0 && i % 2 != 0 {
            check_zero = false;
            break;
        }
    }
    
    if check_zero {
        let result: Option<&i32> = find_max_positive(&vals);

        match result {
            Some(result) => println!("Max positive value: {}", result),
            None         => println!("Positive vaules are not found"),
        };
    }
    else {
        println!("Not all null elements have an even index");
    }

    // Third task
    let a: usize = 3;
    let b: usize = 5;
    println!("Result of dividing vector between {} index and {} index: {:?}", a, b, divide_num_half(&vals, a, b));
    
    // Fouth task
    let sum_btw_neg: Option<i32> = find_sum_btw_neg(&vals);

    match sum_btw_neg {
        Some(result) => println!("The sum between two negative numbers in vector equals {}", result),
        None         => println!("Two negative values are not found"),
    };

    // Fifth task
    println!("Result of sorting first half of vector: {:?}", sort_first_half(&vals)); 

    // Sith task
    println!("Result of custom order of vector: {:?}", custom_order(&vals));

    // Seventh task
    let k: i32 = 3;
    println!("Result of removing elements less {} modulo {:?}", k, delete_el_less_k(&vals, k));
}

