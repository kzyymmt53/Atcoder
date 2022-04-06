use proconio::input;
use std::cmp::{max, min};

fn main() {

    input! { 
        x1: i64,
        y1: i64,
    }
    input! { 
        x2: i64,
        y2: i64,
    }       
    input! { 
        x3: i64,
        y3: i64,
    }       
    input! { 
        x4: i64,
        y4: i64,
    }       

    let mut s = (x1 - x2) * (y3 - y1) - (y1 - y2) * (x3 - x1);
    let mut t = (x1 - x2) * (y4 - y1) - (y1 - y2) * (x4 - x1);

    if s == 0 && t == 0 {
        if min(x3, x4) <= x2 && x2 <= max(x3, x4) && min(y3, y4) <= y2 && y2 <= max(y3, y4) {
            println!("Yes");
        }else if min(x3, x4) <= x1 && x1 <= max(x3, x4) && min(y3, y4) <= y1 && y1 <= max(y3, y4) {
            println!("Yes");
        }else if min(x1, x2) <= x3 && x3 <= max(x1, x2) && min(y1, y2) <= y3 && y3 <= max(y1, y2) {
            println!("Yes");
        }else if min(x1, x2) <= x4 && x4 <= max(x1, x2) && min(y1, y2) <= y4 && y4 <= max(y1, y2) {
            println!("Yes");
        }else{
            println!("No");
        }
    }else if s > 0 && t > 0 || s < 0 && t < 0 {
        println!("No");
    }else{
        s = (x3 - x4) * (y1 - y3) - (y3 - y4) * (x1 - x3);
        t = (x3 - x4) * (y2 - y3) - (y3 - y4) * (x2 - x3);
        if s > 0 && t > 0 || s < 0 && t < 0 {
            println!("No");
        }else{
            println!("Yes");
        }
    }

}

