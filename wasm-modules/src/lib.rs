// mod fib;
// mod prime_number;
// mod double_sha;

wit_bindgen::generate!({
    world: "prime",
    // Change `wit_path` to `path`
    path: "prime.wit", // Assumes prime.wit is in the crate root or specified relative path
});

struct Prime;

impl Guest for Prime {
    fn is_prime(n: u64) -> u32 {
        println!("Is prime running");
        let mut found = 1; // Assume prime
        for i in 2..n {
            if n % i == 0 {
                found = 0; // Mark composite but keep looping
            }
        }
        found
    }
}

export!(Prime);