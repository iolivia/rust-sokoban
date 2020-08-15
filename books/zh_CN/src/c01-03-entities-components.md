# 组件和实体
嗨，少年！看你骨骼惊奇接下来就开始一起学习怎么结合`specs`创建、注册组件和实体。

## 定义组件
我们先从定义组件开始。先前我们提到过`位置组件`、`可渲染组件`和`动作组件`(这个后面再讲哈)。我们需要用一些组件标识实体，比如可以让一个实体包含墙组件标识它是墙。

可以直接简单的说：位置组件其实就是用来存储地图坐标的x、y、z值的可以用来定位；渲染组件就是使用字符串存储一个需要绘制的图片的路径；另外一些组件基本都是 [标记型组件](https://specs.amethyst.rs/docs/tutorials/11_advanced_component.html?highlight=marker#marker-components)并不存储数据。


```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs:13:42}}
```

`#[storage(VecStorage)]`这原来没见过是不是? 恭喜你，少年！你已经使用到了一个Rust很强大的功能`过程宏`。这种宏是一些可以在代码`编译时`对代码进行处理并生成新代码的特殊函数。

> **_MORE:_**  如果你想更深入的了解宏，可以看 [这里](https://doc.rust-lang.org/book/ch19-06-macros.html).

## 注册组件
在`specs`中使用组件前需要先注册组件，就像这样：

> 把组件注册到world

```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs:61:69}}
```

## 创建实体
实体就是代表一系列组件，所以我们创建实体的方法就是简单地指定它们包含哪些组件。就像这个样子：

```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs:71:124}}
```

## 素材

睿智如你应该已经注意到了，我们还引用了些用于创建实体的素材，就是图片什么的。当然你要是觉得我们准备的素材不好看，也可以使用自己的素材。我们准备的素材就在下面了，你可以右键另存为下载到电脑上:

![地板](../images/floor.png)
![墙](../images/wall.png)
![玩家](../images/player.png)
![箱子](../images/box.png)
![方形斑点](../images/box_spot.png)

接下来把这些图片放到我们的项目中。在项目目录中新建`resources`目录，用于存放项目需要用到的资源，目前我们只有图片资源需要存储，以后还会有配置文件啊，音频文件（[第三章的第3小节会用到](/c03-03-sounds-events.html))啊什么的。为了区分不同的资源文件，在`resources`目录下再新建一个`images`目录，用于存放我们的png图片。你也可以按照自己的喜欢命名目录，除了只要你开心就好，还要记得在代码中引用这些资源时要写出正确的路径。一波操作下来后，我们项目的目录结构大概是这个样子地：

```
├── resources
│   └── images
│       ├── box.png
│       ├── box_spot.png
│       ├── floor.png
│       ├── player.png
│       └── wall.png
├── src
│   └── main.rs
└── Cargo.toml
```

## 创建游戏世界（World）
最后，当然只是本小节的最后，接下来在main函数的第一行就创建一个`specs::World`对象，把先前创建的实体还有素材都整合到一起。

```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs}}
```

然后你就可以执行`cargo run`运行看下效果，当你满怀期待却发现看到的依然是一个空白的窗口，控制台里可能还多了些警告信息。这是因为我们还没有编写渲染的代码也就是还没有绘制这些实体。少侠，莫急！下一节，我们就开始绘制。到时这些因为引入而没有使用的警告也就自然消失了。

> **_CODELINK:_**  你可以在 [这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c01-03)找到本小节完整的代码.
