use std::env;

mod error;
mod globals;
mod utils;

#[cfg(test)]
mod test_utils;

fn main() -> anyhow::Result<()> {
  let colors = utils::Colors::from_env();
  let args: Vec<String> = env::args().collect();
  let args: Vec<&str> = args.iter().map(String::as_str).collect();
  let parsed = utils::parse_args(&args);
  let rest: Vec<&str> = parsed.rest.iter().map(String::as_str).collect();
  let dir = utils::get_download_dir(&rest)?;

  if parsed.revert {
    let moved = utils::revert_last(&dir)?;
    let files = noun(moved, "file", "files");
    let line = format!("Reverted — moved {files} back.");
    println!("{}", colors.paint(utils::Color::Summary, &line));
    return Ok(());
  }

  let by_date = parsed.by_date;

  if parsed.dry_run {
    let header = colors.paint(
      utils::Color::Previewed,
      &format!("Dry run — previewing how {dir} would be organized."),
    );
    println!("{header}");
    let s = utils::organize(&dir, true, by_date)?;
    if s.moved > 0 {
      print_tree(colors, &s.destinations, utils::Color::Previewed);
    }
    print_summary(colors, true, &s);
    return Ok(());
  }

  utils::set_write_permissions(&dir)?;
  println!(
    "{}",
    colors.paint(utils::Color::Category, &format!("Organizing {dir}."))
  );
  let mut s = utils::organize(&dir, false, by_date)?;
  if s.moved > 0 {
    print_tree(colors, &s.destinations, utils::Color::Moved);
  }
  print_summary(colors, false, &s);

  // Persist this run so `--revert` can undo it later. Only runs that actually
  // moved something are worth recording.
  if !s.moves.is_empty() {
    let timestamp = chrono::Utc::now()
      .format("%Y-%m-%dT%H:%M:%S%.3fZ")
      .to_string();
    let moves = std::mem::take(&mut s.moves);
    utils::append_run(&utils::RunRecord {
      dir,
      by_date,
      timestamp,
      moves,
    })?;
  }

  Ok(())
}

/// Print the destinations as a coloured tree.
fn print_tree(
  colors: utils::Colors,
  destinations: &[String],
  color: utils::Color,
) {
  let refs: Vec<&str> = destinations.iter().map(String::as_str).collect();
  let rendered = utils::render_tree(&refs);
  for line in rendered.lines() {
    println!("  {}", colors.paint(color, line));
  }
}

fn print_summary(
  colors: utils::Colors,
  dry_run: bool,
  s: &utils::OrganizeSummary,
) {
  let files = noun(s.moved, "file", "files");
  let entries = noun(s.skipped, "entry", "entries");
  println!();
  let line = if dry_run {
    format!("Dry run — would move {files} and leave {entries} untouched.")
  } else {
    format!("Done — moved {files} and left {entries} untouched.")
  };
  println!("{}", colors.paint(utils::Color::Summary, &line));
}

/// Return `count` + noun with correct pluralisation (`1 file`, `2 files`).
fn noun(count: usize, singular: &str, plural: &str) -> String {
  if count == 1 {
    format!("{count} {singular}")
  } else {
    format!("{count} {plural}")
  }
}
