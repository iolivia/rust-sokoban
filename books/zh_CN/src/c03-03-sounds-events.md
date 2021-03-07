# 声音和事件

本节我们将给游戏添加声效.简单说就是在某些场景下播放声音:
1. 当角色碰到墙或障碍物时播放声音,让玩家知道不能穿越过去.
1. 当角色把盒子推到了正确的地方时播放声音,让玩家知道"这么干就对了"
1. 当角色把盒子推到了不正确的地方时播放声音,让玩家知道"这地儿不对"

根据ggez提供的功能实现播放声音并不难,我们需要解决的最大问题是确定在何时播放声音. 

以把盒子推到了正确的地方为例.我们很可能不断的检测游戏状态系统中保存的盒子和目标点的信息,如果匹配上了就播放声效.但这样有个问题,我们的检测会每秒种执行多次,也就是会播放声效多次.但我们只想播放一次.当然也可以通过维护一些状态信息做到只播放一次,但我们并不想这么做.我们不想通过循环检测状态信息,而是使用响应式模型解决这个问题.也就是当某个动作发生时,可以发送一个时间,然后在其它地方就可以监听这个时间并做出响应.比如当玩家把盒子推到正确的地方时触发一个事件,在其它地方监听到这个事件了就播放相应的音效.而且这个事件系统还能用于解决其它问题.

## 实现事件系统
接下来看下怎么实现事件系统.我们不使用组件也不使用实体(虽然也可以用),像实现输入队列一样使用资源.我们需要编写把事件放入到队列中和从资源中获取事件的代码,另外我还需要编写根据事件类型执行相应操作的代码.

## 事件
接下来我们看下需要什么样的事件:
1. 角色碰到障碍物事件- 这个事件是现成的可以在想移动却移动不成时在输入系统中触发这个事件
1. 把盒子推到正确/不正确的地方事件 - 我们可以用一个事件表示这俩种情况,只需要用一个属性区分就好.深入点说就是我们可以做个实体移动事件,当我们接受到实体移动事件时获取移动实体的ID,并判断这个移动的实体是否是盒子,如果是就判断它是不是移动到了正确的地方 (这也是创建事件链的示例-根据一个事件生成另一个事件)

## 事件类型
接下来就开始考虑怎么实现事件了.我们使用enum定义多种事件类型.先前我们已经使用过枚举类型了,像渲染类型,盒子颜色.但这次我们将用到Rust枚举更高级的功能.枚举一个最有趣的功能是每种枚举都可以携带相应属性.

上代码:

```rust
// events.rs
{{#include ../../../code/rust-sokoban-c03-03/src/events.rs:13:23}}
```

注意看第二个 `EntityMoved` 和第二个 `BoxPlacedOnSpot`. 这些就是我们定义用来携带属性的结构体.代码是这样的:

```rust
// events.rs
{{#include ../../../code/rust-sokoban-c03-03/src/events.rs:1:11}}
```

## Event queue resource
现在我们可以编写事件队列资源了.可以有很多系统往这个队列里发送数据,但只有一个系统(事件系统)从队列里消费数据.这是一个典型的多生产者单消费者模式.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:54:57}}
```

跟原来一样,还是要注册下资源的:

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:14:18}}
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:20}}
```

## 事件发送
现在我们已经有了往队列里放事件的方法.接下来我们就创建俩个在input_system中用到的事件:EntityMoved和 PlayerHitObstacle.

```rust
// input_system.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/input_system.rs:1:42}}
                    // ...
                    // ...
{{#include ../../../code/rust-sokoban-c03-03/src/systems/input_system.rs:83:124}}
```

为了读起来方便,这里省略了一些代码.其实我们就是在相应的地方添加了2行代码.

## 事件消费 - 事件系统
是时候添加处理消费事件的功能了,也就是事件系统.这个功能实现根据接受到的事件执行相应操作的逻辑.

接下来我们看下怎么处理每种类型的事件:
* Event::PlayerHitObstacle -> 播放相应音效
* Event::EntityMoved(EntityMoved { id }) -> 检查移动的实体是否是盒子,盒子是放到了正确的目标点还是放到了错误的目标点.
* Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) -> 播放相应音效.

```rust
// event_system.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:1:34}}
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:36:63}}
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:71:78}}

```

## Audio assets
现在已经添加好了事件,接下来我们开始添加声音素材.我们从这个 [素材包](https://opengameart.org/content/512-sound-effects-8-bit-style)里选了3个声音素材,你也可以使用自己的素材.

盒子放到正确地方时播放 [这个](./sounds/correct.wav)

盒子放到不正确地方时播放[这个](./sounds/incorrect.wav)

角色碰到障碍物时播放[这个](./sounds/wall.wav)

把这些声音素材添加到resources文件夹下的sounds文件夹下:

```
.
├── resources
│   ├── images
│   │   ├── box_blue_1.png
│   │   ├── box_blue_2.png
│   │   ├── box_red_1.png
│   │   ├── box_red_2.png
│   │   ├── box_spot_blue.png
│   │   ├── box_spot_red.png
│   │   ├── floor.png
│   │   ├── player_1.png
│   │   ├── player_2.png
│   │   ├── player_3.png
│   │   └── wall.png
│   └── sounds
│       ├── correct.wav
│       ├── incorrect.wav
│       └── wall.wav
├── Cargo.lock
└── Cargo.toml
```

## 声音仓库
现在为了播放声音我们需要加载一些wav文件.为了避免每次播放声音都重新加载一次,我们需要创建一个声音仓库,在游戏开始时就把所有的声音文件加载好.

我们使用一个资源做为声音仓库:

```rust
// audio.rs
{{#include ../../../code/rust-sokoban-c03-03/src/audio.rs:6:9}}
```

像往常一样注册下资源:

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:14:20}}
```

接下来添加初始化仓库的代码:

```rust
// audio.rs
{{#include ../../../code/rust-sokoban-c03-03/src/audio.rs:21:32}}
```

## 播放音效
最后是在仓库中添加播放音效的功能:

```rust
// audio.rs
{{#include ../../../code/rust-sokoban-c03-03/src/audio.rs:11:19}}
```

然后在声音系统中执行播放音效操作:

```rust
    // event_system.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:24:37}}
                        // ...
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:61:73}}
```

现在让我们运行程序,感受下声音效果吧!

<video width="75%" controls>
    <source src="./videos/audio.mov" type="video/mp4">
</video>

> **_CODELINK:_**  点 [这里](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-03)获取示例完整代码.
