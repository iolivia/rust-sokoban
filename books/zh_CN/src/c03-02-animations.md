# 动画

在本节中，我们将学习如何为游戏添加动画，我们将从一些基本的动画开始，但你可以根据本教程中的想法添加更复杂的动画。我们将添加两种动画：让玩家眨眼和让方块在原地轻微晃动。

## 什么是动画？

动画实际上就是在特定时间间隔播放的一组帧，给人以运动的错觉。可以把它想象成一个视频（视频就是按顺序播放的一系列图像），但帧率要低得多。

例如，要让我们的玩家眨眼，我们需要三个动画帧：

1. 我们当前的玩家，眼睛睁开
2. 玩家眼睛稍微闭合
3. 玩家眼睛完全闭合

如果我们按顺序播放这三帧，你会注意到看起来就像玩家在眨眼。你可以通过打开图像并在图像预览中快速切换它们来试试这个效果。

这里有一些需要注意的事项：

* 资源需要针对特定的帧率设计 - 对我们来说，我们将使用 250 毫秒，这意味着我们每 250 毫秒播放一个新的动画帧，所以我们每秒有 4 帧
* 资源之间需要保持一致性 - 想象一下如果我们有两种不同的玩家，它们有不同的资源和不同外观的眼睛，我们需要确保当我们创建上述三帧时它们是一致的，否则两个玩家会以不同的速率眨眼
* 为大量帧设计资源是一项繁重的工作，所以我们会尽量保持动画简单，只关注关键帧

## 它将如何工作？

那么这在我们现有的推箱子游戏中将如何工作呢？我们需要：

1. 修改我们的可渲染组件以允许多个帧 - 我们也可以创建一个新的可渲染组件来处理动画可渲染对象，并保留现有的组件用于静态可渲染对象，但现在把它们放在一起感觉更整洁
2. 修改玩家实体构造以接受多个帧
3. 在我们的渲染循环中跟踪时间 - 我们稍后会详细讨论这个问题，所以如果现在不太清楚为什么需要这样做也不用担心
4. 修改渲染系统，考虑帧数、时间和在给定时间应该渲染的帧

## 资源

让我们为玩家添加新的资源，它应该是这样的。注意我们创建了一个按顺序命名帧的约定，这不是严格必要的，但它将帮助我们轻松跟踪顺序。

![玩家1](./images/player_1.png)
![玩家2](./images/player_2.png)
![玩家3](./images/player_3.png)

```sh
├── resources
│   └── images
│       ├── box_blue.png
│       ├── box_red.png
│       ├── box_spot_blue.png
│       ├── box_spot_red.png
│       ├── floor.png
│       ├── player_1.png
│       ├── player_2.png
│       ├── player_3.png
│       └── wall.png
```

## 可渲染特性

现在，让我们更新可渲染组件，把他们变成一个路径列表以接收多个帧

我们还要添加两个新函数，用于构建两种类型的可渲染对象：一种是单一路径，另一种是多个路径。这两个函数是关联函数，因为它们与结构体 `Renderable` 相关联，但它们相当于其他语言中的静态函数，因为它们不操作实例（注意它们没有接收 `&self` 或 `&mut self` 作为第一个参数，这意味着我们可以在结构体的上下文中调用它们，而不是在结构体实例中调用）。它们也类似于工厂函数，因为它们封装了实际构建对象之前所需的逻辑和验证。

> **_更多:_**  了解更多关于关联函数的信息。[这里](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions).

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:renderable}}

{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:renderable_impl}}
}
```

接下来，我们需要一种方法来判断可渲染对象是动画还是静态的，这将在渲染系统中使用。我们可以将 `paths` 成员变量设为公共，让渲染系统获取 `paths` 的长度并根据长度推断，但有一种更符合语言习惯的方式。我们可以为**可渲染对象**的类型添加一个枚举，并在**可渲染对象**上添加一个方法来获取该类型。这样，我们将类型的逻辑封装在**可渲染对象**内部，同时可以保持 `paths` 私有。你可以将类型的声明放在 `components.rs` 的任何位置，但最好放在 `Renderable` 声明的旁边。

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:renderable_kind}}
```

