
# 模块

主文件已经变得相当大了，可以想象，随着我们的项目增长，这种方式将无法维持下去。幸运的是，Rust 提供了模块的概念，可以让我们根据关注点将功能整齐地拆分到单独的文件中。

目前，我们的目标是实现以下文件夹结构。随着我们添加更多的组件和系统，我们可能需要不止一个文件，但这是一个不错的起点。

```sh
├── resources
│   └── images
│       ├── box.png
│       ├── box_spot.png
│       ├── floor.png
│       ├── player.png
│       └── wall.png
├── src
│   ├── systems
│   │   ├── input.rs
│   │   ├── rendering.rs
│   │   └── mod.rs
│   ├── components.rs
│   ├── constants.rs
│   ├── entities.rs
│   ├── main.rs
│   ├── map.rs
│   └── resources.rs
└── Cargo.toml
```

> **_更多：_**  阅读更多关于模块和管理增长项目的信息 [这里](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)。

首先，让我们将所有组件移到一个文件中。除了将某些字段设置为公共的之外，不会有任何更改。需要将字段设置为公共的原因是，当所有内容都在同一个文件中时，所有内容可以相互访问，这在开始时很方便，但现在我们将内容分开，我们需要更加注意可见性。目前我们将字段设置为公共以使其正常工作，但稍后我们会讨论一种更好的方法。我们还将组件注册移动到了该文件的底部，这样当我们添加组件时，只需要更改这个文件就可以了。

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c02-04/src/components.rs:}}
```

接下来，让我们将常量移到它们自己的文件中。目前我们硬编码了地图的尺寸，这在移动时需要知道我们何时到达地图的边缘，但作为改进，我们可以稍后存储地图的尺寸，并根据地图加载动态设置它们。

```rust
// constants.rs
{{#include ../../../code/rust-sokoban-c02-04/src/constants.rs}}
```

接下来，实体创建代码现在移到了一个实体文件中。

```rust
// entities.rs
{{#include ../../../code/rust-sokoban-c02-04/src/entities.rs}}
```

现在是地图加载。

```rust
// map.rs
{{#include ../../../code/rust-sokoban-c02-04/src/map.rs}}
```

最后，我们将系统代码移到它们自己的文件中（`RenderingSystem` 到 `rendering.rs`，`InputSystem` 到 `input.rs`）。这应该只是从主文件中复制粘贴并移除一些导入，因此可以直接进行。

我们需要更新 `mod.rs`，告诉 Rust 我们想将所有系统导出到外部（在这里是主模块）。

```rust
// systems/mod.rs
{{#include ../../../code/rust-sokoban-c02-04/src/systems/mod.rs}}
```

太棒了，现在我们完成了这些操作，以下是简化后的主文件的样子。注意导入后的 `mod` 和 `use` 声明，它们再次告诉 Rust 我们想要使用这些模块。

```rust
// main.rs
{{#include ../../../code/rust-sokoban-c02-04/src/main.rs}}
```

此时可以运行，所有功能应该与之前完全相同，不同的是，现在我们的代码更加整洁，为更多令人惊叹的 Sokoban 功能做好了准备。

> **_代码链接：_**  你可以在 [这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c02-04) 查看本示例的完整代码。
