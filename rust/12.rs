fn rotate_ccw((px, py): (i32, i32), angle: i32) -> (i32, i32) {
    // Only do counter-clockwise rotations
    let angle = (angle + 360) % 360;

    // x' = x*cos(a) - y*sin(a)
    // y' = x*sin(a) + y*cos(a)
    match angle {
        90 => (-py, px), // x' = x*0 - y*1, y' = x*1 + y*0
        180 => (-px, -py),
        270 => (py, -px), // x' = x*0 - y*-1, y' = x*-1 + y*0
        _ => (px, py),
    }
}

fn main() {
    let input = include_str!("../input12.txt");

    let ((pos_x, pos_y), _) =
        input
            .lines()
            .fold(((0, 0), 0), |((pos_x, pos_y), bearing), instruction| {
                let num: i32 = instruction[1..].parse().unwrap();
                match &instruction[0..1] {
                    "W" => ((pos_x - num, pos_y), bearing),
                    "E" => ((pos_x + num, pos_y), bearing),
                    "N" => ((pos_x, pos_y + num), bearing),
                    "S" => ((pos_x, pos_y - num), bearing),
                    "L" => ((pos_x, pos_y), (bearing + 360 - num) % 360),
                    "R" => ((pos_x, pos_y), (bearing + num) % 360),
                    "F" => match bearing {
                        0 => ((pos_x, pos_y + num), bearing),
                        90 => ((pos_x + num, pos_y), bearing),
                        180 => ((pos_x, pos_y - num), bearing),
                        270 => ((pos_x - num, pos_y), bearing),
                        _ => ((pos_x, pos_y), bearing),
                    },
                    _ => ((pos_x, pos_y), bearing),
                }
            });

    println!("{}", pos_x.abs() + pos_y.abs());

    let ((pos_x, pos_y), _) = input.lines().fold(
        ((0, 0), (10, 1)),
        |((pos_x, pos_y), (wp_x, wp_y)), instruction| {
            let num: i32 = instruction[1..].parse().unwrap();
            match &instruction[0..1] {
                "W" => ((pos_x, pos_y), (wp_x - num, wp_y)),
                "E" => ((pos_x, pos_y), (wp_x + num, wp_y)),
                "N" => ((pos_x, pos_y), (wp_x, wp_y + num)),
                "S" => ((pos_x, pos_y), (wp_x, wp_y - num)),
                "L" => ((pos_x, pos_y), rotate_ccw((wp_x, wp_y), num)),
                "R" => ((pos_x, pos_y), rotate_ccw((wp_x, wp_y), -num)),
                "F" => ((pos_x + wp_x * num, pos_y + wp_y * num), (wp_x, wp_y)),
                _ => ((pos_x, pos_y), (wp_x, wp_y)),
            }
        },
    );

    println!("{}", pos_x.abs() + pos_y.abs());
}
