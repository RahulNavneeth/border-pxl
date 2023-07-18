use std::{env, io::BufRead};

type Pxl = Vec<Vec<char>>;

fn is_valid(pxl: &Pxl, row: usize, col: usize) -> bool {
    let rows = pxl.len();
    let cols = pxl[0].len();
    row >= 0 as usize && row < rows && col >= 0 as usize && col < cols
}

fn is_border(pxl: &Pxl, row: usize, col: usize) -> bool {
    row == 0 || row == pxl.len() - 1 || col == 0 || col == pxl[0].len() - 1
}

fn is_black(cell: char) -> bool {
    cell == '1'
}

fn display(pxl: &Pxl, visited: &Vec<(usize, usize)>) {
    pxl.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, &cell)| {
            if !visited.contains(&(x, y)) && is_black(cell) {
                print!("{} ", 0);
            } else {
                print!("{} ", cell);
            }
        });
        println!();
    });
}

fn pxledge(pxl: &mut Pxl) {
    let mut visited: Vec<(usize, usize)> = Vec::new();
    pxl.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, &cell)| {
            if is_valid(pxl, x, y)
                && is_border(pxl, x, y)
                && is_black(cell)
                && !visited.contains(&(x, y))
            {
                let mut stack: Vec<(usize, usize)> = Vec::new();
                stack.push((x, y));
                visited.push((x, y));
                while let Some((i, j)) = stack.pop() {
                    let dir: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
                    dir.iter().for_each(|&(k, l)| {
                        let new_i = (i as isize + k) as usize;
                        let new_j = (j as isize + l) as usize;
                        if is_valid(pxl, new_i, new_j) {
                            let new_cell = pxl[new_i][new_j];
                            if is_black(new_cell) && !visited.contains(&(new_i, new_j)) {
                                stack.push((new_i, new_j));
                                visited.push((new_i, new_j))
                            }
                        }
                    })
                }
            }
        })
    });
    display(pxl, &visited)
}

fn main() -> std::io::Result<()> {
    let mut pxl: Pxl = Vec::new();
    let args = env::args().collect::<Vec<String>>();
    let fs = std::fs::OpenOptions::new()
        .read(true)
        .open(args.get(1).map_or("./data.txt", |f| &*f))?;
    let f = std::io::BufReader::new(fs);

    f.lines().for_each(|x| match x {
        Ok(value) => {
            let c = value
                .chars()
                .enumerate()
                .filter_map(|(key, f)| if key % 2 == 0 { Some(f) } else { None })
                .collect::<Vec<char>>();
            pxl.push(c);
        }
        Err(e) => println!("ERROR {:?}", e),
    });
    pxledge(&mut pxl);
    Ok(())
}
