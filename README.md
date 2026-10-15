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

## Viewer

Besides the built-in PatchViewer, `git view-diff` can hand off to [pulls.review](https://github.com/antfu/pulls.review) (requires Node.js 20+, run via `npx`):

```sh
git config view-diff.viewer pulls.review            # this repo only
git config --global view-diff.viewer pulls.review   # all repos
GIT_VIEW_DIFF_VIEWER=pulls.review@latest git view-diff   # one-off, overrides git config
```

| Value | Viewer |
| --- | --- |
| `patchviewer` (default) | Built-in PatchViewer |
| `pulls.review` | `pulls.review@^0.3` |
| `pulls.review@<version>` | Any npm version or tag, e.g. `@latest` |

With pulls.review, arguments follow [its CLI](https://github.com/antfu/pulls.review/tree/main/packages/cli) instead of `git diff`. Without a target, `--worktree` is added: uncommitted changes against `HEAD`, including staged and untracked files (the built-in viewer shows unstaged changes only). `-o` and stdin input are not supported.
