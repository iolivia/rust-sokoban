# 让角色动起来

严格来说当前我们编写的还称不上游戏，因为还不能让玩家操作角色动起来．在这一节我们就开始学习怎么获取用户输入事件从而让角色动起来．

## 输入事件
要让玩家可以操作角色动起来，首先我们需要监听用户输入事件．怎么监听呢？可以参考[ggez提供的例子](https://github.com/ggez/ggez/blob/master/examples/input_test.rs#L59)．其中有监听鼠标和键盘事件的示例代码，现在我们只需要监听键盘按下(`key_down_event`)事件.比虎画猫让我们开始编写代码吧！

首先引入下需要用到的模块：

```rust
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:1:11}}
```

接下来为Game实现`event::EventHandler`,这样我们的游戏就可以监听到键盘按键按下的事件了：

```rust
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:134}}

    // ...

{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:155:162}}
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:166}}

    // ...

{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:167}}
```

你可以运行代码，按下方向键试一下，在控制台中就会输出类似下面的信息：

```
Key pressed: Left
Key pressed: Left
Key pressed: Right
Key pressed: Up
Key pressed: Down
Key pressed: Left
```

是不是很神奇?

在使用`println`输出信息时使用了`{:?}`,这个是Rust提供的方便调试的，比如我们这里输出的`keycode`其实是一个枚举对象，因为它实现了Debug特征，所以这里可以很方便的把它转换为字符串输出到控制台．如果要对没有实现Debug特征的对象使用`{:?}`，代码就该编译出错了，好在Rust提供了Debug宏可以非常简单方便实现Debug特征．我们在[第１章的第３节](./c01-03-entities-components.html)介绍过宏，如果对宏不是很了解也可以回头再看一下.

## 资源
资源是用于在系统中共享状态信息的．为什么需要资源呢？因为组件实体模型不适合干这样的事．

接下来我们将添加一个资源，一个用于记录用户按键的队列．

```rust
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:48:52}}
```

当用户按下了按键，Game的方法`key_down_event`就会执行，这个我们上面已经试过了．现在我们需要在key_down_event方法中把`keycode`添加到队列里：

```rust
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:134}}

    // ...

{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:155:166}}

    // ...

{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:167}}
```

最后我们还需要注册下资源，就像注册组件一样．

```rust
// Registering resources
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:179:181}}

// Registering resources in main
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:295:312}}
```

## 输入处理

到这里我们已经有了一个持续记录用户按键的队列，接下来就是在系统中处理这个队列了，准确来说是处理队列中记录的按键．

```rust
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:94:121}}
```

最后我们还需要在渲染循环中运行输入处理代码．

```rust
{{#include ../../../code/rust-sokoban-c02-02/src/main.rs:135:143}}
```

当前的输入处理代码非常简单，就是根据玩家的输入控制角色的位置（虽然我们当前只有一个角色，但是理论上对于多个玩家多个角色的场景也可以这么玩）．

酷不？ 运行下代码应该就是这样的：

![Moving player](./images/input.gif)

注意到没？现在角色可以穿过墙和盒子．没关系，我们下一节就修复这个问题．

> **_CODELINK:_**  可以点[这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c02-02)获取当前完整代码.