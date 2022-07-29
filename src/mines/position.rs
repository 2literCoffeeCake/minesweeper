use std::fmt::Display;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
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
            row: rand::thread_rng().gen_range(0..=(max_value-1)),
            column: rand::thread_rng().gen_range(0..=(max_value-1)),
        }
    }

    pub fn get_neighbors(&self) -> Vec<Self>{
        let mut result = Vec::new();
        let row = self.row as i8;
        let column = self.column as i8;
        for delta_row in -1..=1 {
            for delta_column in -1..=1 {
                if delta_column != 0 || delta_row != 0 {
                    let tmp_row = row + delta_row;
                    let tmp_column = column + delta_column;
                    if tmp_column >= 0 && tmp_row >= 0 {
                        result.push(Self{
                            column: tmp_column as usize,
                            row: tmp_row as usize
                        });
                    }
                }
            }   
        }
        result
    }

    pub fn equals(&self, pos: &Self) -> bool{
        self.column == pos.column && self.row == pos.row
    }
    
}

impl Display for Position{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{row}/{column}", row=self.row, column=self.column)
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

    pub fn build(&mut self) -> Position {
        Position {
            row: self.row.unwrap_or(0),
            column: self.column.unwrap_or(0),
        }
    }
}