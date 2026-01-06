fn letter_from_coord(x: u32, y: u32) -> char {
    char::from_u32('A' as u32 + x + (4 - y) * 5).unwrap()
}

fn id_from_coord(mut x: u32, mut y: u32, mut size: u32) -> String {
    x -= 1;
    y -= 1;

    let mut id = String::new();

    while size > 1 {
        size /= 5;
        // println!("{} {} {}", x, y, size);
        id.push(letter_from_coord(x / size, y / size));

        x %= size;
        y %= size;
    }

    id
}

fn main() {
    assert_eq!(id_from_coord(45, 118, 125), "BIO".to_string()); // test case

    println!("{}", id_from_coord(209, 217, 625));
    println!("{}", id_from_coord(606445, 9161058, 9765625));
}

