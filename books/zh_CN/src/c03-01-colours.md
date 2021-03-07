# 不同颜色的盒子
现在我们的游戏有些单调，玩法也比较简单，只是把盒子推到目标点上就可以了．接下来我们可以把盒子和点分成不同的颜色，比如分成红色和蓝色，你可以按照自己的意愿使用不同的颜色，玩家只有把盒子推到一样颜色的目标点上才算赢得了游戏.

## 素材
首先我们需要添加些新的素材，你可以右键下载这些图片，也可以自己创建一些图片．

![Blue box](./images/box_blue.png)
![Red box](./images/box_red.png)
![Blue box spot](./images/box_spot_blue.png)
![Red box spot](./images/box_spot_red.png)

现在项目的目录结构看起来像这样(需要注意的是我们已经移除了原来的盒子和目标点):

```
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
│   │   ├── gameplay_state_system.rs
│   │   ├── input_system.rs
│   │   ├── mod.rs
│   │   └── rendering_system.rs
│   ├── components.rs
│   ├── constants.rs
│   ├── entities.rs
│   ├── main.rs
│   ├── map.rs
│   └── resources.rs
├── Cargo.lock
├── Cargo.toml
```

## 变更组件
接下来我们新增一个用来表示颜色的枚举类型(我们这里只给出了两种颜色，如果你想使用更多的颜色可以在这里添加):

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-01/src/components.rs:29:32}}
```

现在我们就可以在盒子和目标点上应用颜色了:

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-01/src/components.rs:44:54}}
```

## 创建实体
在创建盒子和目标点时还需要把颜色做为参数，并根据具体颜色加载相应的图片素材.

我们可以把素材的加载路径设置为 `"/images/box_{}.png"` ,其中 `{}` 根据颜色取相应的值．这里我们就需要把枚举类型的颜色值转换为字符串.比如把  `BoxColour::Red` 转换为`"red"`. 如果能调用 `colour.to_string()` 转换就好了． 幸运的是Rust提供了 `Display` 特征，我们只需要为 `BoxColour` 枚举类型实现这个特征就可以把它转换为相应的字符串了:

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-01/src/components.rs:34:43}}
```

接下来就可以在创建实体时使用颜色了:

```rust
// entities.rs
{{#include ../../../code/rust-sokoban-c03-01/src/entities.rs:27:48}}
```

## 地图
现在我们还需要修改下地图映射代码，以支持有颜色的盒子和目标点:

* "BB" 表示蓝色的盒子
* "RB" 表示红色的盒子
* "BS" 表示蓝色的目标点
* "RS" 表示红色的目标点

```rust
// map.rs
{{#include ../../../code/rust-sokoban-c03-01/src/map.rs}}
```

接下来还需要在`main.rs`文件中修改我们的静态地图:

```rust
// main.rs
{{#include ../../../code/rust-sokoban-c03-01/src/main.rs:65:80}}
```

## 试玩
终于完成了，现在可以运行代码试玩下了．你会发现代码虽然可以正常运行，但是当我们把红色的盒子推到蓝色的目标点上时也会提示获胜了!这跟我们先要的玩法不一样啊,那咋办呢?

我们可以在判断是否获胜的代码里加上判断盒子和目标点的颜色是否一致的代码：

```rust
// gameplay_state_system.rs
{{#include ../../../code/rust-sokoban-c03-01/src/systems/gameplay_state_system.rs:20:52}}
```

现在编译代码，编译器会报错，因为Rust不知道怎么对俩个枚举值进行 `==` 操作.怎么告诉Rust怎么处理呢?可以为颜色枚举类型实现 `PartialEq` 特征.

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-01/src/components.rs:28:32}}
```

这里我们使用了扩展 `derive` 注解. 先前我们也用过这个注解只是没做过多介绍．扩展`Deribe`注解可以被用在结构体和枚举类型上，帮他们快速实现一些特征．比如我们这里使用的`#[derive(PartialEq)]`就是为枚举类型`BoxColour` 快速实现了`PartialEq`特征.

快速实现的 `PartialEq` 是什么样的呢?看起来就像下面这样：

```rust
pub trait PartialEq {
  fn eq(&self, other: &Self) -> bool;
  fn ne(&self, other: &Self) -> bool { !self.eq(other) };
}
```

虽然只是简单的判断了两个对象是否相等，如果相等就返回true，如果不相等就返回false，但对我们当前的应用场景已经够用了.

也就是说由于我们通过给枚举类型`BoxColor`添加`#[derive(PartialEq)]`注解实现了`PartialEq`特征，当我们在使用`==`比较俩个颜色时，Rust就知道怎么处理了．它会判断这俩个颜色值是不是一样的．如果一样就返回true,如果不一样就返回false.

> **_MORE:_**  了解更多关于 PartialEq 知识请点[这里](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) 了解更多关于Derive知识请点 [这里](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html).

现在再编译运行代码，就可以基于颜色判断是否获胜了.

![Sokoban play](./images/colours.gif)

> **_CODELINK:_**  可以点 [这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-01)获取当前完整代码.