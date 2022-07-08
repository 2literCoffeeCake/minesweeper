use super::Mine;

pub struct Minesfield{
    mines: Vec<Vec<Mine>>,
    size: usize,
}

impl Minesfield{
    pub fn new(size: usize) -> Self{
        for row in 0..size{
            let mut vec: Vec<Mine> = Vec::new();
            for column in 0..size{
    



            }
        }
        todo!();
    }
}