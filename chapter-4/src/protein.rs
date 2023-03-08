use csv::Writer;
use rand::{thread_rng, Rng};

fn gen_monomer() -> String {
    let mut rng = thread_rng();
    let is_h = rng.gen_bool(0.7);

    if is_h {
        String::from("H")
    } else {
        String::from("P")
    }
}

pub fn fold(del: f32, n: i8) {
    // data output
    let mut writer = Writer::from_path("./chapter-4/data/protein/fold.csv").unwrap();
    let mut writer2 = Writer::from_path("./chapter-4/data/protein/connections.csv").unwrap();

    // initial empty grid
    let mut grid = vec![];

    for _ in 0..n {
        let mut row = vec![];

        for _ in 0..n {
            row.push(String::from(""));
        }
        grid.push(row);
    }

    // data collection
    let mut total_h = 0;
    let mut total_p = 0;
    let mut total_m = 0;

    let mut connected_h = 0;

    let mut previous_m = String::from("");
    let mut next_m;

    // initial position of monomer
    let mut x = rand::thread_rng().gen_range(0..n);
    let mut y = rand::thread_rng().gen_range(0..n);

    println!("{},{}", x, y);

    // generate first monomer
    let mut current_m = gen_monomer();

    grid[x as usize][y as usize] = current_m.clone();

    if current_m == "H" {
        total_h += 1;
    } else {
        total_p += 1;
    }
    total_m += 1;

    // iteration conditions & data
    let iterate = true;
    let mut connections = vec![];
    let mut chain = vec![];

    while iterate {
        // pos
        let mut next_x = x;
        let mut next_y = y;

        // check if cornered
        if corner(x, y, n) {
            println!("corner: {} {}", x, y);
            break;
        }

        // check if trapped
        if !has_space(x as usize, y as usize, n as usize, &grid) {
            println!("no space left: {} {}", x, y);
            break;
        }

        // take step in 1 dir
        let step = rand::thread_rng().gen_range(0..4) as i8;

        if step == 0 {
            next_x += 1;
        }
        if step == 1 {
            next_x -= 1;
        }
        if step == 2 {
            next_y += 1;
        }
        if step == 3 {
            next_y -= 1;
        }

        // check if step is out of bounds
        if out_of_bounds(next_x, next_y, n) {
            println!("out of bounds: {} {}", next_x, next_y);
            continue;
        }

        // get next node
        let next_node = &grid[next_x as usize][next_y as usize];

        // if node is filled, go again
        if next_node != "" {
            continue;
        }

        // generate monomer
        next_m = gen_monomer();

        // update grid
        grid[next_x as usize][next_y as usize] = next_m.clone();

        // save order of monomer build
        connections.push((x, y, next_x, next_y));
        connections.push((next_x, next_y, x, y));
        chain.push(next_m.clone());

        // monomer counts
        if next_m == "H" {
            total_h += 1;
        }
        if next_m == "P" {
            total_p += 1;
        }
        total_m += 1;

        // connection count
        if current_m == "H" && next_m == "H" {
            connected_h += 1;
        }

        if current_m == "H" && next_m != "H" && previous_m == "H" {
            connected_h += 1;
        }

        // update chains
        previous_m = current_m;
        current_m = next_m;

        // update pos
        x = next_x;
        y = next_y;
        println!("{},{}", x, y)
    }

    let unconnected = count_unconnected_neighbors(&grid, &connections, n);

    // encode fold for matlab
    for row in grid.iter() {
        println!("{:?}", row);

        let mut data = vec![];
        for v in row.iter() {
            if v == "" {
                data.push("0")
            }
            if v == "P" {
                data.push("1")
            }
            if v == "H" {
                data.push("2")
            }
        }
        writer.write_record(data).unwrap();
    }

    // encode connections for matlab
    for connection in connections.iter() {
        let data = [
            connection.0.to_string(),
            connection.1.to_string(),
            connection.2.to_string(),
            connection.3.to_string(),
        ];
        writer2.write_record(data).unwrap();
    }

    println!("total p: {}", total_p);
    println!("total h: {}", total_h);
    println!("total m: {}", total_m);
    println!("total connected h: {}", connected_h);
    println!("total unconnected h: {}", unconnected);

    let energy = -del * unconnected as f32;
    println!("Total Energy: {}", energy);
    println!("Total length: {}", total_m);

    writer.flush().expect("Cant write grid data");
}

fn corner(x: i8, y: i8, n: i8) -> bool {
    (x == 0 && y == 0)
        || (x == n - 1 && y == 0)
        || (x == 0 && y == n - 1)
        || (x == n - 1 && y == n - 1)
}

fn out_of_bounds(x: i8, y: i8, n: i8) -> bool {
    let x = x as i8;
    let y = y as i8;
    let n = n as i8;

    x < 0 || y < 0 || x >= n || y >= n
}

fn count_unconnected_neighbors(
    grid: &Vec<Vec<String>>,
    connections: &Vec<(i8, i8, i8, i8)>,
    n: i8,
) -> i32 {
    let mut counter = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            if node == "H" {
                let x = i as i8;
                let y = j as i8;
                // check neighbors
                if i + 1 < n as usize {
                    let next_x = (i + 1) as i8;
                    let next_y = y;
                    if grid[i + 1][j] == "H" && !connections.contains(&(x, y, next_x, next_y)) {
                        println!("{:?}", (x, y, next_x, next_y));
                        counter += 1;
                    }
                }
                if j + 1 < n as usize {
                    let next_x = x;
                    let next_y = (j + 1) as i8;
                    if grid[i][j + 1] == "H" && !connections.contains(&(x, y, next_x, next_y)) {
                        println!("{:?}", (x, y, next_x, next_y));

                        counter += 1;
                    }
                }
                if i > 0 as usize {
                    let next_x = (i - 1) as i8;
                    let next_y = y;
                    if grid[i - 1][j] == "H" && !connections.contains(&(x, y, next_x, next_y)) {
                        println!("{:?}", (x, y, next_x, next_y));

                        counter += 1;
                    }
                }
                if j > 0 as usize {
                    let next_x = x;
                    let next_y = (y - 1) as i8;
                    if grid[i][j - 1] == "H" && !connections.contains(&(x, y, next_x, next_y)) {
                        println!("{:?}", (x, y, next_x, next_y));

                        counter += 1;
                    }
                }
            }
        }
    }
    counter / 2
}

fn has_space(x: usize, y: usize, n: usize, data: &Vec<Vec<String>>) -> bool {
    let mut has_space = false;

    if x + 1 < n as usize {
        if data[x + 1][y] == "" {
            has_space = has_space || true;
        }
    }
    if y + 1 < n as usize {
        if data[x][y + 1] == "" {
            has_space = has_space || true;
        }
    }
    if x > 0 as usize {
        if data[x - 1][y] == "" {
            has_space = has_space || true;
        }
    }
    if y > 0 as usize {
        if data[x][y - 1] == "" {
            has_space = has_space || true;
        }
    }
    has_space
}
