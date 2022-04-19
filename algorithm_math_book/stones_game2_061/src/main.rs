use proconio::input;

fn main() {

    input! { 
        a: i64,
    }

    if a == 1 {
        println!("Second");
        return;
    }
    let mut start: i64 = 1;
    
    for _ in 1..100000 {
        start *= 2;
        start += 1;
        if start == a {
            println!("Second");
                break;
        }

        if start > a {
            println!("First");
            break;
        }

             
    }

}

