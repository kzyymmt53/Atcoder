use proconio::input;

fn main() {

    let mut v: Vec<String> = Vec::new();
    loop {
        input! {
            s: String,
        }
        let mut s2 = s;
        if s2 == "-" {
            break;
        }

        input! {
            m: usize,
        }
   
        for _ in 0..m {
            input! {
                a: usize,
            }
            s2 = s2[a..s2.len()].to_string() + &s2[0..a]; 
        }
        v.push(s2);
    }
    for d in v {
        println!("{}", d);
    } 

}

