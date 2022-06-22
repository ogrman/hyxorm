use rand;

use super::snake::Position;

#[derive(Clone, Copy, PartialEq)]
pub enum CellContent {
    Nothing,
    Wall,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    cells: Vec<CellContent>,
    nuggets: Vec<Position>,
}

fn index(width: usize, x: usize, y: usize) -> usize {
    y * width + x
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let size = (width * height) as usize;
        let mut cells = vec![CellContent::Nothing; size];

        for y in 0..height {
            for x in 0..width {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    cells[index(width, x, y)] = CellContent::Wall;
                }
            }
        }

        World {
            width,
            height,
            cells,
            nuggets: Default::default(),
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> CellContent {
        self.cells[index(self.width, x, y)]
    }

    pub fn has_nugget(&self, x: usize, y: usize) -> bool {
        self.nuggets.iter().any(|n| n.x == x && n.y == y)
    }

    pub fn consume_nugget(&mut self, p: &Position) {
        if let Some((to_remove, _)) = self
            .nuggets
            .iter()
            .enumerate()
            .find(|(_, n)| n.x == p.x && n.y == p.y)
        {
            self.nuggets.remove(to_remove);
        }
    }

    pub fn spawn_nugget(&mut self) -> () {
        'l: loop {
            let x = rand::random::<usize>() % self.width;
            let y = rand::random::<usize>() % self.height;

            if self.get_cell(x, y) == CellContent::Nothing && !self.has_nugget(x, y) {
                self.nuggets.push(Position { x, y });
                break 'l
            }
        }
    }
}
