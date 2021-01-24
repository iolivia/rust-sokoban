# 模块化

`main.rs`文件已经太大了，随着功能的增加，这样下去还会越来越大，越来越难于维护．怎么办呢?还好Rust支持`模块`，可以把程序按照功能拆分到不同的文件中．

那么现在就让我们开始拆分吧，先看下目录结构：

```
├── resources
│   └── images
│       ├── box.png
│       ├── box_spot.png
│       ├── floor.png
│       ├── player.png
│       └── wall.png
├── src
│   ├── systems
│   │   ├── input_system.rs
│   │   ├── mod.rs
│   │   └── rendering_system.rs
│   ├── components.rs
│   ├── constants.rs
│   ├── entities.rs
│   ├── main.rs
│   ├── map.rs
│   └── resources.rs
└── Cargo.toml
```

> **_MORE:_**  想了解更多关于模块的知识可以点 [这里](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html).

接下来我们就开始把每一个组件放到一个文件中．放到单独的文件中后除了要把属性声明成`public`的，也没什么不一样的．之所以现在需要把属性声明成`public`的是因为先前都在一个文件中，刚开始可以都放在一个文件中，但随着文件越来越大我们就需要把代码拆分到不同的文件中了，为了保证不同的文件（模块）间还能互相访问的到就需要把属性声明成`public`的,这样代码就不会报错了．我们后面也会介绍另外一种拆分方式．另外把注册组件的代码放到文件的下面．拆分好后如果需要修改或者增加组件只需要修改对应的文件就可以了．

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c02-04/src/components.rs:}}
```

下面是资源文件:

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c02-04/src/resources.rs:}}
```

接下来我们把常量也拆分到一个单独文件中．先前地图的维度信息是在代码中硬编码的，最好是根据加载地图的维度动态设置.

```rust
// constants.rs
{{#include ../../../code/rust-sokoban-c02-04/src/constants.rs}}
```

接下来把创建实体的代码放到`entities.rs`文件中:

```rust
// entities.rs
{{#include ../../../code/rust-sokoban-c02-04/src/entities.rs}}
```

下面是地图加载的代码:

```rust
// map.rs
{{#include ../../../code/rust-sokoban-c02-04/src/map.rs}}
```

最后我们再把渲染代码放到`randering_system.rs`文件中，把输入处理代码放到`input_system.rs`文件中，其实也就是复制粘贴改下导入语句.

现在还有个有意思的事，在一个文件夹下包含了多个文件．如果不做些其它操作在`main.rs`文件中要使用`RenderingSystem`或者`InputSystem`程序会报错的．咋办呢?只需在文件夹下添加一个`mod.rs`文件告诉Rust当前这个文件夹下包含那些内容.这样在外部就可以访问RenderingSystem和InputSystem了．


```rust
// systems/mod.rs
{{#include ../../../code/rust-sokoban-c02-04/src/systems/mod.rs}}
```

齐活了！现在再看`main.rs`是不是清爽多了！注意我们用了一些`mod`告诉Rust需要用到的模块.

```rust
// main.rs
{{#include ../../../code/rust-sokoban-c02-04/src/main.rs}}
```

至此拆分模块的任务就完成了，运行下代码应该跟先前的功能是一样的，但代码没那么臃肿了也方便后续添加新功能了．

> **_CODELINK:_**  点 [这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c02-04)获取当前完整代码.


