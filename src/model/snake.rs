#[derive(Clone)]
pub struct SnakeSegment {
    pub pos: Position,
    pub direction: Direction,
}

impl SnakeSegment {
    pub fn new(pos: Position, direction: Direction) -> SnakeSegment {
        SnakeSegment { pos, direction }
    }

    pub fn move_fwd(&mut self) {
        self.pos = self.next_pos();
    }

    pub fn next_pos(&self) -> Position {
        match self.direction {
            Direction::Up => Position {
                y: self.pos.y - 1,
                ..self.pos
            },
            Direction::Right => Position {
                x: self.pos.x + 1,
                ..self.pos
            },
            Direction::Down => Position {
                y: self.pos.y + 1,
                ..self.pos
            },
            Direction::Left => Position {
                x: self.pos.x - 1,
                ..self.pos
            },
            Direction::Still => self.pos,
        }
    }

    pub fn turn(&mut self, direction: Direction) {
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

#[derive(Clone)]
pub struct Snake {
    pub segments: Vec<SnakeSegment>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Snake {
    pub fn new(pos: Position, dir: Direction, length: usize) -> Snake {
        let mut segments = vec![SnakeSegment::new(pos, dir)];

        for _ in 1..length {
            segments.push(SnakeSegment::new(pos, Direction::Still));
        }

        Snake { segments }
    }

    pub fn grow(&mut self) {
        let pos = self.last_segment_pos();
        self.segments.push(SnakeSegment::new(pos, Direction::Still));
    }

    pub fn move_fwd(&mut self) {
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
        self.segments.last().unwrap().pos
    }

    pub fn turn(&mut self, dir: Direction) {
        self.segments[0].turn(dir);
    }

    pub fn is_here(&self, p: &Position) -> bool {
        self.segments.iter().any(|s| &s.pos == p)
    }
}
