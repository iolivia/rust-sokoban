
# 渲染系统

现在是时候实现我们的第一个系统——渲染系统了。这个系统将负责在屏幕上绘制所有的实体。

## 渲染系统设置

首先，我们从一个空的实现开始，如下所示：

```rust
pub fn run_rendering(world: &World, context: &mut Context) {
        // TODO 添加实现
}
```

最后，让我们在绘制循环中运行渲染系统。这意味着每次游戏更新时，我们都会渲染所有实体的最新状态。

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:handler}}
```

现在运行游戏应该可以编译，但可能还不会有任何效果，因为我们尚未填充渲染系统的实现，也没有创建任何实体。

## 渲染系统实现

**注意：** 我们将在这里添加 [glam](https://lib.rs/crates/glam) 作为依赖项，这是一个简单快速的 3D 库，可以提供一些性能改进。

```
{{#include ../../../code/rust-sokoban-c01-03/Cargo.toml:9:11}}
```

以下是渲染系统的实现。它完成以下几个任务：

* 清除屏幕（确保我们不会保留前一帧渲染的状态）
* 获取所有具有可渲染组件的实体并按 z 轴排序（这样我们可以确保正确的叠加顺序，例如玩家应该在地板之上，否则我们看不到玩家）
* 遍历排序后的实体并将它们作为图像渲染
* 最后，呈现到屏幕上

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:rendering_system}}
```

## 添加一些测试实体

让我们创建一些测试实体以确保工作正常。

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:init}}
```

最后，让我们将所有内容组合在一起并运行。你应该会看到类似这样的效果！这非常令人兴奋，现在我们有了一个正式的渲染系统，我们终于可以在屏幕上看到一些东西了。接下来，我们将开始处理游戏玩法，使其真正像一个游戏！

![截图](./images/rendering.png)

以下是最终代码。

> **_注意：_**  请注意，这是渲染的一个非常基本的实现，随着实体数量的增加，性能可能不足够好。一个更高级的渲染实现使用批量渲染，可以在[第 3 章 - 批量渲染](/c03-04-batch-rendering.html)中找到。

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs}}
```

> **_代码链接：_**  你可以在 [这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c01-04) 查看本示例的完整代码。
