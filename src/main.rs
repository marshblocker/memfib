// Compares the runtime of a regular Fibonacci function
// with a memoized Fibonacci function. Control the value
// of n to observe the dramatic difference between the
// runtime of each function.

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let mut cache: HashMap<u64, u64> = HashMap::new();

    // When n becomes greater than 50, the regular fibonacci
    // becomes dramatically slow.
    let n: u64 = 40;

    let now = Instant::now();

    println!("Calling fast fib!");
    call_fast_fib(n, &mut cache);

    let fast_time = now.elapsed().as_micros() as f64;
    println!("Finished after {} microseconds\n", fast_time);
    
    let now = Instant::now();

    println!("Calling slow fib!");
    call_slow_fib(n);

    let slow_time = now.elapsed().as_micros() as f64;
    println!("Finished after {} microseconds\n", slow_time);

    println!("Speed-up ratio: {:.5}", slow_time / fast_time);
}

fn call_fast_fib(n: u64, cache: &mut HashMap<u64, u64>) {
    for i in 0..n {
        println!("fib({}) = {}", i, fib_memoized(i, cache));
    }
}

fn call_slow_fib(n: u64) {
    for i in 0..n {
        println!("fib({}) = {}", i, fib_regular(i));
    }
}

fn fib_regular(n: u64) -> u64 {
    if n <= 1 { return n; }

    fib_regular(n-1) + fib_regular(n-2)
}

fn fib_memoized(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    
    let ans = match cache.get(&n) {
        Some(ans) => ans,
        None      => {
            let f1 = fib_memoized(n-1, cache);
            let f2 = fib_memoized(n-2, cache);
            cache.insert(n, f1 + f2);
            cache.get(&n).unwrap()
        }
    };

    *ans
}