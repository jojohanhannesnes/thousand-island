pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut grids: Vec<Vec<char>> = Vec::new();
    for i in minefield {
        let mut rows = Vec::new();
        for x in i.chars() {
            rows.push(x);
        }
        grids.push(rows);
    }
    fn print_2d_array(grid: &[Vec<char>]) {
        for row in grid {
            for element in row {
                print!("{:?}", element);
            }
            println!();
        }
    }

    print_2d_array(&grids);
    for rows in 0..grids.len() {
        for cols in 0..grids[rows].len() {
            // calculate bombs
            let count = calculate_bomb(rows, cols, &grids);
            println!(
                "grid {} {} = {:?}, have total bombs nearby {}\n",
                rows, cols, grids[rows][cols], count
            );
        }
        println!();
    }

    grids
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn calculate_bomb(rows: usize, cols: usize, grid: &[Vec<char>]) -> i32 {
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let new_row = rows as isize + dr;
            let new_col = cols as isize + dc;

            if 0 <= new_row
                && new_row < rows as isize
                && 0 <= new_col
                && new_col < cols as isize
                && grid[new_row as usize][new_col as usize] == '*'
            {
                count += 1;
            }
        }
    }

    count
}
