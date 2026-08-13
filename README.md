## Organizer

### Description

Organizer is a Rust script to organize the Download directory in diferent directories depending of the file extension.
<hr>

### Install

1. Install rustup.
2. Download this repository.
```console
git clone https://github.com/ghaerdi/file-organizer.git
```
3. Run `install.sh` in the directory.

### How to use

```console
organizer
```
You can also pass a directory as argument using relative path if you want specify the folder to organize.

```console
organizer [--dry-run] [--by-date] [DIR]
```

| Flag | Effect |
|------|--------|
| `--dry-run` | Preview the moves without touching any file |
| `--by-date` | Group files into `Category/YYYY/MM/` inside their category by modification date |

Files are grouped into these categories by extension: `Text`, `Image`, `Video`,
`Audio`, `Compressed`, `Executable`, `Code`, `Data`, `Disk`, `Font`, and `Other`.

When a file's name already exists in a category, the incoming file is renamed
with a numeric suffix (e.g. `report-1.pdf`) instead of overwriting the original.
Symlinks are left untouched.

[![Example](./example.gif)](./example.gif)

Note: The script require the `$HOME/Downloads` directory when no command is passed.
