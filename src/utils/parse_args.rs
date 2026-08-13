/// Parsed command-line arguments.
pub struct Args {
  /// Whether `--dry-run` was passed. When true, organisation is previewed and
  /// no filesystem writes are performed.
  pub dry_run: bool,
  /// Whether `--by-date` was passed. When true, files are moved into
  /// year/month subfolders (e.g. `Text/2024/08/`) inside their category.
  pub by_date: bool,
  /// Flag-free positional arguments (program name first, then any directory
  /// path the user supplied). Same shape as the raw `env::args` slice with
  /// flags removed.
  pub rest: Vec<String>,
}

/// Parse argv into a [`Args`], recognising `--dry-run` wherever it appears and
/// returning the remaining argument slice untouched.
pub fn parse_args(args: &[&str]) -> Args {
  let mut dry_run = false;
  let mut by_date = false;
  let mut rest = Vec::with_capacity(args.len());

  for arg in args {
    match *arg {
      "--dry-run" => dry_run = true,
      "--by-date" => by_date = true,
      _ => rest.push((*arg).to_string()),
    }
  }
  Args {
    dry_run,
    by_date,
    rest,
  }
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]
  use super::*;

  #[test]
  fn no_flags_returns_default() {
    let parsed = parse_args(&["organizer"]);
    assert!(!parsed.dry_run, "dry_run defaults to false");
    assert_eq!(parsed.rest, vec!["organizer".to_string()]);
  }

  #[test]
  fn dry_run_flag_set_when_present() {
    let parsed = parse_args(&["organizer", "--dry-run"]);
    assert!(parsed.dry_run, "dry_run should be true");
    // Flag stripped, program name preserved.
    assert_eq!(parsed.rest, vec!["organizer".to_string()]);
  }

  #[test]
  fn dry_run_with_path_flag_comes_first() {
    let parsed = parse_args(&["organizer", "--dry-run", "~/Downloads"]);
    assert!(parsed.dry_run);
    assert_eq!(
      parsed.rest,
      vec!["organizer".to_string(), "~/Downloads".to_string()]
    );
  }

  #[test]
  fn dry_run_flag_after_path() {
    let parsed = parse_args(&["organizer", "~/Downloads", "--dry-run"]);
    assert!(parsed.dry_run);
    assert_eq!(
      parsed.rest,
      vec!["organizer".to_string(), "~/Downloads".to_string()]
    );
  }

  #[test]
  fn unknown_flags_are_treat_as_positional() {
    let parsed = parse_args(&["organizer", "--verbose"]);
    assert!(!parsed.dry_run);
    assert_eq!(
      parsed.rest,
      vec!["organizer".to_string(), "--verbose".to_string()]
    );
  }

  #[test]
  fn by_date_flag_set_when_present() {
    let parsed = parse_args(&["organizer", "--by-date"]);
    assert!(parsed.by_date, "by_date should be true");
    assert!(!parsed.dry_run);
    assert_eq!(parsed.rest, vec!["organizer".to_string()]);
  }

  #[test]
  fn by_date_and_dry_run_compose() {
    let parsed =
      parse_args(&["organizer", "~/Download", "--dry-run", "--by-date"]);
    assert!(parsed.dry_run);
    assert!(parsed.by_date);
    assert_eq!(
      parsed.rest,
      vec!["organizer".to_string(), "~/Download".to_string()]
    );
  }
}
