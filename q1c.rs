fn letter_from_coord(x: u32, y: u32) -> char {
    char::from_u32('A' as u32 + x + (4 - y) * 5).unwrap()
}

fn id_from_coord(mut x: u32, mut y: u32, mut size: u32) -> Vec<char> {
    let mut id = Vec::new();

    while size > 1 {
        size /= 5;
        // println!("{} {} {}", x, y, size);
        id.push(letter_from_coord(x / size, y / size));

        x %= size;
        y %= size;
    }

    id
}

fn neighbors(x: u32, y: u32, size: u32) -> Vec<(u32, u32)> {
    let mut ret = Vec::new();

    if x > 0 {
        ret.push((x-1, y));
    }
    if y > 0 {
        ret.push((x, y-1));
    }

    if x < size - 1 {
        ret.push((x+1, y));
    }
    if y < size - 1 {
        ret.push((x, y+1));
    }

    ret
}

fn main() {
    let size = 3125_u32;

    let mut count = 0_u32;

    for x in 0..size {
        for y in 0..size {
            let id = id_from_coord(x, y, size);

            for (x1, y1) in neighbors(x, y, size) {
                let neighbor_id = id_from_coord(x1, y1, size);

                if neighbor_id
                    .into_iter()
                    .all(|c| !id.contains(&c)) {
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("{}", count);
}

