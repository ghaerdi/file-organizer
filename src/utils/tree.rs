use crate::utils::{Color, Colors};
use std::collections::BTreeMap;

/// A node in the tree built from destination paths. Could be a directory with
/// children, or a file leaf.
#[derive(Debug, Default)]
struct Node {
  children: BTreeMap<String, Self>,
}

impl Node {
  fn new() -> Self {
    Self::default()
  }
}

/// Build a coloured tree string from relative paths, rendering it with
/// box-drawing glyphs and merging shared directory prefixes, e.g.
///
/// ```text
/// Text/
/// ├── notes.md
/// └── 2024/
///     └── 08/
///         └── report.pdf
/// Image/
/// └── photo.png
/// ```
///
/// Paths are split on `/` and folded into a nested [`Node`] tree; components
/// with children are directories, terminal components are files. Directory
/// nodes are painted [`Color::Category`], file leaves are painted `leaf_color`,
/// and the connecting `├──`/`└──` glyphs are dimmed for hierarchy. When
/// `colors` is disabled every component is returned plain, so the rendered
/// output never leaks escape codes.
pub fn render_tree(
  paths: &[&str],
  colors: Colors,
  leaf_color: Color,
) -> String {
  let mut root = Node::new();
  for path in paths {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if !parts.is_empty() {
      insert(&mut root, &parts);
    }
  }
  let mut out = String::new();
  emit(&mut out, &root, colors, leaf_color);
  out
}

fn insert(node: &mut Node, parts: &[&str]) {
  let mut cur = node;
  for &component in parts {
    let child = cur.children.entry(component.to_string()).or_default();
    cur = child;
  }
}

/// Render the root's children (top-level categories) with no guide prefix,
/// recursing into directories with indented `├──`/`└──` connectors.
fn emit(out: &mut String, root: &Node, colors: Colors, leaf_color: Color) {
  for (name, child) in &root.children {
    if child.children.is_empty() {
      out.push_str(&colors.paint(leaf_color, name));
      out.push('\n');
    } else {
      emit_dir(out, "", name, child, colors, leaf_color);
    }
  }
}

/// Render a directory's `/` and its indented children.
fn emit_dir(
  out: &mut String,
  indent: &str,
  name: &str,
  dir: &Node,
  colors: Colors,
  leaf_color: Color,
) {
  out.push_str(&colors.paint(Color::Category, &format!("{name}/")));
  out.push('\n');
  let mut iter = dir.children.iter().peekable();
  while let Some((child_name, child)) = iter.next() {
    let is_last = iter.peek().is_none();
    let connector = if is_last { "└── " } else { "├── " };
    let child_indent = if is_last {
      format!("{indent}    ")
    } else {
      format!("{indent}│   ")
    };
    out.push_str(indent);
    out.push_str(&colors.paint(Color::Connector, connector));
    if child.children.is_empty() {
      out.push_str(&colors.paint(leaf_color, child_name));
      out.push('\n');
    } else {
      emit_dir(out, &child_indent, child_name, child, colors, leaf_color);
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  // A disabled palette renders plain text, keeping the string tests below
  // deterministic regardless of terminal support.
  const PLAIN: Colors = Colors::disabled();
  const MOVED: Color = Color::Moved;

  #[test]
  fn empty_paths_produce_empty_tree() {
    assert_eq!(render_tree(&[], PLAIN, MOVED), "");
  }

  #[test]
  fn single_flat_file() {
    assert_eq!(
      render_tree(&["Text/a.txt"], PLAIN, MOVED),
      "Text/\n└── a.txt\n"
    );
  }

  #[test]
  fn merged_shared_directory_until_last_leaf() {
    let out = render_tree(
      &["Text/2024/08/x.pdf", "Text/2024/08/y.pdf", "Image/a.png"],
      PLAIN,
      MOVED,
    );
    assert_eq!(
      out,
      "Image/\n└── a.png\nText/\n└── 2024/\n    └── 08/\n        ├── x.pdf\n        └── y.pdf\n"
    );
  }

  #[test]
  fn dirs_with_no_files_still_rendered_as_dir() {
    let out =
      render_tree(&["Text/2024/08/a.pdf", "Text/2024/09/b.pdf"], PLAIN, MOVED);
    assert!(
      out.contains(
        "Text/\n└── 2024/\n    ├── 08/\n    │   └── a.pdf\n    └── 09/\n        └── b.pdf\n"
      ),
      "tree:\n{out}",
      out = out
    );
  }

  #[test]
  fn enabled_colors_mark_dirs_connectors_and_leaves() {
    let colors = Colors::enabled_for_test();
    let out = render_tree(&["Text/a.txt", "Image/p.png"], colors, MOVED);
    // Directories and connector glyphs carry distinct ANSI; leaves use the
    // move color. Verify the palette trio appears.
    assert!(
      out.contains("\x1b[1;34mText/\x1b[0m"),
      "dir: {out}",
      out = out
    );
    assert!(
      out.contains("\x1b[1;32ma.txt\x1b[0m"),
      "leaf: {out}",
      out = out
    );
    assert!(
      out.contains("\x1b[2;90m└── \x1b[0m"),
      "connector: {out}",
      out = out
    );
  }
}
