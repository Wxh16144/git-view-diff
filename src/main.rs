use std::io::{self, IsTerminal, Read};
use std::process::{exit, Command, Stdio};
use std::{env, fs};

const TEMPLATE: &str = include_str!("../PatchViewer/PatchViewer.html");

const HELP: &str = "\
Render a diff as a side-by-side HTML page (powered by PatchViewer).

Usage:
  git view-diff [options] [<git diff args>...]
  <command> | git view-diff [options]

Options:
  -o, --output <file>  Output file [default: patch-view.html]
      --open           Open the output file in the default browser
  -h                   Print help
  -V, --version        Print version

Other arguments are passed to `git diff`, e.g.:
  git view-diff --cached
  git view-diff main...HEAD -- src
  git show HEAD | git view-diff -o head.html
";

fn main() {
    let mut output = String::from("patch-view.html");
    let mut open = false;
    let mut diff_args = Vec::new();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--output" => match args.next() {
                Some(v) => output = v,
                None => fail("missing value for --output"),
            },
            "--open" => open = true,
            "-h" | "--help" => {
                print!("{HELP}");
                return;
            }
            "-V" | "--version" => {
                println!("git-view-diff {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "--" => {
                diff_args.push(arg);
                diff_args.extend(args.by_ref());
            }
            _ => diff_args.push(arg),
        }
    }

    let diff = if diff_args.is_empty() && !io::stdin().is_terminal() {
        let mut buf = Vec::new();
        if let Err(e) = io::stdin().read_to_end(&mut buf) {
            fail(&format!("failed to read stdin: {e}"));
        }
        buf
    } else {
        git_diff(&diff_args)
    };
    let diff = String::from_utf8_lossy(&diff);

    if diff.trim().is_empty() {
        eprintln!("No changes.");
        return;
    }

    if let Err(e) = fs::write(&output, render(&diff)) {
        fail(&format!("failed to write {output}: {e}"));
    }
    println!("{output}");

    if open {
        open_in_browser(&output);
    }
}

fn git_diff(args: &[String]) -> Vec<u8> {
    let out = Command::new("git")
        .args(["diff", "--no-color", "--no-ext-diff"])
        .args(args)
        .stderr(Stdio::inherit())
        .output()
        .unwrap_or_else(|e| fail(&format!("failed to run git: {e}")));
    if !out.status.success() {
        exit(out.status.code().unwrap_or(1));
    }
    out.stdout
}

fn render(diff: &str) -> String {
    let script = format!(
        "<script>window.addEventListener(\"load\", () => diffRender({}, document.getElementById(\"diffResult\")));</script>\n",
        js_string(diff)
    );
    let at = TEMPLATE.rfind("</body>").expect("template has no </body>");
    let mut html = String::with_capacity(TEMPLATE.len() + script.len());
    html.push_str(&TEMPLATE[..at]);
    html.push_str(&script);
    html.push_str(&TEMPLATE[at..]);
    html
}

/// `<` is escaped so the diff can never close the surrounding `<script>`.
fn js_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' | '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn open_in_browser(path: &str) {
    let result = if cfg!(target_os = "macos") {
        Command::new("open").arg(path).status()
    } else if cfg!(windows) {
        Command::new("explorer").arg(path).status()
    } else {
        Command::new("xdg-open").arg(path).status()
    };
    if let Err(e) = result {
        eprintln!("git-view-diff: failed to open browser: {e}");
    }
}

fn fail(msg: &str) -> ! {
    eprintln!("git-view-diff: {msg}");
    exit(2)
}
