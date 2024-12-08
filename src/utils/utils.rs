pub struct Grid<T> {
    storage: Vec<Vec<T>>,
}

impl <T> Grid<T> {

    pub fn new(width: usize, height: usize, value: T) -> Self where T : Clone {
        let mut grid: Vec<Vec<T>> = vec![];
        for y in 0..height {
            grid.push(vec![]);
            for _ in 0..width {
                grid[y].push(value.clone())
            }
        }
        Grid::from(grid)
    }

    pub fn get_storage(&self) -> &Vec<Vec<T>> {
        &self.storage
    }

    pub fn width(&self) -> usize {
        self.storage[0].len()
    }

    pub fn height(&self) -> usize {
        self.storage.len()
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        let width = self.width() as i32;
        let height = self.height() as i32;
        x >= 0 && x < width && y >= 0 && y < height
    }

    pub fn try_get(&mut self, x: i32, y: i32) -> Option<&T> {
        if self.contains(x, y) {
            Some(self.get(x as usize, y as usize))
        } else {
            None
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &(&self.storage[y])[x]
    }

    pub fn try_set(&mut self, x: i32, y: i32, value: T) -> bool {
        if self.contains(x, y) {
            self.set(x as usize, y as usize, value);
            true
        } else {
            false
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        (&mut self.storage[y])[x] = value
    }

    pub fn count(&self, predicate: impl Fn(&T) -> bool) -> i32 {
        let mut count = 0;
        for row in &self.storage {
            for value in row {
                if predicate(value) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn get_occurrences(&self, predicate: impl Fn(&T) -> bool) -> Vec<(usize, usize)> {
        let mut output: Vec<(usize, usize)>  = vec![];

        for (y, row) in self.storage.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if predicate(value) {
                    output.push((x, y));
                }
            }
        }

        output
    }
}

impl <T : Clone> From<Vec<Vec<T>>> for Grid<T> {

    fn from(value: Vec<Vec<T>>) -> Self {
        Grid {
            storage: value.clone()
        }
    }
}

impl From<String> for Grid<char> {

    fn from(value: String) -> Self {
        let grid: Vec<Vec<char>> = value.lines().map(|it| it.chars().collect()).collect();
        Grid::from(grid)
    }
}