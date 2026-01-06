use std::io;
use memoize::memoize;

#[memoize]
fn count_unordered_digits(n: u64) -> u32 {
    if n / 10 == 0 {
        return 0;
    }

    let unordered = if (n / 10) % 10 > n % 10 { 1 } else { 0 };
    unordered + count_unordered_digits(n / 10)

    /*
    let mut unordered_digits = 0_u32;

    while n > 0 {
        if (n / 10) % 10 > n % 10 {
            unordered_digits += 1;
        }

        n /= 10;
    }

    unordered_digits
    */
}


fn count_raindrops_to(n: u64) -> u64 {
    let mut count = 0;

    for i in 1..n {
        if count_unordered_digits(i) == 1 {
            count += 1;
        }
    }

    count
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: u64 = input
        .trim()
        .parse()
        .unwrap();

    println!("{}", count_raindrops_to(n));

    Ok(())
}

