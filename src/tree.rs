pub enum TreeStyle {
    Lines,
    Spaces,
}

#[derive(Debug, Clone)]
struct TreeItem<T> {
    value: T,
    depth: usize,
    is_last: bool,
}

pub struct Tree<T> {
    row: Vec<TreeItem<T>>,
    style: TreeStyle,
}

impl<T: std::fmt::Display> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { row: vec![], style: TreeStyle::Lines }
    }

    pub fn push(&mut self, value: T, depth: usize, is_last: bool) {
        self.row.push(
            TreeItem { value, depth, is_last }
        );
    }

    pub fn style(&mut self, style: TreeStyle) {
        self.style = style;
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for item in &self.row {

            match self.style {
                TreeStyle::Lines => {
                    result.push_str(&"│ ".repeat(item.depth));
                    if item.is_last {
                        result.push_str("└");
                    } else {
                        result.push_str("├");
                    }
                    result.push_str(&format!("─ {}\n", item.value));
                },
                TreeStyle::Spaces => {
                    result.push_str(&" ".repeat(4 * item.depth));
                    result.push_str(&format!("{}\n", item.value));
                },
            }

            
        }
        result = result.trim_end_matches('\n').to_string();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let tree: Tree<&str> = Tree::new();
        assert_eq!(
            tree.to_string(),
            ""
        )
    }

    #[test]
    fn lines() {
        let mut tree = Tree::new();
        tree.push("first", 0, false);
        tree.push("first", 0, false);
        tree.push("second", 0, false);
        tree.push("2-1", 1, false);
        tree.push("2-2", 1, false);
        tree.push("2-3", 1, true);
        tree.push("third", 0, false);

        assert_eq!(
            tree.to_string(),
"├─ first
├─ first
├─ second
│ ├─ 2-1
│ ├─ 2-2
│ └─ 2-3
├─ third"
        );
    }

    #[test]
    fn spaces() {
        let mut tree = Tree::new();
        tree.style(TreeStyle::Spaces);
        tree.push("first", 0, false);
        tree.push("first", 0, false);
        tree.push("second", 0, false);
        tree.push("2-1", 1, false);
        tree.push("2-2", 1, false);
        tree.push("2-3", 1, true);
        tree.push("third", 0, false);

        assert_eq!(
            tree.to_string(),
"first
first
second
    2-1
    2-2
    2-3
third"
        );
    }
}
