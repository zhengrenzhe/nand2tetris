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

    pub fn preview(&self) -> usize {
        self.start
    }

    pub fn has_next(&self) -> bool {
        self.start < self.max
    }

    pub fn next(&mut self) {
        self.start += 1
    }
}
