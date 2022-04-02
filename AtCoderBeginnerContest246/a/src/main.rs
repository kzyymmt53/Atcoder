use proconio::input;
use std::collections::HashMap;

fn main() {

    let mut v = Vec::new();
    let mut v2 = Vec::new();
    for _ in 0..3{
        input! { 
            x: i32,
            y: i32,
        }

        v.push(x);
        v2.push(y);
        
    }


        
    let mut hashmap = HashMap::new();
    for x in v {
        let counter = hashmap.entry(x).or_insert(0);
        *counter += 1;
    }

    let mut hashmap2 = HashMap::new();
    for y in v2 {
        let counter = hashmap2.entry(y).or_insert(0);
        *counter += 1;
    }





    let mut ansy = 0;
    for d in hashmap2 {
        if d.1 % 2 != 0 {
            ansy = d.0;
            break;
        }
    }
    let mut ansx = 0;
    for d in hashmap {
        if d.1 % 2 != 0 {
            ansx = d.0;
            break;
        }
    }
    println!("{} {}", ansx, ansy); 
}


