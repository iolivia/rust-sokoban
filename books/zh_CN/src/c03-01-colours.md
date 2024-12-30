# 彩色方块

是时候为我们的游戏增添一些色彩了！到目前为止，游戏玩法相当简单，就是把方块放到指定位置。让我们通过添加不同颜色的方块来让游戏更有趣！现在我们将使用红色和蓝色方块，但你可以根据自己的喜好进行调整，创建更多颜色！现在要赢得游戏，你必须把方块放在相同颜色的目标点上。

## 资源

首先让我们添加新的资源，右键下载这些图片，或者创建你自己的图片！

![蓝色方块](./images/box_blue.png)
![红色方块](./images/box_red.png)
![蓝色目标点](./images/box_spot_blue.png)
![红色目标点](./images/box_spot_red.png)

目录结构应该是这样的（注意我们已经移除了默认的方块和目标点）：

```sh
├── resources
│   └── images
│       ├── box_blue.png
│       ├── box_red.png
│       ├── box_spot_blue.png
│       ├── box_spot_red.png
│       ├── floor.png
│       ├── player.png
│       └── wall.png
├── src
│   ├── systems
│   │   ├── gameplay.rs
│   │   ├── input.rs
│   │   ├── mod.rs
│   │   └── rendering.rs
│   ├── components.rs
│   ├── constants.rs
│   ├── entities.rs
│   ├── main.rs
│   ├── map.rs
│   └── resources.rs
├── Cargo.lock
├── Cargo.toml
```

## 组件更改

现在让我们为颜色添加一个枚举（如果你选择实现两种以上的颜色，你需要在这里添加它们）。

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-01/src/components.rs:box_colour}}
```

现在让我们在方块和目标点中使用这个枚举。

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-01/src/components.rs:box}}
```

## 实体创建

让我们在创建方块和目标点时添加颜色参数，并确保根据颜色枚举传递正确的资源路径。

为了根据颜色创建正确的资源路径字符串，我们基本上想要 `"/images/box_{}.png"`，其中 `{}` 是我们要创建的方块的颜色。现在我们面临的挑战是我们使用的是颜色枚举，所以 Rust 编译器不知道如何将 `BoxColour::Red` 转换为 `"red"`。如果能够使用 `colour.to_string()` 并获得正确的值就太好了。幸运的是，Rust 为我们提供了一个很好的方法，我们需要在 `BoxColour` 枚举上实现 `Display` 特征。下面是具体实现，我们只需指定如何将枚举的每个变体映射到字符串。

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-01/src/components.rs:box_colour_display}}
```

现在让我们在实体创建代码中包含颜色，并使用我们刚刚实现的 `colour.to_string()` 功能。

```rust
// entities.rs
{{#include ../../../code/rust-sokoban-c03-01/src/entities.rs:create_box}}
```

## 地图

现在让我们修改地图代码以允许新的彩色方块和目标点选项：

* "BB" 表示蓝色方块
* "RB" 表示红色方块
* "BS" 表示蓝色目标点
* "RS" 表示红色目标点

```rust
// map.rs
{{#include ../../../code/rust-sokoban-c03-01/src/map.rs:map_match}}
```

让我们在初始化关卡时更新我们的静态地图。

```rust
// map.rs
{{#include ../../../code/rust-sokoban-c03-01/src/map.rs:initialize_level}}
```

## 游戏玩法

现在我们已经完成了艰难的工作，可以继续测试这段代码了。你会注意到一切都能工作，但是存在一个重大的游戏玩法错误。你可以通过把红色方块放在蓝色目标点上或反之来赢得游戏。让我们来修复这个问题。

我们之前学过，根据 ECS 方法论，数据放在组件中，行为放在系统中。我们现在讨论的是行为，所以它必须在系统中。还记得我们如何添加检查是否获胜的系统吗？这正是我们需要修改的地方。

让我们修改运行函数，检查目标点和方块的颜色是否匹配。

```rust
// gameplay.rs
{{#include ../../../code/rust-sokoban-c03-01/src/systems/gameplay.rs}}
```

如果你现在编译代码，它会抱怨我们试图用 `==` 比较两个枚举。Rust 默认不知道如何处理这个问题，所以我们必须告诉它。我们能做的最好的方法是为 `PartialEq` 特征添加一个实现。

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-01/src/components.rs:box_colour_eq}}
```

现在是讨论这些不寻常的 `derive` 注解的好时机。我们以前使用过它们，但从未深入探讨它们的作用。派生属性可以应用于结构体或枚举，它们允许我们为我们的类型添加默认的特征实现。例如，这里我们告诉 Rust 为我们的 `BoxColour` 枚举添加 `PartialEq` 默认特征实现。

这是 `PartialEq` 默认实现的样子，它只检查某个东西是否等于它自己。如果相等，比较成功；如果不相等，比较失败。如果这不太容易理解，也不用太担心。

```rust
pub trait PartialEq {
  fn eq(&self, other: &Self) -> bool;
  fn ne(&self, other: &Self) -> bool { !self.eq(other) };
}
```

所以通过在枚举上方添加 `#[derive(PartialEq)]`，我们告诉 Rust `BoxColour` 现在实现了我们之前看到的偏等特征，这意味着如果我们尝试进行 `box_colour_1 == box_colour_2`，它将使用这个实现，只检查 colour_1 对象是否与 colour_2 对象相同。这不是最复杂的偏等实现，但对我们的用例来说应该足够了。

> **_更多：_** 在[这里](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)阅读更多关于 PartialEq 的信息，在[这里](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)阅读更多关于可派生特征的信息。

现在我们可以编译代码并通过看到游戏运行来收获我们努力的成果，只有当我们把正确的方块放在正确的位置时，游戏才会告诉我们赢了！

![索科班游戏](./images/colours.gif)

> **_代码链接：_** 你可以在[这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-01)看到这个示例的完整代码。
