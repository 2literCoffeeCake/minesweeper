use std::fmt::Display;
use super::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mine {
    column: usize,
    row: usize,
    is_bomb: bool,
    state: MineState,
    amount_neighbors: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MineState{
    Unknown,
    Marked(usize),
    Revealed
}

impl Display for MineState{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            MineState::Unknown => write!(f, "Unknown"),
            MineState::Marked(level) => write!(f, "Marked Level: {level}"),
            MineState::Revealed => write!(f, "Revealed"),
        }
        
    }
}

impl Mine {

    pub fn set_state(&mut self, state: MineState){
        if self.state != MineState::Revealed{
            let state = match state{
                MineState::Marked(level) => {
                    let mut state = MineState::Marked(level);
                    if level > 2 || level < 1 {
                        state =  MineState::Unknown;
                    }
                    state
                },
                state => state
            };
            self.state = state;
        }
    }

    pub fn get_state(self) -> MineState{
        self.state
    }

    pub fn is_bomb(self) -> bool {
        self.is_bomb
    }

    pub fn get_position(self) -> Position {
        Position::new().column(self.column).row(self.row).build()
    }

    pub fn get_amount_neighbors(&self) -> usize{
        self.amount_neighbors
    }

    pub fn generate_mines(size: usize, amount_bombs: usize) -> Vec<Self>{
        let mut mines: Vec<Mine> = Vec::new();
        let mut bombs: Vec<Position> = Vec::new();
        let mut neighbors: Vec<Position> = Vec::new();
        loop{
            if bombs.len() >= amount_bombs {
                break;
            }
            let bomb = Position::get_random(size);
            if !bombs.iter().any(|bomb_pos|bomb_pos.equals(&bomb)){
                neighbors.append(&mut bomb.get_neighbors());
                bombs.push(bomb);
            }
        }

        for row in 0..size{
            for column in 0..size{
                let pos = Position::new().row(row).column(column).build();
                let is_bomb = bombs.iter().any(|bomb_pos|bomb_pos.equals(&pos));
                let amount_neighbors = neighbors.iter().filter(|mine_pos|mine_pos.equals(&pos)).count();
                let mine = Mine { 
                    column, 
                    row, 
                    is_bomb, 
                    state: MineState::Unknown,
                    amount_neighbors
                };
                mines.push(mine);
            }
        }
        mines

    }
}
