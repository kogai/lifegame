pub type Matrix<T> = Vec<Vec<T>>;
type Position = (i32, i32);

#[derive(Debug, Clone)]
pub enum Cell {
    Alive(Position),
    Death(Position),
}

impl Cell {
    fn alive(x: Position) -> Self {
        Cell::Alive(x)
    }

    fn death(x: Position) -> Self {
        Cell::Death(x)
    }

    fn show(&self) -> String {
        use self::Cell::*;
        match self {
                &Alive(_) => "1",
                &Death(_) => "0",
            }
            .to_owned()
    }

    fn is_alive(&self) -> bool {
        use self::Cell::*;
        match self {
            &Alive(_) => true,
            &Death(_) => false,
        }
    }

    fn should_live(&self, grid: &Matrix<Cell>) -> bool {
        use self::Cell::*;
        match self {
            &Alive((x, y)) | &Death((x, y)) => {
                let arround = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1),
                                   (1, 1)];
                let lifes = arround.iter()
                    .map(|&(pos_x, pos_y)| {
                        if let Some(row) = grid.iter().nth((y + pos_y) as usize) {
                            if let Some(cell) = row.iter().nth((x + pos_x) as usize) {
                                return cell.is_alive();
                            }
                        }
                        false
                    })
                    .collect::<Vec<_>>();
                
                
                let alives: Vec<bool> = lifes.into_iter().filter(|x| x.clone()).collect();

                if self.is_alive() {
                    alives.len() == 2 || alives.len() == 3
                } else {
                    alives.len() == 3
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct World {
    grid: Matrix<Cell>,
}

impl World {
    pub fn new(column: i32, row: i32, initial_state: Vec<Position>) -> Self {
        World {
            grid: (0..row)
                .map(|y| {
                    (0..column)
                        .map(|x| {
                            for &(initial_x, initial_y) in &initial_state {
                                if initial_x == x && initial_y == y {
                                    return Cell::alive((x, y));
                                }
                            }
                            Cell::death((x, y))
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn tick(&mut self) {
        let grid = &self.grid.clone();

        self.grid = self.grid
            .iter()
            .map(|y| {
                y.iter()
                    .map(|x| if x.should_live(grid) {
                        match x {
                            &Cell::Alive(cell) |
                            &Cell::Death(cell) => Cell::alive(cell),
                        }
                    } else {
                        match x {
                            &Cell::Alive(cell) |
                            &Cell::Death(cell) => Cell::death(cell),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
    }

    pub fn show(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().map(|x| x.show()).collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
