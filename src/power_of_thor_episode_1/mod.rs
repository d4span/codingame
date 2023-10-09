use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let initial_tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let initial_ty = parse_input!(inputs[3], i32); // Thor's starting Y position

    let mut tx = initial_tx;
    let mut ty = initial_ty;

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.

        let dx = light_x - tx;
        let dy = light_y - ty;

        let result = if dx > 0 {
            tx += 1;
            if dy > 0 {
                ty += 1;
                "SE"
            } else if dy < 0 {
                ty -= 1;
                "NE"
            } else {
                "E"
            }
        } else if dx < 0 {
            tx -= 1;
            if dy > 0 {
                ty += 1;
                "SW"
            } else if dy < 0 {
                ty -= 1;
                "NW"
            } else {
                "W"
            }
        } else {
            if dy > 0 {
                ty += 1;
                "S"
            } else if dy < 0 {
                ty -= 1;
                "N"
            } else {
                "" // do nothing
            }
        };

        // A single line providing the move to be made: N NE E SE S SW W or NW
        println!("{}", result);
    }
}
