use std::fmt;

#[derive(Debug, Clone)]
struct TreeItem {
    value: String,
    depth: usize,
    is_last: bool,
}

pub struct Tree {
    row: Vec<TreeItem>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree { row: vec![] }
    }

    pub fn push(&mut self, value: &str, depth: usize, is_last: bool) -> &mut Self {
        self.row.push(
            TreeItem { value: value.to_string(), depth, is_last }
        );
        self
    }

    pub fn to_my_string(self) -> String {
        let mut result = String::new();

        for item in self.row {
            result.push_str(&"│ ".repeat(item.depth));
            if item.is_last {
                result.push_str("└");
            } else {
                result.push_str("├");
            }
            result.push_str(&format!("─ {}\n", item.value));
        }
        result = result.trim_end_matches('\n').to_string();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short() {
        let mut tree = Tree::new();
        tree.push("first", 0, false)
            .push("first", 0, false)
            .push("second", 0, false)
            .push("2-1", 1, false)
            .push("2-2", 1, false)
            .push("2-3", 1, true)
            .push("third", 0, false);

        assert_eq!(
            tree.to_my_string(),
"├─ first
├─ first
├─ second
│ ├─ 2-1
│ ├─ 2-2
│ └─ 2-3
├─ third"
        );
    }
}
