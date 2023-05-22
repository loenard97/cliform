#![allow(dead_code)]

pub enum TableHeaderStyle {
    LeftAlign,
    CenterAlign,
    RightAlign,
}

#[derive(PartialEq)]
pub enum TableLineStyle {
    NoLines,
    Lines,
}

pub struct TableStyle {
    header: TableHeaderStyle,
    lines: TableLineStyle,
}

pub struct Table<T> {
    header: Vec<T>,
    content: Vec<Vec<T>>,
    max_cols: usize,
    max_col_size: usize,
    style: TableStyle,
}

impl<T: std::fmt::Display> Table<T> {
    pub fn new() -> Table<T> {
        let header = vec![];
        let content = vec![vec![]];
        let n_cols = header.len();
        let max_size = 0;
        let style = TableStyle { 
            header: TableHeaderStyle::LeftAlign, 
            lines: TableLineStyle::Lines
        };

        return Table { header, content, max_cols: n_cols, max_col_size: max_size, style }
    }

    pub fn header(&mut self, header: Vec<T>) {
        self.max_cols = usize::max(self.max_cols, header.len());
        self.header = header;
    }

    pub fn style(&mut self, style: TableStyle) {
        self.style = style;
    }

    pub fn push(&mut self, row: Vec<T>) {
        self.max_cols = usize::max(self.max_cols, row.len());
        let max_size = row.iter()
            .map(|item| { item.to_string().len() })
            .max()
            .unwrap();
        self.max_col_size = usize::max(self.max_col_size, max_size);
        self.content.push(row);
    }

    pub fn to_string(&self, padding: usize) -> String {
        let col_width = self.max_col_size + padding;
        let mut result = String::new();

        for item in &self.header {
            result.push_str(&format!("{item: <col_width$}"));
        }
        result = result.trim_end_matches(' ').to_string();

        result.push('\n');

        if self.style.lines == TableLineStyle::Lines {
            result.push_str(&"─".repeat(self.max_cols * (self.max_col_size + padding) - padding));
        }

        for row in &self.content {
            for item in row {
                result.push_str(&format!("{item: <col_width$}"));
            }
            result = result.trim_end_matches(' ').to_string();
            result.push('\n');
        }
        result = result.trim_end_matches('\n').to_string();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table() {
        let mut table = Table::new();
        table.header(vec!["first", "second", "third"]);
        table.push(vec!["Hello", "World", "!"]);
        table.push(vec!["How", "are", "you?"]);
        table.push(vec!["Great", "weather", "right?"]);

        assert_eq!(
            table.to_string(2),
"first    second   third
─────────────────────────
Hello    World    !
How      are      you?
Great    weather  right?"
        )
    }
}
