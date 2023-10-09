use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x0 = parse_input!(inputs[0], i32);
    let y0 = parse_input!(inputs[1], i32);

    // game loop
    let mut position = (x0, y0);
    let mut search_frame = (0, 0, w, h);
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

        let jump = match bomb_dir.as_str() {
            "U" => {
                move_up(&mut position, &mut search_frame);
            }
            "UR" => {
                move_right(&mut position, &mut search_frame);
                move_up(&mut position, &mut search_frame);
            }
            "R" => {
                move_right(&mut position, &mut search_frame);
            }
            "DR" => {
                move_right(&mut position, &mut search_frame);
                move_down(&mut position, &mut search_frame);
            }
            "D" => {
                move_down(&mut position, &mut search_frame);
            }
            "DL" => {
                move_left(&mut position, &mut search_frame);
                move_down(&mut position, &mut search_frame);
            }
            "L" => {
                move_left(&mut position, &mut search_frame);
            }
            &_ => {
                move_up(&mut position, &mut search_frame);
                move_left(&mut position, &mut search_frame);
            }
        };

        // the location of the next window Batman should jump to.
        println!("{} {}", position.0, position.1);
    }
}

fn move_right(position: &mut (i32, i32), search_frame: &mut (i32, i32, i32, i32)) {
    search_frame.0 = position.0;
    let new_x = (search_frame.2 - position.0) / 2;
    position.0 += new_x;
}

fn move_left(position: &mut (i32, i32), search_frame: &mut (i32, i32, i32, i32)) {
    search_frame.2 = position.0;
    let new_x = (search_frame.0 - (position.0 - 1)) / 2 - 1;
    position.0 += new_x;
}

fn move_down(position: &mut (i32, i32), search_frame: &mut (i32, i32, i32, i32)) {
    search_frame.1 = position.1;
    let new_y = (search_frame.3 - position.1) / 2;
    position.1 += new_y;
}

fn move_up(position: &mut (i32, i32), search_frame: &mut (i32, i32, i32, i32)) {
    search_frame.3 = position.1;
    let new_x = (search_frame.1 - (position.1 - 1)) / 2 - 1;
    position.1 += new_x;
}
