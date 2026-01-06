use std::io;

fn char_to_coord(c: char) -> (u32, u32) {
    let val = c as u32 - 'A' as u32;
    (val % 5, 4 - val / 5)
}

fn coords(divisions: Vec<char>) -> (u32, u32) {
    let mut coords = (0_u32, 0_u32);

    for c in divisions {
        coords.0 *= 5;
        coords.1 *= 5;

        let (x, y) = char_to_coord(c);
        coords.0 += x;
        coords.1 += y;
    }

    coords.0 += 1;
    coords.1 += 1;

    coords
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let divisions: Vec<char> = input
        .trim()
        .chars()
        .collect();

    let (x, y) = coords(divisions);
    println!("{} {}", x, y);

    Ok(())
}
