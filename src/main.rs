use std::env;

mod error;
mod globals;
mod utils;

#[cfg(test)]
mod test_utils;

fn main() -> anyhow::Result<()> {
  let args: Vec<String> = env::args().collect();
  let args: Vec<&str> = args.iter().map(String::as_str).collect();
  let parsed = utils::parse_args(&args);
  let rest: Vec<&str> = parsed.rest.iter().map(String::as_str).collect();
  let dir = utils::get_download_dir(&rest)?;
  let by_date = parsed.by_date;

  if parsed.dry_run {
    println!("[dry-run] previewing organization of {dir}");
    let s = utils::organize(&dir, true, by_date)?;
    print_summary(&s);
    return Ok(());
  }

  utils::set_write_permissions(&dir)?;
  utils::create_dirs(&dir)?;
  let s = utils::organize(&dir, false, by_date)?;
  print_summary(&s);

  Ok(())
}

fn print_summary(s: &utils::OrganizeSummary) {
  println!();
  println!(
    "moved {} file(s), skipped {} entr(y/ies)",
    s.moved, s.skipped
  );
}
