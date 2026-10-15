# git-view-diff

把 `git diff` 渲染成并排对比（side-by-side）的单文件 HTML，页面来自 [PatchViewer](https://github.com/megatops/PatchViewer)。

## Install

从 [Releases](../../releases) 下载对应平台的二进制，放进 `PATH` 即可作为 `git view-diff` 子命令使用。

## Usage

```sh
git view-diff                          # 工作区改动 -> patch-view.html
git view-diff --cached                 # 其余参数原样传给 git diff
git view-diff main...HEAD -- src
git view-diff -o review.html --open    # 指定输出文件并用浏览器打开
git show HEAD | git view-diff          # 从 stdin 读取 diff/patch
```

## Development

PatchViewer 以 submodule 形式引入，编译时内嵌进二进制：

```sh
git clone --recursive <repo>
cargo build --release
```

更新 PatchViewer：`git submodule update --remote PatchViewer`。

发布：推送 `v*` tag，GitHub Actions 自动构建并发布各平台二进制。
