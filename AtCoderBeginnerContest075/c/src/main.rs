use proconio::input;                                                                                                                           
 
fn main() {
 
    input! {
        a: i64,
    }
 
    let n = a.to_string().len();
    let mut result = 10000000000;
    let mut limit = 1;
 
    for _ in 0..(n/2 + 1) {
        limit *= 10;
    }
 
    if n == 1 {
        println!("1");
    }else{
        for i in 1..=limit {
            if a % i as i64 == 0 {
                let alen = (a / i).to_string().len();
                if i.to_string().len() <= alen {
                    if result >= alen {
                        result = alen;
                    }
                }
            }
        }
       println!("{}", result);
    }
 
}
