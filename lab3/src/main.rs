use regex::Regex;


fn main() {
   let mut data: Vec<String> = vec!["234dwe", "er++we23", "rege", "23324", "ewewr***1", "dsd", "kkdkk"]
       .iter()
       .map(|s| s.to_string())
       .collect();
   
   println!("Data before modifying:");
   dbg!(&data);

   let re_num = Regex::new(r"\d").unwrap();
   let re = Regex::new(r"\+{2}|\*{3}").unwrap();

   for s in data.iter_mut() {
       *s = re_num.replace_all(s, "").to_string();
       *s = re.replace_all(s, "?").to_string();
   }
   
   println!("Data aftermodifying:");
   dbg!(&data);
   
   let k: usize = 4;
   let mut result: Vec<String> = data
       .iter()
       .filter(|x| !x.is_empty() && is_palindrome(&x))
       .cloned()
       .collect::<Vec<String>>();

   result.sort_by(|a, b| b.len().cmp(&a.len()));

   let sentence: String = result.join(" ");
   dbg!(sentence);
}

fn is_palindrome(word: &str) -> bool {
    let mut i: usize = 0;
    let bytes = word.as_bytes();

    while i != (word.len() - 1) / 2 {
        if bytes[i] != bytes[word.len() - i - 1] {
            return false;
        }
        i += 1;
    }

    return true;
}
