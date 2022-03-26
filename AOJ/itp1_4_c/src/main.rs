use proconio::input;

fn main() {
    
    let mut v: Vec<i32> = Vec::new();
    loop {
        input! {
            a: i32,
            op: String,
            b: i32,
        }
        if op == "?" {
            break;
        }
        if op == "+" { 
            v.push(a + b);
        }else if op == "-" {
            v.push(a - b);
        }else if op == "*" {
            v.push(a * b);
        }else if op == "/" {
            v.push(a / b);
        } 
    }

    for d in v {
        println!("{}", d);
    }
     
    
}

