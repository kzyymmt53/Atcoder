use proconio::input;

fn main() {

    input! { 
        s1: String,
        s2: String,
        s3: String,
    }

    input! {
        t1: String,
        t2: String,
        t3: String,
    }

    let mut count = 0;
    if s1 != t1 {
        count += 1;
    }
    if s2 != t2 {
        count += 1;
    }

    if s3 != t3 {
        count += 1;
    }

    if count % 3 == 0 {
        println!("Yes");
    }else{
        println!("No");
    }


}

