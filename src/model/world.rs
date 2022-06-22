use rand;

#[derive(Clone, Copy, PartialEq)]
pub enum CellContent {
    Nothing,
    Wall,
    Nugget,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    nugget_x: usize,
    nugget_y: usize,
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
            nugget_x: 0,
            nugget_y: 0,
            cells: cells,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> CellContent {
        self.cells[index(self.width, x, y)]
    }

    pub fn consume_nugget(&mut self) -> () {
        self.cells[index(self.width, self.nugget_x, self.nugget_y)] = CellContent::Nothing;
    }

    pub fn spawn_nugget(&mut self) -> () {
        'l: loop {
            let x = rand::random::<usize>() % self.width;
            let y = rand::random::<usize>() % self.height;

            if self.get_cell(x, y) == CellContent::Nothing {
                self.cells[index(self.width, x, y)] = CellContent::Nugget;
                self.nugget_x = x;
                self.nugget_y = y;
                break 'l
            }
        }
    }
}
