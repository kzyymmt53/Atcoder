use proconio::input;
use regex::Regex;

fn main() {

    input! { 
        _: usize,
        b: String,
    }

    let mut result = 0;

    
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                let search_str = format!{"{}.*{}.*{}", i, j, k};
                let re = Regex::new(&search_str).unwrap();
                if re.find(&b).is_some() {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);

}

