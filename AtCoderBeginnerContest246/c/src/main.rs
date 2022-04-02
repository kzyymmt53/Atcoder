use proconio::input;

fn main() {

    input! { 
        n: usize,
        k: i64,
        x: i64,
    }

    let mut v = Vec::new();
    for _ in 0..n {
        input! {
            a: i64,
        }
        v.push(a);
    }


    let vector_ref = &mut v;
    (*vector_ref).sort();
    vector_ref.reverse();



    
    let mut other = vector_ref.clone();
    let mut rest = k;
    for (j, d) in vector_ref.iter().enumerate() {
        if d / x <= rest {
            rest -= d/x;
            other[j] -= x*(d/x as i64);
        }else{
            other[j] -= x* rest;
            rest = 0;
        }

        if rest == 0 {
            break;
        }
    }


    let vector_ref = &mut other;
    (*vector_ref).sort();
    vector_ref.reverse();

    let mut ans = 0;
    for i in vector_ref {
        let temp = *i as i64;
        if rest != 0 {
            rest -= 1;
        }else{
            ans += temp;
        }


    }
    println!("{}", ans);

    

}

