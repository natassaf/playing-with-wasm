use std::convert::TryInto;

#[no_mangle]
pub extern "C" fn is_prime(n: u64) -> u32 {
    if n <= 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    if n % 2 == 0 {
        return 0;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        let i_new:u64 = i.try_into().unwrap();
        if n % i_new == 0 {
            return 0;
        }
    }
    1
}
