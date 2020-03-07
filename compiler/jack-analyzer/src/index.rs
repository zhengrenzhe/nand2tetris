pub struct Index {
    pub start: usize,
    max: usize,
}

impl Index {
    pub fn new(start: usize, max: usize) -> Index {
        Index { start, max }
    }

    pub fn get(&mut self) -> usize {
        let cur = self.start;
        self.start += 1;
        cur
    }

    pub fn prev(&self) -> usize {
        self.start - 1
    }

    pub fn preview(&self) -> usize {
        self.start
    }

    pub fn has_next(&self) -> bool {
        self.start < self.max - 1
    }
}
