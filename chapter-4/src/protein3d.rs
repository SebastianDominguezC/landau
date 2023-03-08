use csv::Writer;
use rand::{thread_rng, Rng};
use serde_json;
use std::fs::File;
use std::io::{BufWriter, Write};

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
    let mut writer2 = Writer::from_path("./chapter-4/data/protein/connections.csv").unwrap();
    let fold_file = File::create("./chapter-4/data/protein/fold.json").unwrap();

    // initial empty grid
    let mut grid = vec![];

    for _ in 0..n {
        let mut row = vec![];

        for _ in 0..n {
            let mut col = vec![];

            for _ in 0..n {
                col.push(String::from(""));
            }

            row.push(col);
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
    let mut z = rand::thread_rng().gen_range(0..n);

    println!("{},{}, {}", x, y, z);

    // generate first monomer
    let mut current_m = gen_monomer();

    grid[x as usize][y as usize][z as usize] = current_m.clone();

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
        let mut next_z = z;

        // check if cornered
        if corner(x, y, z, n) {
            println!("corner: {} {} {}", x, y, z);
            break;
        }

        // check if trapped
        if !has_space(x as usize, y as usize, z as usize, n as usize, &grid) {
            println!("no space left: {} {} {}", x, y, z);
            break;
        }

        // take step in 1 dir
        let step = rand::thread_rng().gen_range(0..6) as i8;

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
        if step == 4 {
            next_z += 1;
        }
        if step == 5 {
            next_z -= 1;
        }

        // check if step is out of bounds
        if out_of_bounds(next_x, next_y, next_z, n) {
            println!("out of bounds: {} {} {}", next_x, next_y, next_z);
            continue;
        }

        // get next node
        let next_node = &grid[next_x as usize][next_y as usize][next_z as usize];

        // if node is filled, go again
        if next_node != "" {
            continue;
        }

        // generate monomer
        next_m = gen_monomer();

        // update grid
        grid[next_x as usize][next_y as usize][next_z as usize] = next_m.clone();

        // save order of monomer build
        connections.push((x, y, z, next_x, next_y, next_z));
        connections.push((next_x, next_y, next_z, x, y, z));
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
        z = next_z;
        println!("{},{},{}", x, y, z);
    }

    let unconnected = count_unconnected_neighbors(&grid, &connections, n);
    let data: Vec<Vec<Vec<i8>>> = grid
        .iter()
        .map(|row| {
            println!("{:?}", row);
            row.iter()
                .map(|col| {
                    col.iter()
                        .map(|v| {
                            if v == "P" {
                                return 1;
                            }
                            if v == "H" {
                                return 2;
                            }
                            0
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    // encode to json for matlab
    let mut writer = BufWriter::new(fold_file);
    serde_json::to_writer(&mut writer, &data).expect("Cant encode to json");

    // encode connections for matlab
    for connection in connections.iter() {
        let data = [
            connection.0.to_string(),
            connection.1.to_string(),
            connection.2.to_string(),
            connection.3.to_string(),
            connection.4.to_string(),
            connection.5.to_string(),
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
    writer2.flush().expect("Cant write connection data");
}

fn corner(x: i8, y: i8, z: i8, n: i8) -> bool {
    let x_bound = x == 0 || x == n - 1;
    let y_bound = y == 0 || y == n - 1;
    let z_bound = z == 0 || z == n - 1;

    x_bound && y_bound && z_bound
}

fn out_of_bounds(x: i8, y: i8, z: i8, n: i8) -> bool {
    let x = x as i8;
    let y = y as i8;
    let n = n as i8;

    x < 0 || y < 0 || z < 0 || x >= n || y >= n || z >= n
}

fn count_unconnected_neighbors(
    grid: &Vec<Vec<Vec<String>>>,
    connections: &Vec<(i8, i8, i8, i8, i8, i8)>,
    n: i8,
) -> i32 {
    let mut counter = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            for (k, node) in col.iter().enumerate() {
                if node == "H" {
                    let x = i as i8;
                    let y = j as i8;
                    let z = k as i8;

                    // check neighbors
                    if i + 1 < n as usize {
                        let next_x = (i + 1) as i8;
                        let next_y = y;
                        let next_z = z;

                        if grid[i + 1][j][k] == "H"
                            && !connections.contains(&(x, y, z, next_x, next_y, next_z))
                        {
                            counter += 1;
                        }
                    }
                    if j + 1 < n as usize {
                        let next_x = x;
                        let next_y = (j + 1) as i8;
                        let next_z = z;

                        if grid[i][j + 1][k] == "H"
                            && !connections.contains(&(x, y, z, next_x, next_y, next_z))
                        {
                            counter += 1;
                        }
                    }
                    if k + 1 < n as usize {
                        let next_x = x;
                        let next_y = y;
                        let next_z = (k + 1) as i8;

                        if grid[i][j][k + 1] == "H"
                            && !connections.contains(&(x, y, z, next_x, next_y, next_z))
                        {
                            counter += 1;
                        }
                    }
                    if i > 0 as usize {
                        let next_x = (i - 1) as i8;
                        let next_y = y;
                        let next_z = z;

                        if grid[i - 1][j][k] == "H"
                            && !connections.contains(&(x, y, z, next_x, next_y, next_z))
                        {
                            counter += 1;
                        }
                    }
                    if j > 0 as usize {
                        let next_x = x;
                        let next_y = (j - 1) as i8;
                        let next_z = z;

                        if grid[i][j - 1][k] == "H"
                            && !connections.contains(&(x, y, z, next_x, next_y, next_z))
                        {
                            counter += 1;
                        }
                    }
                    if k > 0 as usize {
                        let next_x = x;
                        let next_y = y;
                        let next_z = (k - 1) as i8;

                        if grid[i][j][k - 1] == "H"
                            && !connections.contains(&(x, y, z, next_x, next_y, next_z))
                        {
                            counter += 1;
                        }
                    }
                }
            }
        }
    }
    counter / 2
}

fn has_space(x: usize, y: usize, z: usize, n: usize, data: &Vec<Vec<Vec<String>>>) -> bool {
    let mut has_space = false;

    if x + 1 < n as usize {
        if data[x + 1][y][z] == "" {
            has_space = has_space || true;
        }
    }
    if y + 1 < n as usize {
        if data[x][y + 1][z] == "" {
            has_space = has_space || true;
        }
    }
    if z + 1 < n as usize {
        if data[x][y][z + 1] == "" {
            has_space = has_space || true;
        }
    }
    if x > 0 as usize {
        if data[x - 1][y][z] == "" {
            has_space = has_space || true;
        }
    }
    if y > 0 as usize {
        if data[x][y - 1][z] == "" {
            has_space = has_space || true;
        }
    }
    if z > 0 as usize {
        if data[x][y][z - 1] == "" {
            has_space = has_space || true;
        }
    }
    has_space
}
