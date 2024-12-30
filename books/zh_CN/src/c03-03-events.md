# 声音和事件

在这个 section 中，我们将工作于添加事件，这些事件将在后续阶段用来添加声音效果。在短语中，我们想在以下情况下播放声音：

1. 当玩家击打墙或障碍时 - 为了让他们知道不能通过
2. 当玩家把箱放在正确的位置 - 以表明 "你做得对"
3. 当玩家把箱放在错误的位置 - 以表示move的错误

实际上播放声音并不是太难，ggez提供了这个功能，但我们目前面临的问题是需要确定*何时*播放声音。

让我们从box on correct spot来看。我们可能会使用游戏状态系统，并且会循环遍历boxes和spots来检查是否处于这种情况，然后播放声音。但是，这并不是一种好主意，因为我们将每次循环都尝试多次，造成不必要的重复和播放太快。

我们可以尝试在此过程中保持一些状态，但这并不感兴趣。我们的主要问题是，我们无法通过仅检查状态来做到这一点，而必须使用一种有反应性的模型，当发生某件事情时就能让系统作出反应。

我们会使用事件模型。这意味着当一个框架发生变化（如玩家击打墙或移动箱子）时，将引发一个事件。然后，我们可以在另一端接收这个事件，并根据其类型执行相应的操作。这个系统可以复用。

## 事件实现

让我们开始 discussing how we will implement events。

1.  玩家击打障碍 - 这可以是事件本身，通过输入系统当玩家试图移动但无法移动时会引发
2.  箱放在正确或错误的位置 - 我们可以将其表示为一个单独的事件，其中包含是否 correct_spot 的属性（我稍后再解释这个属性）

## 变化类型

我们需要用enum 来定义各种事件类型。我们曾使用过enum（例如Rendering类型和box颜色），但是这次我们要把它的潜力全推到使用，特别是我们可以在其中添加属性。

查看事件定义，它可能是这样的。

```rust
// events.rs
{{#include '../../../code/rust-sokoban-c03-03/src/events.rs'}}
```

## 事件资源

现在，我们需要一个resource来接收事件。这将是一个多生产者单消费者模型。我们会有多个系统添加事件，而一个system（events system）会只消费该事件。

```rust
// components.rs
{{#include '../../../code/rust-sokoban-c03-03/src/components.rs:events'}}
```

## 发送事件

现在，我们需要将两个事件在input_system中添加：EntityMoved和PlayerHitObstacle。

```rust
// input.rs
{{#include '../../../code/rust-sokoban-c03-03/src/systems/input.rs:run_input}}

    /// Code omitted
    /// ......
    /// ......
{{#include '../../../code/rust-sokoban-c03-03/src/systems/input.rs:event_add}}
}
```

## 消费事件 - events系统

现在它是时候添加一个events system来处理事件。

我们将会对每个事件做出以下决策：

*   Event::PlayerHitObstacle -> 这是声音播放的位置，但我们在添加音频部分之前要等
*   Event::EntityMoved(EntityMoved { id }) -> 这是我们将在其中添加逻辑来检查移动的实体是否为box，并且它是否位于一个 spot 上
*   Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) -> 这是声音播放的位置，但在增加音频部分之前要等

```rust
// systems/events.rs
{{#include '../../../code/rust-sokoban-c03-03/src/systems/events.rs'}}
```

事件处理系统的结尾很重要，因为处理一个事件可能会导致另一个事件被创建。因此，我们必须将事件添加回世界。

> ***CODELINK***：您可以在这个示例中看到完整代码 [这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-03).