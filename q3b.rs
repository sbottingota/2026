fn count_unordered_digits(n: u64) -> u32 {
    if n / 10 == 0 {
        return 0;
    }

    let unordered = if (n / 10) % 10 > n % 10 { 1 } else { 0 };
    unordered + count_unordered_digits(n / 10)
}

fn main() {
    for i in 8_102_026+1.. {
        if count_unordered_digits(i) == 1 {
            println!("{}", i);
            break;
        }
    }
}

