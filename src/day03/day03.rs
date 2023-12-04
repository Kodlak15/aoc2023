use crate::read_input;

// Can't seem to find a way to alter the schematic while iterating over it, so need to find another
// way to keep track of what cells have been visited

#[derive(Debug)]
struct Cell {
    row: usize,
    col: usize,
}

struct Engine {
    schematic: Vec<Vec<char>>,
}

impl Engine {
    fn from(input: &str) -> Self {
        let mut schematic: Vec<Vec<char>> = vec![];
        for (i, line) in input.lines().enumerate() {
            schematic.push(line.chars().collect::<Vec<char>>());
            // Add an additional column to prevent index overload when reading number
            schematic[i].push('.')
        }

        Engine { schematic }
    }

    fn rows(&self) -> i8 {
        self.schematic.len().try_into().unwrap()
    }

    fn cols(&self) -> i8 {
        self.schematic[0].len().try_into().unwrap()
    }

    fn get(&self, row: usize, col: usize) -> char {
        let row: usize = row.into();
        let col: usize = col.into();
        self.schematic[row][col]
    }

    fn adjacent_cells(&self, row: i8, col: i8) -> Vec<Cell> {
        let mut cells: Vec<Cell> = vec![];
        for i in row - 1..row + 2 {
            for j in col - 1..col + 2 {
                if i >= 0 && i < self.rows() && j >= 0 && j < self.cols() {
                    cells.push(Cell {
                        row: i.try_into().unwrap(),
                        col: j.try_into().unwrap(),
                    })
                }
            }
        }

        cells
    }
}

#[allow(dead_code)]
fn is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c == '.')
}

#[allow(dead_code)]
fn pt1(input: &str) -> u32 {
    let mut part_nums: Vec<(usize, usize, usize)> = vec![];
    let engine = Engine::from(input);

    for (i, row) in engine.schematic.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if is_symbol(*c) {
                let row: i8 = i.try_into().unwrap();
                let col: i8 = j.try_into().unwrap();
                let cells = engine.adjacent_cells(row, col);
                for cell in cells {
                    if engine.get(cell.row, cell.col).is_digit(10) {
                        // println!("{:?}", cell);
                        // Walk column index backwards until start of num is found
                        let mut start_col: i8 = cell.col.try_into().unwrap();
                        while start_col >= 0
                            && engine
                                .get(cell.row, start_col.try_into().unwrap())
                                .is_digit(10)
                        {
                            start_col -= 1;
                        }

                        start_col += 1;
                        let mut end_col = start_col;
                        while engine
                            .get(cell.row, end_col.try_into().unwrap())
                            .is_digit(10)
                        {
                            end_col += 1;
                        }

                        // println!("Start col: {}", start_col);
                        // println!("End col  : {}", end_col);

                        // Push part_num to part_nums
                        let data = (
                            row.try_into().unwrap(),
                            start_col.try_into().unwrap(),
                            end_col.try_into().unwrap(),
                        );

                        if !part_nums.contains(&data) {
                            part_nums.push(data);
                        }

                        println!("Part nums: {:?}", part_nums);
                    }
                }
            }
        }
    }

    0
}

#[allow(dead_code)]
fn pt2(_input: &str) -> u32 {
    0
}

pub fn day03() {
    let input = read_input("./src/day02/puzzle_input.txt");
    println!("Day 3:");
    println!("Part 1: {}", pt1(&input));
    // println!("Part 2: {}", pt2(&input));
    println!("-------------------------------------------------------")
}

#[test]
fn test_pt1() {
    let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    assert_eq!(pt1(input), 4361);
}

// #[test]
// fn test_pt2() {
//     unimplemented!()
// }
