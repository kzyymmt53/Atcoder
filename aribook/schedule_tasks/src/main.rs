use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut svec: Vec<i32> = Vec::new();
    for _ in 0..n {
        input! {
            s: i32,
        }
        svec.push(s);
    }

    let mut tvec: Vec<i32> = Vec::new();
    for _ in 0..n {
        input! {
            t: i32,
        }
        tvec.push(t);
    }

    let mut pairs: Vec<(i32, i32)> = Vec::new();
    for i in 0..n {
        pairs.push((svec[i], tvec[i]));
    }
   
    pairs.sort_by(|a, b| (a.1).partial_cmp(&(b.1)).unwrap());

   
    let mut result = 0;
    let mut start = 0;

    for j in 0..pairs.len() {
        if start < pairs[j].0 {
            result += 1;
            start = pairs[j].1;
        }    
    }
    println!("{}", result);

}

