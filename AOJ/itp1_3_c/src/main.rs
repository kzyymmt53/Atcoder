use proconio::input;

fn main() {
    let mut v: Vec<String> = Vec::new();

    loop {
        input! {
            a: i32,
            b: i32,
        }
        if a == 0 && b == 0 {
            break;
        }
        let s: String;
        if a < b {
            s = a.to_string() + " " + &b.to_string(); 
        }else{
            s = b.to_string() + " " + &a.to_string(); 
        }
        v.push(s);
    }
   
    for d in v {
        println!("{}", d); 
    }   
    
}

