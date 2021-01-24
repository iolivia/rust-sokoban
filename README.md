[![Code-samples-compile](https://github.com/iolivia/rust-sokoban/workflows/Code-samples-compile/badge.svg)](https://github.com/iolivia/rust-sokoban/actions?query=workflow%3ACode-samples-compile)

[English](README-en.md)

# [Rust编写推箱子游戏教程 ](https://sokoban.iolivia.me)

## :dart: 这是什么?
这里是使用[mdbook](https://github.com/rust-lang/mdBook)创建的《Rust编写推箱子游戏》教程和相关代码。想在线阅读下？ 好嘞，给您安排，点这里[sokoban.iolivia.me](https://sokoban.iolivia.me)(当前只支持英文)。

<img src="src/images/readme.gif" width="80%">

- [x] 翻译第一章游戏开发基础  
- [x] 翻译第二章实现基本功能  
- [ ] 翻译第三章开发高级功能  
- [ ] 支持在线阅读

## :running: 怎么做?

### 想在自己的电脑上阅读?
正如上面介绍，本教程使用`mdbook`编写，所以如果需要在本机浏览阅读本教程需要先安装`mdbook`,可以使用cargo安装，就像这个样子：

```bash
$ cargo install mdbook
```

更多安装方式可以参考[mdbook](https://github.com/rust-lang/mdBook)。

安装成功后就可以把项目clone到本机并启动`mdbook serve`：

```bash
$ git clone  git@github.com:iolivia/rust-sokoban.git  
$ cd rust-cokoban/books/zh_CN
$ mdbook serve
```

启动成功后就可以打开浏览器访问http://localhost:3000/开始阅读本教程了。

### 运行教程代码

可以像这个样子运行教程中的代码:

```
$ cd code/rust-sokoban-c01-01
$ cargo run --release
```

## :muscle: 欢迎参与贡献

### 新增章节

你可以通过为本教程新增章节的方式对本项目做出贡献。

新增章节的方式：

1. 复制最近的章节目录`code/rust-sokoban-x` 到  `code/rust-sokoban-x+1`
1. 在新创建的目录中编写你想添加的内容。请尽量保证章节间独立并容易理解掌握
1. 确保代码是正确的（最好连警告也没有）
1. 把新增的内容添加到 `src/SUMMARY.md` 中 - markdown文件名格式最好遵循 `cxx-yy-text.md`这个样式.
1. 使用markdown文件的格式编写指向 `code/rust-sokoban-x+1` 文件夹的链接
1. 在章节的后面还可以添加一个`gif`格式的图片，用于展示。可以录屏然后转换为`gif`格式(我是使用的ffmpeg:`ffmpeg -i Screen_Recording.mov -r 60 -f gif - > moves.gif` )。
1. 如果你想展示文件目录列表可以使用`tree --dirsfirst -I target`
1. 最后发起一个合并请求 ​，​然后​就​可以​等着:clap: :tada:

### 问题反馈
你也可以通过提出问题和改进意见的方式支持本项目。在创建新的issue前请先浏览下issue列表以免重复提交.

## :car: 开源协议
MIT
