
# 地图加载

上一章我们创建了一些实体来测试我们的渲染系统，但现在是时候渲染一个正式的地图了。在本节中，我们将创建一个基于文本的地图配置并加载它。

## 地图配置

第一步，让我们尝试基于如下所示的二维地图加载一个关卡。

```rust
{{#include ../../../code/rust-sokoban-c02-01/src/main.rs:map}}

其中：
. 是空白位置
W 是墙
P 是玩家
B 是箱子
S 是箱子放置点
N 是空：用于地图的外边缘
```

最终我们可以从文件中加载，但为了简单起见，现在我们先用代码中的常量。

以下是加载地图的实现。

```rust
{{#include ../../../code/rust-sokoban-c02-01/src/main.rs:init}}
```

这里最有趣的 Rust 概念可能是 `match`。我们在这里使用了模式匹配的基本功能，仅仅是匹配地图配置中每个标记的值，但我们可以进行更高级的条件或类型模式匹配。

> **_更多：_**  阅读更多关于模式匹配的信息 [这里](https://doc.rust-lang.org/book/ch06-02-match.html)。

现在运行游戏，看看我们的地图是什么样子。

![截图](./images/map.png)

以下是最终代码。

```rust
{{#include ../../../code/rust-sokoban-c02-01/src/main.rs}}
```

> **_代码链接：_**  你可以在 [这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c02-01) 查看本示例的完整代码。
