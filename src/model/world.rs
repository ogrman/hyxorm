use rand;

use model::snake::Position;
use model::snake::Snake;

#[derive(PartialEq, Clone)]
pub enum CellContent {
    Nothing,
    Wall,
    Nugget,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    nugget: Position,
    cells: Vec<CellContent>,
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
            width: width,
            height: height,
            nugget: Position { x: 0, y: 0 },
            cells: cells,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> CellContent {
        self.cells[index(self.width, x, y)].clone()
    }

    pub fn consume_nugget(&mut self) -> () {
        self.cells[index(self.width, self.nugget.x, self.nugget.y)] = CellContent::Nothing;
    }

    pub fn spawn_nugget(&mut self, snake: &Snake) -> () {
        'l: loop {
            let x = rand::random::<usize>() % self.width;
            let y = rand::random::<usize>() % self.height;

            let is_nothing = self.get_cell(x, y) == CellContent::Nothing;
            let is_snake_here = snake.is_here(Position { x: x, y: y });

            if is_nothing && !is_snake_here {
                self.cells[index(self.width, x, y)] = CellContent::Nugget;
                self.nugget.x = x;
                self.nugget.y = y;
                break 'l;
            }
        }
    }
}
