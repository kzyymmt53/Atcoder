use std::collections::HashMap;
use proconio::input;

fn main() {

    input! { 
        s: String,
    }
    let mut map = HashMap::new();
    let mut flag = true;
    let mut omoji = false;
    let mut komoji = false;

    for i in s.chars() {
        if map.contains_key(&i) {
            flag = false;
            break;
        }

        if i.is_uppercase() {
            omoji = true;
        }else{
            komoji = true;
        }

        map.insert(i, 1);
    }

    if flag && omoji && komoji {
        println!("Yes");
    }else{
        println!("No");
    }



}