现在让我们添加一个函数，根据内部的 `paths` 告诉我们可渲染对象的类型。

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:renderable_kind_fn}}
```

最后，由于我们将 `paths` 设为私有，因此需要让可渲染对象的使用者能够从列表中获取特定路径。对于静态可渲染对象，这将是第 0 个路径（唯一的一个），而对于动画路径，我们将让渲染系统根据时间决定应该渲染哪个路径。唯一需要注意的地方是，如果请求的帧数超出了我们拥有的范围，我们将通过对长度取模来循环处理。

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:renderable_path_fn}}
```

## 实体创建

接下来，我们将更新玩家实体的创建，以考虑多个路径。请注意，现在我们使用 `new_animated` 函数来构建可渲染对象

```rust
// entities.rs
{{#include ../../../code/rust-sokoban-c03-02/src/entities.rs:create_box}}
```

并且让我们更新所有其他部分以使用 `new_static` 函数——以下是我们如何在墙壁实体创建中实现这一点的示例，请随意将其应用到其他静态实体中。


```rust
// entities.rs
{{#include ../../../code/rust-sokoban-c03-02/src/entities.rs}}
```

## 时间

我们还需要另一个组件来记录时间。时间与此有什么关系？它又是如何与帧率联系起来的呢？基本思路是这样的：ggez 控制渲染系统的调用频率，这取决于帧率，而帧率又取决于我们在游戏循环的每次迭代中做了多少工作。由于我们无法控制这一点，在一秒钟内，渲染系统可能会被调用 60 次、57 次，甚至可能只有 30 次。这意味着我们的动画系统不能基于帧率，而需要基于时间。

正因如此，我们需要记录增量时间（delta time），也就是上一次循环和当前循环之间经过的时间。由于增量时间比我们设定的动画帧间隔（我们决定为 250 毫秒）要小得多，因此我们需要累积增量时间，也就是从游戏启动开始到现在经过的总时间。

> **_更多:_**  了解更多关于增量时间、帧率和游戏循环的详细介绍 [这里](https://medium.com/@dr3wc/understanding-delta-time-b53bf4781a03#:~:text=Delta%20time%20describes%20the%20time,drawn%20and%20the%20current%20frame.&text=If%20you%20read%20my%20article,until%20the%20game%20is%20stopped.), [here](https://www.reddit.com/r/pcmasterrace/comments/29qcqr/an_explanation_of_game_loops_fps_and_delta_time/) or [here](https://www.youtube.com/watch?v=pctGOMDW-HQ&list=PLlrATfBNZ98dC-V-N3m0Go4deliWHPFwT&index=37) .

现在，我们为时间添加一个资源。这并不适合放入组件模型中，因为时间只是一些需要维护的全局状态。


```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:create_time}}
```

现在，让我们在主循环中更新时间。幸运的是，ggez 提供了一个函数来获取增量时间，所以我们只需要累积它即可。

```rust
// main.rs
{{#include ../../../code/rust-sokoban-c03-02/src/main.rs:update}}
```


## 渲染系统

现在，我们来更新渲染系统。我们将从可渲染对象中获取类型，如果是静态的，我们直接使用第一帧；否则，我们根据增量时间来确定要使用哪一帧。

首先，我们添加一个函数来封装获取正确图像的逻辑。

```rust
// rendering.rs
{{#include ../../../code/rust-sokoban-c03-02/src/systems/rendering.rs:get_image}}
```

最后，我们在 `run` 函数中使用新的 `get_image` 函数（我们还需要将时间添加到 `SystemData` 定义中，并添加一些导入，但基本上就是这样了）。

```rust
// rendering.rs
{{#include ../../../code/rust-sokoban-c03-02/src/systems/rendering.rs:run_rendering}}
```

## 箱体动画

现在我们已经学会了如何实现这一点，接下来我们将其扩展到让箱子也实现动画效果。我们只需要添加新的资源并调整实体创建部分，其他部分应该就能正常工作了。以下是我使用的资源，你可以随意复用或创建新的资源！

![Box red 1](./images/box_red_1.png)
![Box red 2](./images/box_red_2.png)
![Box blue 1](./images/box_blue_1.png)
![Box blue 2](./images/box_blue_2.png)


## 总结一下

这一部分内容比较长，但希望你喜欢！以下是游戏现在应该呈现的效果。

![Sokoban animations](./images/animations.gif)

> **_代码链接:_**  在这个例子中你可以看到完整的代码 [here](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-02).