use proconio::input;

fn main() {

    input! { 
        n: i64,
        x: i64,
    }

    input! {
        mut a: [i64; n],
    }

    a.sort();

    let r = n - 1;
    let l = 0;

    if binary_search(r, l, a, x){
        println!("Yes");
    }else{
        println!("No");
    }


}

fn binary_search(r: i64, l: i64, a: Vec<i64>, x: i64) -> bool {

     if r - l < 0 {
         return false;
     }

     let m = ((r + l) / 2) as usize;

     if a[m] == x {
         return true;
     }

     let mut l2: i64 = l;
     let mut r2: i64 = r;

     if a[m] > x { 
         r2 = m as i64 - 1;
     }else {
         l2 = m as i64 + 1;
     }

     if binary_search(r2, l2, a, x) {
         return true;
     }else{
         return false;
     }
}


