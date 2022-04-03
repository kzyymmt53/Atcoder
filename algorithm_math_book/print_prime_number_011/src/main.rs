use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    let mut v = Vec::new(); 
    for i in 2..=n {
        let mut check = true;
        for j in 2..=i {
            if j*j > i {
                break;
            }
            if i == j {
                continue;
            }
            if i % j == 0 {
                check =false;
                break;
            }
        }
        if check {
            v.push(i);
        }
    }

    for (i, d) in v.iter().enumerate() {
        if i == 0 {
            print!("{}", d);
        }else{
            print!(" {}", d);
        }
    }
    println!("");

}

