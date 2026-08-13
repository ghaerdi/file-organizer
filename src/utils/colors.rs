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

/// The color of a whole line of output (category headers, file names, ...).
#[derive(Clone, Copy, Debug)]
pub enum Color {
  /// Category directory name (e.g. `Image`).
  Category,
  /// A real (executed) move line, `Moved to ...`.
  Moved,
  /// A previewed (dry-run) move line, `Will move to ...`.
  Previewed,
  /// The final summary line ("Done — moved N files ...").
  Summary,
}

impl Colors {
  /// Build from the current process, enabling ANSI codes only when both
  /// stdout and stderr are interactive terminals.
  pub fn from_env() -> Self {
    Self {
      enabled: std::io::stdout().is_terminal(),
    }
  }

  /// Wrap `text` in the ANSI codes for `color`, or return it unchanged when
  /// colors are disabled.
  pub fn paint(self, color: Color, text: &str) -> String {
    if !self.enabled {
      return text.to_string();
    }
    // Codes are only the prefix+suffix; the interior `text` is never
    // interpreted, so no user input can inject escape sequences here.
    let (pre, post) = ansi(color);
    format!("{pre}{text}{post}")
  }
}

/// Map a [`Color`] to its ANSI "start / reset" pair.
///
/// SGR resets to `\x1b[0m`, which is defensive: it also clears any background
/// the surrounding terminal set, rather than relying on the caller to chain.
const fn ansi(color: Color) -> (&'static str, &'static str) {
  match color {
    Color::Category => ("\x1b[1;34m", "\x1b[0m"), // bold blue
    Color::Moved => ("\x1b[1;32m", "\x1b[0m"),    // bold green
    Color::Previewed => ("\x1b[1;33m", "\x1b[0m"), // bold yellow
    Color::Summary => ("\x1b[1;36m", "\x1b[0m"),  // bold cyan
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn disabled_colors_leave_text_untouched() {
    let colors = Colors { enabled: false };
    assert_eq!(colors.paint(Color::Category, "Image"), "Image");
    assert_eq!(colors.paint(Color::Summary, "Done"), "Done");
  }

  #[test]
  fn enabled_colors_wrap_with_ansi() {
    let colors = Colors { enabled: true };
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
