use proconio::input;

fn main() {

    input! { 
        n: i64,
        r: i64,
    }


   let mut a: f64 = 1.0; 
   for i in 1..=r {
       a *= i as f64;
   }

   let mut b: f64 = 1.0;
   for i in 0..r {
       b *= (n - i) as f64;
   }

   println!("{}", b / a);
}

