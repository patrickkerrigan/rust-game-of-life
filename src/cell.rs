use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Cell {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

pub struct ProtoCell {
    pub red: u16,
    pub green: u16,
    pub blue: u16
}

impl ProtoCell {
    pub fn new() -> Self {
        ProtoCell {
            red: 0,
            green: 0,
            blue: 0
        }
    }

    pub fn add(&mut self, cell: &Cell) {
        self.red += cell.red as u16;
        self.green += cell.green as u16;
        self.blue += cell.blue as u16;
    }

    pub fn into_cell(self, neighbours: u16) -> Cell {
        Cell {
            red: self.red.div(neighbours) as u8,
            green: self.green.div(neighbours) as u8,
            blue: self.blue.div(neighbours) as u8,
        }
    }
}
