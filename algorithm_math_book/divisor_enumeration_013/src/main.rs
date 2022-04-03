use proconio::input;

fn main() {

    input! { 
        a: i64,
    }

    
    
    let mut ans = Vec::new();
    for i in 1..=a{
        if i*i > a {
            break;
        }

       if a % i == 0{
           ans.push(i);
           ans.push(a/i);
       }
    }
    for d in ans {
        println!("{}", d);
    }

}

