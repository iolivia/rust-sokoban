# 批量渲染

你可能已经注意到在玩游戏时输入感觉有点慢。让我们添加一个 FPS 计数器来看看我们的渲染速度如何。如果你不熟悉 FPS 这个术语，它代表每秒帧数（Frames Per Second），我们的目标是达到 60FPS。

## FPS 计数器

让我们从添加 FPS 计数器开始，这包含两个部分：

1. 获取或计算 FPS 值
2. 在屏幕上渲染这个值

对于第1点，幸运的是 ggez 提供了获取 fps 的方法 - 参见[这里](https://docs.rs/ggez/latest/ggez/timer/struct.TimeContext.html#method.fps)。对于第2点，我们在渲染系统中已经有了渲染文本的方法，所以我们只需要在那里获取 FPS。让我们在代码中把这些都组合起来。

```rust
// rendering.rs
{{#include ../../../code/rust-sokoban-c03-05/src/systems/rendering.rs:run_rendering}}

    /// 代码省略
    /// .....
    /// .....
    
{{#include ../../../code/rust-sokoban-c03-05/src/systems/rendering.rs:render_fps}}

    /// 代码省略
    /// .....
    /// .....

{{#include ../../../code/rust-sokoban-c03-05/src/systems/rendering.rs:run_rendering_end}}
```

运行游戏并用按键移动一下，你会看到 FPS 从预期的 60 显著下降。对我来说，它看起来在 20-30 范围内，但根据你的机器可能会更多或更少。

![低fps](./images/low_fps.png)

## 是什么导致了 FPS 下降？

现在你可能会问自己，我们做了什么导致 FPS 这么低？我们有一个相当简单的游戏，我们的输入和移动逻辑实际上并不复杂，我们也没有太多的实体或组件来导致如此大的 FPS 下降。要理解这一点，我们需要更深入地了解我们当前的渲染系统是如何工作的。

目前，对于每个可渲染的实体，我们都要确定要渲染哪个图像并渲染它。这意味着如果我们有 20 个地板贴图，我们将加载地板图像 20 次并发出 20 个单独的渲染调用。这太昂贵了，这就是导致我们 FPS 大幅下降的原因。

我们如何解决这个问题？我们可以使用一种叫做批量渲染的技术。使用这种技术，我们要做的就是只加载一次图像，并告诉 ggez 在所有需要渲染的 20 个位置渲染它。这样我们不仅只加载一次图像，而且每个图像只调用一次渲染，这将大大提高速度。作为旁注，一些引擎会在底层为你完成这种渲染批处理，但 ggez 不会，这就是为什么我们需要关注这一点。

## 批量渲染实现

以下是我们实现批量渲染需要做的事情：

* 对于每个可渲染实体，确定我们需要渲染的图像和 DrawParams（这是我们目前给 ggez 的渲染位置指示）
* 将所有（图像，DrawParams）保存为一个方便的格式
* 按照 z 轴排序遍历（图像，DrawParams），每个图像只进行一次渲染调用

在深入渲染代码之前，我们需要做一些集合分组和排序，我们将使用 itertools crate 来完成这个任务。我们可以自己实现这个分组，但没有必要重新发明轮子。让我们将 itertools 作为依赖项添加到我们的项目中。

```toml
// Cargo.toml
{{#include ../../../code/rust-sokoban-c03-05/Cargo.toml:9:13}}
```

我们也在渲染系统中导入它

```rust
// rendering.rs
{{#include ../../../code/rust-sokoban-c03-05/src/systems/rendering.rs:use_itertools}}
```

还记得我们在动画章节中编写的用来确定每一帧需要哪个图像的 get_image 函数吗？我们可以重用它，只需要确保我们不实际加载图像，而是返回图像的路径。

```rust
// rendering.rs
{{#include ../../../code/rust-sokoban-c03-05/src/systems/rendering.rs:get_image}}
```

现在让我们确定我们想要的批处理数据的格式。我们将使用 `HashMap<u8, HashMap<String, Vec<DrawParam>>>`，其中：

* 第一个键（`u8`）是 z 位置 - 记住我们需要遵守 z 位置并从最高到最小的 z 绘制以确保正确的顺序（例如地板应该在玩家下面等）
* 值是另一个 `HashMap`，其中第二个键（`String`）是图像的路径
* 最后，最后的值是一个 `Vec<DrawParam>`，它包含了我们必须渲染该特定图像的所有参数

让我们现在编写代码来填充 rendering_batches 哈希映射。

```rust
// rendering.rs
{{#include ../../../code/rust-sokoban-c03-05/src/systems/rendering.rs:rendering_batches}}
```

最后，让我们实际渲染这些批次。我们不能使用之前使用的 draw(image) 函数，但幸运的是 ggez 有一个批处理 API - [SpriteBatch](https://docs.rs/ggez/0.7.0/ggez/graphics/spritebatch/struct.SpriteBatch.html)。另外注意这里的 `sorted_by`，这是 itertools 提供给我们的。

```rust
// rendering.rs
{{#include ../../../code/rust-sokoban-c03-05/src/systems/rendering.rs:rendering_batches_2}}
```

就是这样！再次运行游戏，你应该看到闪亮的 60FPS，一切都应该感觉更流畅！

![高fps](./images/high_fps.png)

> **_CODELINK:_** 你可以在[这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-05)看到这个示例的完整代码。
