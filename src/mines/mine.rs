use super::Positition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mine {
    column: usize,
    row: usize,
    is_bomb: bool,
    revealed: bool,
}

impl Mine {
    pub fn _reveal(&mut self) {
        self.revealed = true;
    }

    pub fn is_bomb(self) -> bool {
        self.is_bomb
    }

    pub fn get_position(self) -> Positition {
        Positition::new().column(self.column).row(self.row).build()
    }

    pub fn generate_mines(size: usize, amount_bombs: usize) -> Vec<Self>{
        let mut mines: Vec<Mine> = Vec::new();
        let mut bombs: Vec<Positition> = Vec::new();

        loop{
            if bombs.len() > amount_bombs {
                break;
            }
            let bomb = Positition::get_random(size);
            if !bombs.iter().any(|bomb_pos|bomb_pos.equals(&bomb)){
                bombs.push(bomb);
            }
        }

        for row in 0..size{
            for column in 0..size{
                let pos = Positition::new().row(row).column(column).build();
                let is_bomb = bombs.iter().any(|bomb_pos|bomb_pos.equals(&pos));
                let mine = Mine { 
                    column, 
                    row, 
                    is_bomb, 
                    revealed: false 
                };
                mines.push(mine);
            }
        }
        mines

    }
}
