use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    input! {
        p: [usize; n],
        q: [usize; n],
    }


    let mut order = vec![];
    for i in 1..n+1 {
        order.push(i);
    }

    let mut v = vec![];
    while {
        v.push(order.clone());
        order.next_permutation()
    }{}

    v.sort();

    let mut a = 0;
    let mut b = 0;

    for (i, d) in v.iter().enumerate() {
        if &p == d {
           a = i;
        }
        if &q == d {
           b = i;
        }
    }

    println!("{}", (a as i32 - b as i32).abs());

}


pub trait LexicalPermutation { 
    fn next_permutation(&mut self) -> bool; 
    fn prev_permutation(&mut self) -> bool; 
}

impl<T> LexicalPermutation for [T] where T: Ord {
    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 { return false; }
        let mut i = self.len() - 1;

        while i > 0 && self[i-1] >= self[i] {
            i -= 1;
        }

        if i == 0 {
            return false;
        }

        let mut j = self.len() - 1;

        while j >= i && self[j] <= self[i-1]  {
            j -= 1;
        }

        self.swap(j, i-1); 
        self[i..].reverse();  

        true 

    }

    fn prev_permutation(&mut self) -> bool {
        if self.len() < 2 { return false; }
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] <= self[i] {
            i -= 1;
        }

        if i == 0 { 
           return false;
        }

        self[i..].reverse();

        let mut j = self.len() - 1;
        while j >= i && self[j-1] < self[i-1]  {
            j -= 1;
        }

        self.swap(i-1, j);

        true

    }

}




