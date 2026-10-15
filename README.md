# git-view-diff

Render `git diff` as a side-by-side, single-file HTML page, powered by [PatchViewer](https://github.com/megatops/PatchViewer).

## Install

Download the binary for your platform from [Releases](https://github.com/Wxh16144/git-view-diff/releases) and put it on your `PATH`. Git picks it up as the `git view-diff` subcommand.

## Usage

```sh
git view-diff                          # working tree changes -> patch-view.html
git view-diff --cached                 # other args are passed to git diff
git view-diff main...HEAD -- src
git view-diff -o review.html --open    # custom output file, then open in browser
git show HEAD | git view-diff          # read a diff/patch from stdin
```
