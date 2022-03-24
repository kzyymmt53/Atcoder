use proconio::input;

fn main() {
    input! {
        n1: i32,
        n5: i32,
        n10: i32,
        n50: i32,
        n100: i32,
        n500: i32,
        a: i32,
    }
    
    let mut total = a;
    let n: Vec<i32> = vec![n500, n100, n50, n10, n5, n1]; 
    let v: Vec<i32> = vec![500, 100, 50, 10, 5, 1]; 


    let mut result = 0; 

    for i in 0..5 {
        if n[i] <= total / v[i] {
            result += n[i];
            total -= n[i] * v[i];
        }else{
            result += total / v[i];
            total -=  total / v[i] * v[i];
        } 
    } 
    
    println!("{}", result);
    
}

