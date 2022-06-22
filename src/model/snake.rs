pub struct SnakeSegment {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

impl SnakeSegment {
    pub fn new(x: usize, y: usize, direction: Direction) -> SnakeSegment {
        SnakeSegment {
            x: x,
            y: y,
            direction: direction,
        }
    }

    pub fn move_fwd(&mut self) -> () {
        let np = self.next_pos();
        self.x = np.x;
        self.y = np.y;
    }

    pub fn pos(&self) -> Position {
        Position {x: self.x, y: self.y}
    }

    pub fn next_pos(&self) -> Position {
        match self.direction {
            Direction::Up => Position {x: self.x, y: self.y - 1},
            Direction::Right => Position {x: self.x + 1, y: self.y},
            Direction::Down => Position {x: self.x, y: self.y + 1},
            Direction::Left => Position {x: self.x - 1, y: self.y},
            Direction::Still => Position {x: self.x, y: self.y},
        }
    }

    pub fn turn(&mut self, direction: Direction) -> () {
        self.direction = direction;
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    Still,
}

pub struct Snake {
    pub segments: Vec<SnakeSegment>,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Snake {
    pub fn new(x: usize, y: usize, dir: Direction, length: usize) -> Snake {
        let mut segments = vec![SnakeSegment::new(x, y, dir)];

        for _ in 1..length {
            segments.push(SnakeSegment::new(x, y, Direction::Still));
        }

        Snake {
            segments: segments,
        }
    }

    pub fn grow(&mut self) -> () {
        let pos = self.last_segment_pos();
        self.segments.push(SnakeSegment::new(pos.x, pos.y, Direction::Still));
    }

    pub fn move_fwd(&mut self) -> () {
        let mut next_dir = None::<Direction>;

        for segm in self.segments.iter_mut() {
            if next_dir.is_none() {
                next_dir = Some(segm.direction);
            }

            segm.move_fwd();

            let current_dir = segm.direction;
            segm.turn(next_dir.unwrap());
            next_dir = Some(current_dir);
        }
    }

    pub fn next_head_pos(&self) -> Position {
        self.segments[0].next_pos()
    }

    pub fn last_segment_pos(&self) -> Position {
        self.segments.last().unwrap().pos()
    }

    pub fn turn(&mut self, dir: Direction) -> () {
        self.segments[0].turn(dir);
    }

    pub fn contains(&self, p: &Position) -> bool {
        self.segments.iter().any(|s| s.x == p.x && s.y == p.y)
    }
}
