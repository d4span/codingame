use std::{collections::HashMap, io, ptr::null};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Don't let the machines win. You are humanity's last hope...
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let width = parse_input!(input_line, i32); // the number of cells on the X axis

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let height = parse_input!(input_line, i32); // the number of cells on the Y axis

    let mut neighbors = HashMap::new();
    let mut previous_in_col = Vec::with_capacity(width as usize);
    previous_in_col.resize(width as usize, Option::<(usize, usize)>::None);

    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line
            .trim_matches('\n')
            .to_string()
            .chars()
            .collect::<Vec<char>>(); // width characters, each either 0 or .

        let mut previous_in_row = Option::<(usize, usize)>::None;
        for j in 0..line.len() {
            if line[j] == '0' {
                let mut entries: Vec<(i32, i32)> = Vec::with_capacity(2);
                entries.resize(2, (-1, -1));

                neighbors.insert((j, i), entries);

                if previous_in_row.is_some() {
                    neighbors
                        .get_mut(&previous_in_row.unwrap())
                        .map(|v| v[0] = (j as i32, i as i32));
                }

                if previous_in_col[j].is_some() {
                    neighbors
                        .get_mut(&previous_in_col[j].unwrap())
                        .map(|v| v[1] = (j as i32, i as i32));
                }

                previous_in_row = Some((j, i));
                previous_in_col[j] = Some((j, i));
            }
        }
    }

    // Three coordinates: a node, its right neighbor, its bottom neighbor
    neighbors.keys().for_each(|k| {
        let (x1, y1) = k;
        let ns = neighbors.get(k).unwrap();
        let (x2, y2) = ns[0];
        let (x3, y3) = ns[1];

        println!("{} {} {} {} {} {}", x1, y1, x2, y2, x3, y3);
    });
}
