use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Positition {
    row: usize,
    column: usize,
}

impl Positition {
    pub fn new() -> PositionBuilder {
        PositionBuilder {
            row: None,
            column: None,
        }
    }

    pub fn get_row(self) -> usize {
        self.row
    }

    pub fn get_column(self) -> usize {
        self.column
    }

    pub fn get_random(max_value: usize) -> Self {
        Self {
            row: rand::thread_rng().gen_range(0..=max_value),
            column: rand::thread_rng().gen_range(0..=max_value),
        }
    }

    pub fn equals(&self, pos: &Positition) -> bool{
        self.column == pos.column && self.row == pos.row
    }
}

pub struct PositionBuilder {
    row: Option<usize>,
    column: Option<usize>,
}

impl PositionBuilder {
    pub fn row(&mut self, row: usize) -> &mut Self {
        self.row = Some(row);
        self
    }

    pub fn column(&mut self, column: usize) -> &mut Self {
        self.column = Some(column);
        self
    }

    pub fn build(&mut self) -> Positition {
        Positition {
            row: self.row.unwrap_or(0),
            column: self.column.unwrap_or(0),
        }
    }
}
