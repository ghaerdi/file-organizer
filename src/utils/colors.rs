use std::io::IsTerminal;

/// ANSI color support: wraps strings with escape codes only when the output is
/// a terminal. When piped to a file or another program (or the platform is
/// missing `IsTerminal`) the wrappers fall back to the plain string, so colored
/// output never leaks raw escape codes into non-interactive output.
///
/// `std::io::IsTerminal` is Unix- and Windows-stable (gated on rustc 1.70+),
/// so this degrades gracefully across platforms.
#[derive(Clone, Copy)]
pub struct Colors {
  enabled: bool,
}

/// A named role in the output, each mapped to an ANSI style.
#[derive(Clone, Copy, Debug)]
pub enum Color {
  /// A category directory name (e.g. `Image` or `Image/`). Bold blue.
  Category,
  /// A real (executed) move: a file leaf or the "Moved to" hint. Bold green.
  Moved,
  /// A previewed (dry-run) move: a file leaf or the "Would move" hint. Bold
  /// yellow.
  Previewed,
  /// The final summary line ("Done — moved N files ..."). Muted cyan,
  /// deliberately calmer than the tree so the tree stays the focal point.
  Summary,
  /// The `├──`/`└──` box-drawing glyphs linking tree nodes. Dim gray so the
  /// hierarchy reads without shouting over the category/file names.
  Connector,
}

impl Colors {
  /// Build from the current process, enabling ANSI codes only when stdout is
  /// an interactive terminal.
  pub fn from_env() -> Self {
    Self {
      enabled: std::io::stdout().is_terminal(),
    }
  }

  #[cfg(test)]
  pub const fn disabled() -> Self {
    Self { enabled: false }
  }

  #[cfg(test)]
  pub const fn enabled_for_test() -> Self {
    Self { enabled: true }
  }

  /// Wrap `text` in the ANSI codes for `color`, or return it unchanged when
  /// colors are disabled.
  pub fn paint(self, color: Color, text: &str) -> String {
    if !self.enabled {
      return text.to_string();
    }
    let (pre, post) = ansi(color);
    format!("{pre}{text}{post}")
  }
}

/// Map a [`Color`] to an ANSI "start / reset" pair.
///
/// The SGR reset is `\x1b[0m`, which is defensive: it also clears any
/// background the surrounding terminal set, rather than relying on the caller
/// to chain.
const fn ansi(color: Color) -> (&'static str, &'static str) {
  match color {
    Color::Category => ("\x1b[1;34m", "\x1b[0m"), // bold blue
    Color::Moved => ("\x1b[1;32m", "\x1b[0m"),    // bold green
    Color::Previewed => ("\x1b[1;33m", "\x1b[0m"), // bold yellow
    Color::Summary => ("\x1b[36m", "\x1b[0m"),    // muted (non-bold) cyan
    Color::Connector => ("\x1b[2;90m", "\x1b[0m"), // dim gray
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn disabled_colors_leave_text_untouched() {
    let colors = Colors::disabled();
    assert_eq!(colors.paint(Color::Category, "Image"), "Image");
    assert_eq!(colors.paint(Color::Summary, "Done"), "Done");
    assert_eq!(colors.paint(Color::Connector, "├── "), "├── ");
  }

  #[test]
  fn enabled_colors_wrap_with_ansi() {
    let colors = Colors::enabled_for_test();
    let painted = colors.paint(Color::Category, "Image");
    assert!(
      painted == "\x1b[1;34mImage\x1b[0m" || painted == "Image",
      "got: {painted:?}",
      painted = painted
    );
  }

  #[test]
  fn ansi_pairs_are_balanced() {
    for color in [
      Color::Category,
      Color::Moved,
      Color::Previewed,
      Color::Summary,
      Color::Connector,
    ] {
      let (start, end) = ansi(color);
      assert!(
        start.starts_with('\x1b'),
        "missing ESC prefix for {color:?}",
        color = color
      );
      assert!(
        end.starts_with('\x1b'),
        "missing ESC for {color:?}",
        color = color
      );
      assert!(
        end.ends_with('m'),
        "missing SGR terminator for {color:?}",
        color = color
      );
    }
  }
}
