# 项目搭建

建议使用[rustup](https://www.rust-lang.org/tools/install)安装管理Rust。安装好Rust后可以在命令行输入以下俩条命令，检查确认是否安装成功:

```
$ rustc --version
rustc 1.40.0
$ cargo --version
cargo 1.40.0
```

输出的版本信息未必都是这样的，但建议使用比较新的Rust版本。

## 创建项目

Cargo是Rust的包管理工具，可以使用它创建我们的游戏项目。首先切换到游戏项目存储路径，然后再输入以下命令：

```
$ cargo init rust-sokoban
```

命令执行成功后，会在当前目录下创建一个名称为`rust-sokoban`的文件夹。文件夹内部是这个样子的：

```
├── src
│   └── main.rs
└── Cargo.toml
```

切换到文件夹`rust-sokoban`并运行命令 `cargo run` ，你会看到类似下面的输出信息：

```
$ cargo run
   Compiling rust-sokoban v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.30s
     Running `../rust-sokoban/target/debug/rust-sokoban`
Hello, world!
```

## 添加游戏开发依赖
接下来让我们一起把默认生成的项目修改成一个游戏项目! 我们使用当前最受欢迎的2D游戏引擎之一的[ggez](https://ggez.rs/) 

还记得我们刚才在项目目录里看到的`Cargo.toml`文件吧？这个文件是用来管理项目依赖的，所以需要把我们需要使用到的`crate`添加到这个文件中。就像这样添加 [ggez](https://github.com/ggez/ggez) 依赖：

```toml
[dependencies]
ggez = "0.5.1"
```

> **_MORE:_** 更多关于Cargo.toml的信息可以看 [这里](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).

接下来再次执行`cargo run`.这次执行的会长一点，因为需要从[crates.io](https://crates.io)下载我们配置的依赖库并编译链接到我们库中。

```
cargo run
    Updating crates.io index
    Downloaded ....
    ....
    Compiling ....
    ....
    Finished dev [unoptimized + debuginfo] target(s) in 2m 15s
    Running `.../rust-sokoban/target/debug/rust-sokoban`
    Hello, world!
```

> **_NOTE:_** 如果你是使用的Ubuntu操作系统，在执行命令的时候可能会报错，如果报错信息有提到`alsa` 和`libudev`可以通过执行下面的命令安装解决：
```sudo apt-get install libudev-dev libasound2-dev```.

接下来我们在main.rs文件中使用`ggez`创建一个窗口。只是创建一个空的窗口，代码比较简单：

```rust
{{#include ../../../code/rust-sokoban-c01-01/src/main.rs}}
```

可以把代码复制到main.rs文件中，并再次执行`cargo run`,你会看到:

![Screenshot](../images/window.png)

## 基本概念和语法

现在我们有了个窗口，我们创建了个窗口耶！接下来我们一起分析下代码并解释下使用到的Rust概念和语法。

### 引入
您应该在其它编程语言中也接触过这个概念，就是把我们需要用到的依赖包（或crate）里的类型和命名空间引入到当前的代码作用域中。在Rust中，使用`use`实现引入功能:

```rust
// 从ggez命名空间引入conf, event, Context 和 GameResult 
{{#include ../../../code/rust-sokoban-c01-01/src/main.rs:1}}
```

### 结构体声明
```rust
{{#include ../../../code/rust-sokoban-c01-01/src/main.rs:4:7}}
```

> **_MORE:_**  查看更多结构体相关信息可以点 [这里](https://doc.rust-lang.org/book/ch05-00-structs.html).


### 实现特征
特征类似其它语言中的接口，就是用来表示具备某些行为的特定类型。在我们的示例中需要结构体Game实现EventHandler特征。

```rust
{{#include ../../../code/rust-sokoban-c01-01/src/main.rs:9:23}}
```

> **_MORE:_**  想更深入的了解特征可以点 [这里](https://doc.rust-lang.org/book/ch10-02-traits.html).


### 函数
我们还需要学习下怎么使用Rust编写函数:

```rust
{{#include ../../../code/rust-sokoban-c01-01/src/main.rs:14:17}}
```

你可能会疑惑这里的`self`是几个意思呢？这里使用`self`代表函数`update`是属于结构体的实例化对象而不是静态的。

> **_MORE:_**  想深入了解函数可以点 [这里](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html).

### 可变语法
你可能更疑惑`&mut self`这里的`&mut`是做什么的? 这个主要用来声明一个对象（比如这里的`self`）是否可以被改变的。再来看个例子：

```rust
let a = 10; // a是不可变的,因为没有使用`mut`声明它是可变的
let mut b = 20; // b是可变的，因为使用了`mut`声明了它是可变的
```

再回头看`update`函数，我们使用了`&mut `声明self是实例对象的可变引用。有没有点感觉了, 要不我们再看一个例子：

```rust
// 一个简单的结构体X
struct X {
    num: u32
}

//结构体X的实现代码块
impl X {
    fn a(&self) { self.num = 5 } 
    // 在函数a中不能修改`self`，这会编译失败的，因为是使用的`&self`

    fn b(&mut self) { self.num = 5 } 
    // 在函数b中可以修改`self`,因为使用的是`&mut self`
}
```

> **_MORE:_**  想更多的了解`可变性`可以看 [这里](https://web.mit.edu/6.005/www/fa15/classes/09-immutability/) (虽然是使用的Java作为演示语言讲解的，但对于理解可变性还是很有帮助地), 另外还可以看 [这里](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).

对代码和Rust语法的简单介绍就先到这里，让我们继续前进吧，下一节见!

> **_CODELINK:_**  要获取本节的完整代码可以点 [这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c01-01).
