use proconio::input;

fn main() {


    input! {
        s: String,
    }
    let mut s2 = s;
    input! {
        n: usize,
    }
    
    for _ in 0..n {
        input! {
            t: String,
        }
   
        if t == "print" {
            input! {
                a: usize, 
                b: usize, 
            }
            println!("{}", &s2[a..b+1]);
        }
       
        if t == "reverse" {
            input! {
                a: usize,
                b: usize,
            }
            let temp = s2.clone()[a..b+1].chars().rev().collect::<String>();
            let mut temps = String::new();
            for (i, c) in s2.chars().enumerate() {
                 if a <= i && i <= b {
                     temps += &temp[(i-a)..(i-a+1)];
                 }else{
                     temps += &c.to_string();
                 }
            }
            s2 = temps;
        }

        if t == "replace" {
            input! {
                a: usize,
                b: usize,
                p: String,
            }
            let mut temps = String::new();
            for (i, c) in s2.chars().enumerate() {
                if a <= i && i <= b {
                   temps += &p[(i-a)..(i-a+1)]; 
                }else{
                   temps += &c.to_string();
                }
            }
            s2 = temps;
        }

    } 
}
