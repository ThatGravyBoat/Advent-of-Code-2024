pub struct Grid<T> {
    storage: Vec<Vec<T>>,
}

impl <T> Grid<T> {

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

    pub fn get(&self, x: usize, y: usize) -> &T {
        &(&self.storage[y])[x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        (&mut self.storage[y])[x] = value
    }

    pub fn count(&self, predicate: fn(&T) -> bool) -> i32 {
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
}

impl <T : Clone> From<Vec<Vec<T>>> for Grid<T> {

    fn from(value: Vec<Vec<T>>) -> Self {
        Grid {
            storage: value.clone()
        }
    }
}