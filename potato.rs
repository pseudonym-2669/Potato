//rust

fn main() {
    let mut count = 0u32;

    // Infinite loop
    loop {
        count += 1;

        if count == 11 {
            break;
        }

        println!("Potato {}", count);

    }
}
