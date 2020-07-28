use std::time::{Instant};
use std::io;
use rand::Rng;


// i implemented a timer that finds the time each algorithm takes to compute the LISS, to show the superiority of the patience algorithm
fn main() {
    let mut seq = generate_ran(); // uses function to generate random sequence of distinct positive numbers
    let mut seqslow = seq.clone(); // allocates another copy of the sequence to test on the slow algorithm
    let len = seq.len();
    let time_fast: f32 = liss_fast(&mut seq) as f32;
    println!("{}",time_fast);
    let time_slow: f32 = liss_slow(&mut seqslow) as f32;
    let ratio: f32 = 1.0/ (time_fast / time_slow);
    println!("The fast algorithm was {} times faster than the slower one for a vector size of {}.", ratio, len);
}

fn liss_fast(seq: &mut Vec<i32>) -> u32 {
    let now = Instant::now();
    let mut piles: Vec<i32> = Vec::new();
    piles.push(seq.remove(0));
    while seq.len() > 0 {
        let num = seq.remove(0);
        let mut index = piles.len()-1;
        let mut one_more = true;
        while num < piles[index] {
            if index == 0 {
                one_more = false;
                break;
            }
            index-=1;
        }
        if one_more {
            index+=1;
        }
        if index > piles.len()-1 {
            piles.push(num);
        }
        else {
            piles[index] = num;
        }
    }
    let answer = piles.len();
    let nseconds = now.elapsed().subsec_nanos();
    println!("LISS: {:?}", answer);
    println!("Time elapsed for FAST: {}", nseconds);
    nseconds
}

fn liss_slow(seq: &mut Vec<i32>) -> u32 { // CODE WE WROTE IN CLASS
    let now = Instant::now();
    let mut history = vec![1; seq.len()];
    for i in 0..(seq.len()-1) {
        for j in (i+1)..seq.len() {
            if seq[j] > seq[i] {
                if history[j] <= history[i] {
                    history[j] += 1;
                }
            }
        }
    }
    let max_value = history.iter().max();
    let nseconds = now.elapsed().subsec_nanos();
    println!("LISS: {:?}", max_value.unwrap());
    println!("Time elapsed for SLOW: {}", nseconds);
    nseconds
}

fn generate_ran() -> Vec<i32> {
    println!("Please input a vector size.");
    let mut num = String::new();

    io::stdin()
    .read_line(&mut num)
    .expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("Please type a number!");
    let mut gen_array: Vec<i32> = Vec::new();
    for _i in 0..num {
        let mut x = rand::thread_rng().gen_range(1, num+1);
        while gen_array.contains(&x) {
            x = rand::thread_rng().gen_range(1, num+1);
        }
        gen_array.push(x);
    }
    gen_array
}
