#[cfg(test)]
mod tests {
    use cliform::*;

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
}
