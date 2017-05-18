pub type Matrix<T> = Vec<Vec<T>>;

#[derive(Debug)]
pub enum Cell {
    Alive,
    Death,
}

impl Cell {
    fn alive() -> Self {
        Cell::Alive
    }

    fn death() -> Self {
        Cell::Death
    }

    fn show(&self) -> String {
        use self::Cell::*;
        match self {
                &Alive => "1",
                &Death => "0",
            }
            .to_owned()
    }
}

#[derive(Debug)]
pub struct World {
    grid: Matrix<Cell>,
}

impl World {
    pub fn new(column: i32, row: i32, initial_state: Vec<(i32, i32)>) -> Self {
        World {
            grid: (0..row)
                .map(|y| {
                    (0..column)
                        .map(|x| {
                            for &(initial_x, initial_y) in &initial_state {
                                if initial_x == x && initial_y == y {
                                    return Cell::alive();
                                }
                            }
                            Cell::death()
                        })
                        .collect()
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn tick(&self) {
        unimplemented!();
    }

    pub fn show(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().map(|x| x.show()).collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
