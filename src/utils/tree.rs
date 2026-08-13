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

/// Build a tree string from relative paths and render it with box-drawing
/// glyphs, merging shared directory prefixes, e.g.
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
/// with children are directories, terminal components are files. Output order
/// is alphabetical (via [`BTreeMap`]) and deterministic.
pub fn render_tree(paths: &[&str]) -> String {
  let mut root = Node::new();
  for path in paths {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if !parts.is_empty() {
      insert(&mut root, &parts);
    }
  }
  let mut out = String::new();
  emit(&mut out, &root);
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
fn emit(out: &mut String, root: &Node) {
  for (name, child) in &root.children {
    out.push_str(name);
    if child.children.is_empty() {
      out.push('\n');
    } else {
      emit_entries(out, "", child);
    }
  }
}

/// Render a directory's `/` and its indented children.
fn emit_entries(out: &mut String, indent: &str, dir: &Node) {
  out.push('/');
  out.push('\n');
  let mut iter = dir.children.iter().peekable();
  while let Some((name, child)) = iter.next() {
    let is_last = iter.peek().is_none();
    let connector = if is_last { "└── " } else { "├── " };
    out.push_str(indent);
    out.push_str(connector);
    out.push_str(name);
    let child_indent = if is_last {
      format!("{indent}    ")
    } else {
      format!("{indent}│   ")
    };
    if child.children.is_empty() {
      out.push('\n');
    } else {
      emit_entries(out, &child_indent, child);
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn empty_paths_produce_empty_tree() {
    assert_eq!(render_tree(&[]), "");
  }

  #[test]
  fn single_flat_file() {
    assert_eq!(render_tree(&["Text/a.txt"]), "Text/\n└── a.txt\n");
  }

  #[test]
  fn merges_shared_directory_until_last_leaf() {
    let out =
      render_tree(&["Text/2024/08/x.pdf", "Text/2024/08/y.pdf", "Image/a.png"]);
    assert_eq!(
      out,
      "Image/\n└── a.png\nText/\n└── 2024/\n    └── 08/\n        ├── x.pdf\n        └── y.pdf\n"
    );
  }

  #[test]
  fn dirs_with_no_files_still_rendered_as_dir() {
    // A path ending in a directory-like component with a colliding leaf keeps
    // rendering the children; here `2024` has files so it stays a directory.
    let out = render_tree(&["Text/2024/08/a.pdf", "Text/2024/09/b.pdf"]);
    assert!(out.contains("Text/\n└── 2024/\n    ├── 08/\n    │   └── a.pdf\n    └── 09/\n        └── b.pdf\n"), "tree:\n{out}", out = out);
  }
}
