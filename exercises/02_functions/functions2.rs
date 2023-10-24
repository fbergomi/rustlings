// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {
    call_me(13);
}

///Example method
fn call_me(num: i16) {
    for i in 1..=num {
        println!("Ring! Call number {}", i);
    }

    println!("----");
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
