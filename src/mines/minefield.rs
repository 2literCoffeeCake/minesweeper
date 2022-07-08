use super::{Mine, Positition};

pub struct Minefield{
    mines: Vec<Vec<Mine>>,
    size: usize,
}

impl Minefield{
    pub fn new(size: usize, amount_bombs: usize) -> Self{
        let mut mines: Vec<Vec<Mine>> = Vec::new();
        let mut bombs: Vec<Positition> = Vec::new();
        for _ in 0..amount_bombs{
            bombs.push(Positition::get_random(size));
        }
        for row in 0..size{
            let mut vec: Vec<Mine> = Vec::new();
            for column in 0..size{
                let pos = Positition::new().row(row).column(column).build();
                let is_bomb = bombs.iter().any(|bomb_pos|bomb_pos.equals(&pos));
                let mine = Mine::new().position(&pos).is_bomb(is_bomb).build();
                vec.push(mine);
            }
            mines.push(vec);
        }
        Self{
            mines,
            size
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Minefield;

    #[test]
    fn it_works() {
        Minefield::new(8, 9);
    }
}
