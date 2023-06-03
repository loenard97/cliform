pub struct Grid<T> {
    content: Vec<T>,
}

impl<T: std::fmt::Display> Grid<T> {
    pub fn new() -> Self {
        Grid { content: vec![] }
    }

    pub fn from_vec(row: Vec<T>) -> Grid<T> {
        Grid { content: row }
    }

    pub fn push(&mut self, item: T) {
        self.content.push(item);
    }

    pub fn to_string(self, padding: usize, max_len: usize) -> String {
        let mut result = String::new();
        let mut i = 0;
        let max_size = self.content.iter()
            .map(|s| { s.to_string().len() })
            .max().unwrap_or_default()
            + padding;
        let items_per_row = max_len / max_size;

        for item in self.content {
            if i == items_per_row {
                result.push('\n');
                i = 0;
            }
            result.push_str(&format!("{item: <max_size$}"));
            i += 1;
        }
        result = result.trim_end_matches(' ').to_string();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let grid: Grid<&str> = Grid::new();
        assert_eq!(
            grid.to_string(2, 10),
            ""
        )
    }

    #[test]
    fn small() {
        let input = vec![
            String::from("Hello"),
            String::from("World"),
        ];
        let grid = Grid::from_vec(input);

        assert_eq!(
            grid.to_string(2, 100),
            "Hello  World",
        );
    }

    #[test]
    fn long() {
        let input = vec![
            String::from("a"), 
            String::from("ab"),
            String::from("abc"),
            String::from("abcd"),
            String::from("abcde"),
            String::from("abcdef"),
        ];
        let grid = Grid::from_vec(input);

        assert_eq!(
            grid.to_string(2, 100), 
            "a       ab      abc     abcd    abcde   abcdef"
        );
    }

    #[test]
    fn wrap() {
        let grid = Grid::from_vec(vec![ "hi", "test", "two", "three", "wrap", "this" ]);
        assert_eq!(
            grid.to_string(2, 15),
"hi     test   
two    three  
wrap   this"
        );
    }
}
