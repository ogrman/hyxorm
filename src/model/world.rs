use rand;

#[derive(PartialEq)]
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
    cells: Vec<i32>,
}

fn index(width: usize, x: usize, y: usize) -> usize {
    y * width + x
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let size = (width * height) as usize;
        let mut cells = vec![0; size];

        for y in 0..height {
            for x in 0..width {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    cells[index(width, x, y)] = 1;
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

    pub fn get_cell(&self, x: usize, y: usize) -> i32 {
        self.cells[index(self.width, x, y)]
    }

    pub fn check_collision(&self, x: usize, y: usize) -> CellContent {
        match self.get_cell(x, y) {
            0 => CellContent::Nothing,
            1 => CellContent::Wall,
            2 => CellContent::Nugget,
            _ => CellContent::Nothing,
        }
    }

    pub fn consume_nugget(&mut self) -> () {
        self.cells[index(self.width, self.nugget_x, self.nugget_y)] = 0;
    }

    pub fn spawn_nugget(&mut self) -> () {
        'l: loop {
            let x = rand::random::<usize>() % self.width;
            let y = rand::random::<usize>() % self.height;

            if self.check_collision(x, y) == CellContent::Nothing {
                self.cells[index(self.width, x, y)] = 2;
                self.nugget_x = x;
                self.nugget_y = y;
                break 'l
            }
        }
    }
}
