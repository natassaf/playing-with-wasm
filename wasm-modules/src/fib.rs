#[unsafe(no_mangle)]
pub extern "C" fn fib(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 1;
    for _ in 0..n {
        let t = a;
        a = b;
        b += t;
    }
    
    return b;
}
