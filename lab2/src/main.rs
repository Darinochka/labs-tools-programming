use std::io::{self, BufRead};
use std::collections::HashMap;


fn main() {
    let mut num_regions: HashMap<String, i32> = HashMap::new();
    let mut data: Vec<(String, String, String)> = vec![];
    let mut nums_record: u32 = 0;
    let mut lines = io::stdin().lock().lines();
    let mut i: u32 = 0; 
    
    while nums_record <= 0 {
        let line = match lines.next() {
            Some(i) => i.unwrap(),
            None    => { eprintln!("Error"); continue },
        };

        if line.len() == 0 {
            eprintln!("Enter the number of records, please.");
            continue;
        }
        
         
        nums_record = match line.trim().parse::<u32>() {
            Ok(i) => {
                if i == 0 {
                    eprintln!("Enter the number of records, please.");
                    continue;
                } else {
                    i
                }
            },
            Err(..) => { eprintln!{"Enter u32, please"}; 0 },
        };
    }

    while i != nums_record {
        // ----------------------- TASK A -----------------------
        let line = match lines.next() {
            Some(i) => i.unwrap(),
            None    => { eprintln!("Error"); continue },
        };
        
        if line.len() == 0 {
            eprintln!("Enter your record, please.");
            continue;
        }
        
        let record: Vec<&str> = line.split_whitespace().collect();
        
        if record.len() != 3 {
            eprintln!("Not enough arguments");
            continue;
        }
 
        let name: String = record[0].to_string();
        let license: String = record[1].to_string(); 
        let region: String = record[2].to_string();

        if name.len() > 50 {
            eprintln!("The name length should be less 50 chars\nTry again");
            continue;
        }

        if !license.chars().all(char::is_numeric) || license.len() != 6 {
            eprintln!("The license number must be six digits\nTry again");
            continue;
        };         
        
        if !region.chars().all(char::is_numeric) || region.len() > 2 {
            eprintln!("The license number must be not less thatn two digits\nTry again");
            continue;
        }

        i += 1;
        data.push((name, license, region.clone()));
        // ------------------------- TASK B -------------------------
        if let Some(x) = num_regions.get_mut(&region) {
            *x += 1;
        } else {
            num_regions.insert(
                region,
                1
            );
        }
    }
    println!("The result of task A:");
    dbg!(data);
    
    println!("The result of task B:");
    let min_region: Option<String> = num_regions
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k).cloned();
    match min_region {
        Some(region) => println!("The region with min number of applications is {}", region),
        None    => println!("Applitcations are not found"),
    }
}
