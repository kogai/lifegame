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
    pub fn new(x: i32, y: i32) -> Self {
        World { grid: (0..y).map(|_| (0..x).map(|_| Cell::death()).collect()).collect::<Vec<_>>() }
    }

    pub fn show(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().map(|x| x.show()).collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
