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
    nuggets: Vec<Position>,
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
            nuggets: Vec::new(),
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

    pub fn consume_nugget(&mut self, np: &Position) {
        let maybe_nugget = self.nuggets.drain_filter(|p| p == np);
        for _ in maybe_nugget {
            self.score += 1;
            // Why can't i do this instead???
            // self.set_cell(&nugget, CellContent::Nothing);
            self.cells[index(self.width, np.x, np.y)] = CellContent::Nothing;
        }
    }

    fn count_nothings(&self) -> usize {
        self.cells
            .iter()
            .filter(|c| **c == CellContent::Nothing)
            .count()
    }

    pub fn spawn_nugget(&mut self, snake: &Snake, next_pos: Option<&Position>) {
        let free_cells = (self.count_nothings() - next_pos.iter().count() - snake.len()) as i32;
        if free_cells >= 1 {
            'l: loop {
                let p = Position {
                    x: rand::random::<usize>() % self.width,
                    y: rand::random::<usize>() % self.height,
                };

                let is_nothing = self.get_cell(&p) == CellContent::Nothing;
                let is_snake_here = snake.is_here(&p);
                let is_next_pos = next_pos.map_or(false, |np| *np == p);

                if is_nothing && !is_snake_here && !is_next_pos {
                    self.set_cell(&p, CellContent::Nugget);
                    self.nuggets.push(p);
                    break 'l;
                }
            }
        }
    }
}
