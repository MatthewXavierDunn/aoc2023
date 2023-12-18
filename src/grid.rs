#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self {
            rows: 0,
            cols: 0,
            data: Vec::new(),
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.data
    }

    pub fn clear(&mut self) {
        self.rows = 0;
        self.cols = 0;
        self.data.clear();
    }

    pub fn push_row(&mut self, row: impl Iterator<Item = T>) {
        self.rows += 1;
        let mut len = 0;
        for item in row {
            self.data.push(item);
            len += 1;
        }
        self.cols = usize::max(self.cols, len);
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row * self.cols + col)
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        self.data[row * self.cols + col] = val;
    }

    pub fn get_col(&self, col: usize) -> impl Iterator<Item = &T> {
        self.data.iter().skip(col).step_by(self.cols)
    }

    pub fn get_row(&self, row: usize) -> impl Iterator<Item = &T> {
        self.data.iter().skip(row * self.cols).take(self.cols)
    }

    pub fn insert_col(&mut self, mut col_index: usize, col: impl Iterator<Item = T>) {
        self.cols += 1;
        for item in col {
            self.data.insert(col_index, item);
            col_index += self.cols;
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows() {
            for item in self.get_row(r) {
                write!(f, "{}", item)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
