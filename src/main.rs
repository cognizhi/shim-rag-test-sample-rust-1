use shim_rag_test_sample_rust_1::legacy::*;
use shim_rag_test_sample_rust_1::util::{helper, Counter as Tick};

fn main() {
    let doubled = helper(21);
    let mut counter = Tick::new();
    let total = counter.bump(doubled);
    let legacy = old_path(total);
    println!("{} {} {}", doubled, total, legacy);
}
