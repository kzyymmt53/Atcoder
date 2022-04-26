use proconio::input;

fn main() {

    input! { 
        n: i64,
        x: i64,
        y: i64,
    }

    let mut ans = false;
    for i in 1..=n {
        for j in i..=n {
            for k in j..=n {
                for l in k..= n {
                    if i + j + k + l == x && i * j * k * l == y {
                        ans = true;
                        break;
                    }
                }
            }
        }
    }


    if ans {
        println!("Yes");
    }else{
        println!("No");
    }

}

