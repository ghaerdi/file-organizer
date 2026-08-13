# Changelog

## 1.0.0

### New Features

- **Organize any directory by file extension.** `file-organizer` moves files
  into category subfolders based on their extension. Run with no argument to
  organize the OS Downloads directory, or pass a directory to target any
  folder.
- **11 categories by extension** — `Text`, `Image`, `Video`, `Audio`,
  `Compressed`, `Executable`, `Code`, `Data`, `Disk`, `Font`, and `Other`.
  Category folders are created lazily: only categories that actually receive
  files are materialised, so a run that touches only `Text` and `Image` leaves
  the other categories uncreated.
- **`--dry-run`** — preview how the directory would be organized as a
  box-drawing tree without touching any file.
- **`--by-date`** — group files into `Category/YYYY/MM/` subfolders inside each
  category by modification date.
- **`--revert`** — undo the most recent organize run on the target directory,
  moving every file back where it came from and pruning emptied category/date
  branches. Runs are recorded in a JSON register at
  `~/.local/state/file-organizer/register.log` (honouring `$XDG_STATE_HOME`).
  Multiple `--revert` calls undo runs in reverse order.
- **Collision-safe moves** — when a file's name already exists in a category,
  the incoming file is renamed with a numeric suffix (e.g. `report-1.pdf`)
  instead of overwriting the original.
- **Symlinks are left untouched.**
- **Runs on multi-platform** — files are organized correctly on both Unix and
  Windows (directories are never moved, only files).
- **Colour-coded terminal output** — category directories in bold blue, file
  leaves in bold green (moved) or bold yellow (previewed), dim gray
  `├──`/`└──` tree connectors, and a muted-cyan summary line. Colours are
  terminal-gated, so output falls back to plain text when piped.
- **`--help` and `--version`** provided by [clap](https://clap.rs), which also
  produces clear errors for unknown flags and extra arguments.
- **Robust error handling** — no panics; typed errors are surfaced as clear
  messages instead of unwinding.
- **Stable, deterministic output** — the tree and summary render in a fixed
  order regardless of filesystem iteration order.

### Documentation

- `README.md` covers installation, usage, flags, the category taxonomy, and the
  revert register.
- `install.sh` installs the binary on Linux and macOS (with sudo support).
- A `devenv` development environment configures `rustfmt`, a strict `clippy`
  lint set, and pre-commit hooks.

### Tests

- Unit tests cover category detection, directory resolution, file moves (flat
  and `--by-date` paths, collisions, dry-run), symlink skipping, the revert
  register, and tree rendering. Filesystem tests use a shared RAII `TempDir`
  helper for deterministic, isolated runs.
- CI builds the project and runs the full test suite on push.
