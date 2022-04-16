use proconio::input;

fn main() {

    input! { 
        s: String,

    }

    
    let mut a:Vec<bool> = vec![false; 10]; 

    for i in s.chars() {
        a[i as usize - 48] = true; 
    }

    for i in 0..10 {
        if !a[i] {
            println!("{}", i);
            break;
        }
    }
        


}

