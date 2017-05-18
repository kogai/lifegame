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

    fn should_live(&self, grid: &Matrix<Cell>) -> bool {
        use self::Cell::*;
        match self {
            &Alive((x, y)) |
            &Death((x, y)) => {
                // let xx = &grid[1];
                true
            },
        };
        unimplemented!();
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
