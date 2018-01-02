extern crate rand;

//use rand::Rng;
use rand::{thread_rng, Rng};

fn get_random(num: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::with_capacity(num);
    for i in 0..num {
        vec.push(i);
    }
    let mut rng = thread_rng();
    rng.shuffle(&mut vec);
    vec
}

fn ins_sort(a: &mut Vec<usize>){
    let n = a.len();
    for j in 1..n {
        println!("{:?}", j);
        let key = a[j];
        let mut i = j - 1;
        while a[i] > key && i >= 0 {
            a[i] = a[i];
            i -= 1;
        }
        a[i] = key;
    }

}

fn main() {
    let mut a = get_random(20);
    //println!("{:?}", a);
    ins_sort(&mut a);
    println!("{:?}", a);
    //let num = rand::thread_rng().gen_range(1, 101);
    //println!("Hello, world: {:?}", left);
}
