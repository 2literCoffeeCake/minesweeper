use super::Positition;

pub struct Mine {
    column: usize,
    row: usize,
    is_bomb: bool,
    revealed: bool,
}

impl Mine {
    pub fn new() -> MineBuilder {
        MineBuilder {
            column: None,
            row: None,
            is_bomb: None
        }
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }

    pub fn is_bomb(self) -> bool {
        self.is_bomb
    }

    pub fn get_position(self) -> Positition {
        Positition::new().column(self.column).row(self.row).build()
    }
}

pub struct MineBuilder {
    column: Option<usize>,
    row: Option<usize>,
    is_bomb: Option<bool>,
}

impl MineBuilder {
    pub fn row(&mut self, row: usize) -> &mut Self {
        self.row = Some(row);
        self
    }

    pub fn column(&mut self, column: usize) -> &mut Self {
        self.row = Some(column);
        self
    }

    pub fn is_bomb(&mut self, is_bomb: bool) -> &mut Self {
        self.is_bomb = Some(is_bomb);
        self
    }

    pub fn build(&mut self) -> Mine {
        Mine {
            column: self.column.unwrap_or(0),
            row: self.row.unwrap_or(0),
            is_bomb: self.is_bomb.unwrap_or_default(),
            revealed: false,
        }
    }
}
