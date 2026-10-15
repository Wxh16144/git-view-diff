# git-view-diff

Render `git diff` as a side-by-side, single-file HTML page, powered by [PatchViewer](https://github.com/megatops/PatchViewer).

## Install

Download the binary for your platform from [Releases](https://github.com/Wxh16144/git-view-diff/releases) and put it on your `PATH`. Git picks it up as the `git view-diff` subcommand.

```sh
# aarch64-apple-darwin | x86_64-apple-darwin | x86_64-unknown-linux-gnu | aarch64-unknown-linux-gnu
TARGET=aarch64-apple-darwin
curl -fsSL https://github.com/Wxh16144/git-view-diff/releases/latest/download/git-view-diff-$TARGET.tar.gz | tar xz -C ~/.local/bin
```

## Usage

```sh
git view-diff                          # working tree changes -> $TMPDIR/git-view-diff.html
git view-diff --cached                 # other args are passed to git diff
git view-diff main...HEAD -- src
git view-diff -o review.html --open    # custom output file, then open in browser
git show HEAD | git view-diff          # read a diff/patch from stdin
```
