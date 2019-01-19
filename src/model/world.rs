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
    pub score: usize,
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
            nugget: Position { x: 0, y: 0 },
            cells,
            score: 0,
        }
    }

    pub fn get_cell(&self, p: &Position) -> CellContent {
        self.cells[index(self.width, p.x, p.y)].clone()
    }

    fn set_cell(&mut self, p: &Position, content: CellContent) {
        self.cells[index(self.width, p.x, p.y)] = content;
    }

    pub fn consume_nugget(&mut self) {
        let nugget = self.nugget;
        self.score += 1;
        self.set_cell(&nugget, CellContent::Nothing);
    }

    pub fn spawn_nugget(&mut self, snake: &Snake) {
        'l: loop {
            let p = Position {
                x: rand::random::<usize>() % self.width,
                y: rand::random::<usize>() % self.height,
            };

            let is_nothing = self.get_cell(&p) == CellContent::Nothing;
            let is_snake_here = snake.is_here(&p);

            if is_nothing && !is_snake_here {
                self.set_cell(&p, CellContent::Nugget);
                self.nugget = p;
                break 'l;
            }
        }
    }
}
