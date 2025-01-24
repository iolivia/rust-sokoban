# 音效

本节我们将添加音效。简而言之，我们希望在以下情况下播放声音：

1.  当玩家撞击墙或障碍物时 — 提示他们无法通过
2.  当玩家将箱子放在正确位置时 - 提示 “做对了”
3.  当玩家将箱子放在错误位置时 - 提示 “操作错误”

## 音频存储

为了播放声音，需要加载wav文件。为了避免每次播放前临时加载，我们将创建一个音频存储，并在游戏开始时预先加载它们。
 
我们可以使用一个资源来定义音频存储。

```rust
{{#include ../../../code/rust-sokoban-c03-04/src/components.rs:audio_store}}
```

接下来添加初始化存储的代码，也就是预加载游戏所需的所有音效。

```rust
{{#include ../../../code/rust-sokoban-c03-04/src/map.rs:load_sounds}}
```

然后在初始化关卡时调用这个函数。

```rust
{{#include ../../../code/rust-sokoban-c03-04/src/map.rs:initialize_level}}
```


## 播放声音

最后， 我们需要在 audio store 中添加声音播放的代码。

```rust
{{#include ../../../code/rust-sokoban-c03-04/src/components.rs:audio_store_impl}}
```

现在在事件系统中播放音效：


```rust
// systems/events.rs
{{#include ../../../code/rust-sokoban-c03-04/src/systems/events.rs}}
```

现在运行游戏，享受这些音效吧！


<video width="75%" controls>
    <source src="./videos/audio.mov" type="video/mp4">
</video>

> **_代码链接:_**  你可以在[这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-04)查看所有代码.