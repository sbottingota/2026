use std::io;

fn is_raindrop(mut n: u64) -> bool {
    let mut unordered_digits = 0_u32;

    while n > 0 {
        if (n / 10) % 10 > n % 10 {
            unordered_digits += 1;
        }

        n /= 10;
    }

    unordered_digits == 1
}

fn count_raindrops_to(n: u64) -> u64 {
    let mut count = 0;

    for i in 1..n {
        if is_raindrop(i) {
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

