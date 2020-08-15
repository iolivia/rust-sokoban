# 渲染系统

是时候开始创建第一个系统(`system`)了——渲染系统。这个系统负责把实体绘制到屏幕上，也就是能不能在窗口上看见点东西就看它的了。

## 渲染系统走起
首先我们定义个结构体`RenderingSystem`,它需要使用`ggez`的上下文对象(`context`)绘制实体。

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:47:49}}
```

注意代码中的&apos;a, &apos; 可不是单引号哦,在你的键盘上应该也是`Esc`键下面的那个建。这是什么东东，为何写法如此奇怪嘞?这是Rust的生命周期声明语法。因为Rust编译器自己推断不出结构体`RenderingSystem`持有的`Context`引用的有效性，所以需要我们使用生命周期声明语法告诉它。

> **_MORE:_**  更深入的了解生命周期请点 [这里](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).

接下来我们需要为结构体`RenderingSystem`实现`System`特征。当前只是编写个架子，并不对方法做具体实现。

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:51:57}}
        // implementation here
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:83:84}}
```

代码中定义的`SystemData`类型是方便访问位置和可渲染存储信息的。我们使用了只读存储`ReadStorage`，也就是只读取数据不修改数据。

最后在绘制循环中运行渲染系统。也就是当每次游戏更新时也同时根据实体的最新状态重新绘制实体。

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:97:111}}
```

现在我们的代码是可以编译运行的，但是依然看不到任何东西，因为我们还没编写渲染的逻辑代码，也还没创建实体。

## 实现渲染系统

实现渲染系统需要做这些事：

* 清空屏幕(确保不显示过去的`帧`)
* 获取所有具备可渲染组件的实体，并按空间z轴排列好后渲染。这样可以保证实体可以一层一层累加渲染，比如玩家应该在地板上面，不然我们就看不到他了。
* 按排列好的顺序一个一个的把实体渲染为图片展示。
* 最后就可以在屏幕上看到它们了。

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:56:83}}
```

## 添加实体测试下

接下来我们创建一些用来测试的实体，验证下我们的代码是不是可以正常工作。

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:179:204}}
```

最后我们把所有这些都整合到一起，然后编译运行，你会看到：

![Screenshot](../images/rendering.png)

是不是小激动？这是我们第一次实现了个渲染系统在窗口上绘制出了点东西。小激动一下就可以了，毕竟现在还只是显示了些静态的图片还不能称之为游戏，后面我们会让它更像个游戏。

最终的代码是这个样子的：

> **_注意:_**  当前实现的渲染系统还比较简单，随着实体的增多可能会有性能问题。在第三章的[批量渲染](/c03-04-batch-rendering.html)章节我们还会做些优化,敬请期待!


```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs}}
```

> **_CODELINK:_**  可以点 [这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c01-04)获取本章节完整代码.

