<div align="center">
  <img src="doc/logo.png" alt="Logo" width="200" height="200">
</div>
# skp 🪄  
A minimal, lightning-fast CLI to **skip between your favorite project directories**.

> _Bookmarks for your terminal_, powered by Rust.

---

## 🚀 Features

- 📁 Save folders as favorites with a single command
- ⚡ Instantly jump to saved paths via CLI or alias
- 🧭 Intuitive commands for managing saved locations
- 🦀 Built in Rust — small, fast, and reliable

---

## 📦 Installation

### Using Cargo

```bash
cargo install skp
```

### Prebuilt Binaries (coming soon)

Download from the [Releases](https://github.com/yourusername/skp/releases) page.

---

## 🛠️ Usage

### Add the current directory as a favorite

```bash
skp add my-project
```

### Jump to a saved path

```bash
skp go my-project
```

### List saved paths

```bash
skp list
```

### Remove a saved path

```bash
skp rm my-project
```

---

## 🧩 Shell Integration

To integrate `skp` with your shell for seamless `cd` behavior:

### Bash / Zsh

```bash
function skp() {
  cd "$(command skp go "$@")"
}
```

Add this to your `.bashrc` or `.zshrc`.

### Fish

```fish
function skp
  cd (skp go $argv)
end
```

Add this to your `config.fish`.

---

## 📁 Storage

Saved paths are stored in:

```
~/.config/skp/paths.json
```

This is a plain JSON file — easy to back up or edit manually if needed.

---

## 🔮 Roadmap

- [x] Core commands: `add`, `go`, `list`, `rm`
- [ ] Fuzzy matching
- [ ] Shell autocomplete
- [ ] Group/tag support
- [ ] Configurable storage backend
- [ ] Sync via Git or remote storage (optional)

---

## 🧑‍💻 Author

Built with ❤️ and 🦀 by [Your Name](https://github.com/yourusername)

