# 声音和事件
# Sound Effects

在这个 section 中，我们将在游戏中添加sound effects。我们的目标是在以下情况下播放声音：

1.  当玩家撞击 wall 或 obstacle 时—to告知他们无法通过
2.  当玩家放置box 在正确位置时—as一种提示“你成功完成了”
3.  当玩家放置box 在错误位置时—as一种提示(move 是错的)

## 音频存储

现在，为了在游戏中播放声音，我们需要先把wav文件加载到一个audio store 中。这将避免每次想要播放声音都在load.wav上。
 
我们可以使用一个资源来定义音频存储。

```rust
// components.rs
{{#include '../../../code/rust-sokoban-c03-04/src/components.rs:audio_store'}}
```

然后，我们需要编写初始化 store 的代码，这意味着在游戏开始前预载所有用于游戏的sound。
 
我们可以通过在map中创建一个load_sounds函数来实现这一点：

```rust
{{#include '../../../code/rust-sokoban-c03-04/src/map.rs:load_sounds}}
```

当我们是初始化游戏级别时，我们需要调用这个函数。

```rust
{{#include '../../../code/rust-sokoban-c03-04/src/map.rs:initialize_level}}
```

## 播放声音

最后， 我们需要在 audio store 中添加声音播放的代码。

```rust
// components.rs
{{#include '../../../code/rust-sokoban-c03-04/src/components.rs:audio_store_impl'}}
```

然后，在events系统中，我们可以使用这个实现来 playback 鲜样：

```rust
// systems/events.rs
{{#include '../../../code/rust-sokoban-c03-04/src/systems/events.rs}}
```

现在，我们就可以在玩家完成各种动作时添加声音！让我们启动游戏并享受这些音频效果！

<video width="75%" controls>
    <source src="./videos/audio.mov" type="video/mp4">
</video>

> **_CODELINK:_**  你可以在[这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-04).