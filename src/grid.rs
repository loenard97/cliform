pub struct Grid<T> {
    row: Vec<T>,
    padding: usize,
    _max_len: usize,
}

impl<T: std::fmt::Display> Grid<T> {
    pub fn new(padding: usize, max_len: usize) -> Grid<T> {
        Grid { row: vec![], padding, _max_len: max_len }
    }

    pub fn push(&mut self, item: T) -> &mut Self {
        self.row.push(item);
        
        self
    }

    pub fn from_vec(row: Vec<T>, padding: usize, max_len: usize) -> Grid<T> {
        Grid { row, padding, _max_len: max_len }
    }

    pub fn to_string(self) -> String {
        let max_size = self.row.iter()
            .map(|s| { s.to_string().len() })
            .max().unwrap()
            + self.padding;
        let mut result = String::new();

        for item in self.row {
            result.push_str(&format!("{item: <max_size$}"));
        }
        result = result.trim_end_matches(' ').to_string();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid() {
        let input = vec![
            String::from("Hello"),
            String::from("World"),
        ];
        let grid = Grid::from_vec(input, 2, 100);

        assert_eq!(
            grid.to_string(),
            "Hello  World",
        );
    }

    #[test]
    fn grid_long() {
        let input = vec![
            String::from("a"), 
            String::from("ab"),
            String::from("abc"),
            String::from("abcd"),
            String::from("abcde"),
            String::from("abcdef"),
        ];
        let grid = Grid::from_vec(input, 2, 100);

        assert_eq!(
            grid.to_string(), 
            "a       ab      abc     abcd    abcde   abcdef"
        );
    }
}
