# 加载地图

在上一章中为了测试渲染系统是否正常,我们编写了一些实体.接下来是时候渲染一个合适的地图了.在这一节中我们就先创建一个文本格式的地图配置文件,后面再加载这个配置文件．以此学习怎么创建加载地图.

## 地图配置
首先让我们看一个二维的地图：

```
{{#include ../../../code/rust-sokoban-c02-01/src/main.rs:181:189}}

where:
. is an empty spot
W is a wall
P is the player
B is a box
S is a box spot
N is nothing: used for the outer edges of the map
```

为了简单起见我们直接使用一个字符串常量保存地图信息，就不把它放在一个文件中再读取了：

```rust
{{#include ../../../code/rust-sokoban-c02-01/src/main.rs:179:193}}
```

接下来是编写加载地图(函数load_map)的代码:

```rust
{{#include ../../../code/rust-sokoban-c02-01/src/main.rs:195:234}}
```

这里特别适合使用Rust提供的特别有意思的功能`match`.不过这里我们只是用简单的模式匹配功能分别处理地图配置中的每一个字符，模式匹配(match)还有很多更高级的用法，比如：条件判断，类型匹配等.

> **_MORE:_**  想了解更多模式匹配的功能可以看 [这里](https://doc.rust-lang.org/book/ch06-02-match.html).

现在可以运行下我们的游戏，如果你是跟这我们一起编写的，它看起来应该像这样：

![Screenshot](./images/map.png)

下面是完整代码：

```rust
{{#include ../../../code/rust-sokoban-c02-01/src/main.rs}}
```

> **_CODELINK:_**  可以从[这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c02-01)获取当前实例的完整代码.
