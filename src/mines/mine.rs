use super::Position;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mine {
    column: usize,
    row: usize,
    is_bomb: bool,
    state: MineState,
    amount_neighbors: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MineState {
    Unknown,
    Marked,
    Revealed,
}

impl Display for MineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MineState::Unknown => write!(f, "Unknown"),
            MineState::Marked => write!(f, "Marked Level"),
            MineState::Revealed => write!(f, "Revealed"),
        }
    }
}

impl Mine {
    pub fn set_state(&mut self, state: MineState) {
        if self.state != MineState::Revealed {
            self.state = state;
        }
    }

    pub fn reveal(&mut self) {
        self.set_state(MineState::Revealed);
    }

    pub fn get_state(self) -> MineState {
        self.state
    }

    pub fn is_bomb(self) -> bool {
        self.is_bomb
    }

    pub fn get_position(self) -> Position {
        Position::new().column(self.column).row(self.row).build()
    }

    pub fn get_number_of_neighboring_bombs(&self) -> usize {
        self.amount_neighbors
    }

    pub fn has_no_neighbors(&self) -> bool{
        self.amount_neighbors == 0
    }

    pub fn generate_mines(rows: usize, columns: usize, amount_bombs: usize) -> Vec<Self> {
        let mut mines: Vec<Mine> = Vec::new();

        let (bombs, neighbors) = generate_bombs(rows, columns, amount_bombs);

        for row in 0..rows {
            for column in 0..columns {
                let pos = Position::new().row(row).column(column).build();
                let is_bomb = bombs.iter().any(|bomb_pos| bomb_pos.equals(&pos));
                let amount_neighbors = neighbors
                    .iter()
                    .filter(|mine_pos| mine_pos.equals(&pos))
                    .count();
                let mine = Mine {
                    column,
                    row,
                    is_bomb,
                    state: MineState::Unknown,
                    amount_neighbors,
                };
                mines.push(mine);
            }
        }
        mines
    }

    pub fn generate_preview() -> Vec<Self>{
        let mut mines: Vec<Mine> = Vec::new();
        mines.push(Mine {
            row: 0,
            column: 0,
            is_bomb: false,
            state: MineState::Revealed,
            amount_neighbors: 0,
        });

        mines.push(Mine {
            row: 0,
            column: 1,
            is_bomb: true,
            state: MineState::Revealed,
            amount_neighbors: 0,
        });

        mines.push(Mine {
            row: 1,
            column: 0,
            is_bomb: true,
            state: MineState::Marked,
            amount_neighbors: 0,
        });

        mines.push(Mine {
            row: 1,
            column: 1,
            is_bomb: false,
            state: MineState::Revealed,
            amount_neighbors: 2,
        });

        mines
    }
}

fn generate_bombs(max_row: usize, max_column: usize, amount_bombs: usize) -> (Vec<Position>, Vec<Position>) {
    let mut bombs: Vec<Position> = Vec::new();
    let mut neighbors: Vec<Position> = Vec::new();
    loop {
        if bombs.len() >= amount_bombs {
            break;
        }
        let bomb = Position::get_random(max_row, max_column);
        if !bombs.iter().any(|bomb_pos| bomb_pos.equals(&bomb)) {
            neighbors.append(&mut bomb.get_neighbors());
            bombs.push(bomb);
        }
    }
    (bombs, neighbors)
}

pub trait MineVec {
    fn get_by_pos(&mut self, pos: &Position) -> Option<&mut Mine>;
    fn reveal_all(&mut self);
    fn reveal_neighbors(&mut self, pos: &Position);
}

impl MineVec for Vec<Mine> {
    fn get_by_pos(&mut self, pos: &Position) -> Option<&mut Mine> {
        self.iter_mut()
            .find(|mine| mine.get_position().equals(&pos))
    }

    fn reveal_all(&mut self) {
        for mine in self {
            mine.reveal();
        }
    }

    fn reveal_neighbors(&mut self, pos: &Position) {
        let pos_vec = reveal_neighbors_inner(&pos, self);
        for inner_pos in pos_vec {
            let some_vec = reveal_neighbors_inner(&inner_pos, self);
            for outer_pos in some_vec {
                self.reveal_neighbors(&outer_pos);
            }
        }
    }
}

fn reveal_neighbors_inner(pos: &Position, minefield: &mut Vec<Mine>) -> Vec<Position> {
    let neighbors = pos.get_neighbors();
    let mut tmp = Vec::<Position>::new();
    minefield
        .iter_mut()
        .filter(|mine| {
            neighbors
                .iter()
                .position(|p| p.equals(&mine.get_position()))
                .is_some()
        })
        .for_each(|mine| {
            if mine.get_state() != MineState::Revealed && mine.has_no_neighbors() {
                tmp.push(mine.get_position());
            }
            mine.reveal();
        });
    tmp
}
