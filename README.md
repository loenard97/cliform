<div align="center">

# cliform
A rust library to format the output of cli programs

![](https://img.shields.io/github/last-commit/loenard97/cliform?&style=for-the-badge&color=F74C00)
![](https://img.shields.io/github/repo-size/loenard97/cliform?&style=for-the-badge&color=F74C00)

</div>


## ▶️ Usage
### Grid
```rust
let grid = Grid::new();
grid.push("Hello");
grid.push("World");
grid.push("!");

println!("{}", grid.to_string());
```

```sh
Hello  World  !
```

### Table
```rust
let mut table = Table::new();
table.header(vec!["first", "second", "third"]);
table.push(vec!["Hello", "World", "!"]);
table.push(vec!["How", "are", "you?"]):
table.push(vec!["Great", "weather", "right?"]);

println!("{}", table.to_string());
```

```sh
first    second   third
───────────────────────────
Hello    World    !
How      are      you?
Great    weather  right?
```

### Tree
```rust
let tree = Tree::new();
tree.push("first", 0);
tree.push("second", 1);
tree.push("third", 1);

println!("{}", tree.to_string());
```

```sh
├─ first
│  ├─ second
│  └─ third
```
