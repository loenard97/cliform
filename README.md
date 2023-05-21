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
grid.push("Hello")
    .push("World")
    .push("!")

println!("{}", grid.to_string());
```

```sh
Hello  World  !
```

### Table

### Tree
```rust
let tree = Tree::new();
tree.push("first", 0)
    .push("second", 1)
    .push("third", 1)

println!("{}", tree.to_string());
```

```sh
├─ first
│  ├─ second
│  └─ third
```
