# catfetch 🐾
<img width="807" height="337" alt="image" src="https://github.com/user-attachments/assets/ec4df03b-aa83-401c-8fee-a9ad7ee9e549" />

A minimalist, lightweight CLI system information fetch utility written in Rust. It displays clean system stats alongside random cat facts and pixel-art logos. Optimized to perfectly fit narrow terminal screens.

> **Note:** Due to temporary closure of new AUR account registrations, please use the manual or local installation methods below to get `catfetch` on your system.

---

## Installation for End Users

### Method 1: Local PKGBUILD (Recommended for Arch Linux)
You don't need an active AUR connection to build this package like a native Arch utility. Just clone the repo and use `makepkg`:

```bash
# 1. Clone this repository
git clone https://github.com/ArchEnjoyer333/catfetch.git
cd catfetch

# 2. Build and install via pacman automatically
makepkg -si
```

### Method 2: Manual Installation (Any Linux / Rust users)
If you prefer installing via Cargo into your user binaries folder:

```bash
# 1. Clone and enter the directory
git clone https://github.com/ArchEnjoyer333/catfetch.git
cd catfetch

# 2. Compile and install into ~/.cargo/bin/
cargo install --path . --force
```
*Make sure `~/.cargo/bin` is added to your shell's `$PATH` (e.g., run `fish_add_path ~/.cargo/bin` if you use Fish shell).*

---

## Data & Asset Paths (FHS Compliant)

When installed via **Method 1 (PKGBUILD)**, `pacman` places resources globally into:
- Binary: `/usr/bin/catfetch`
- Resources: `/usr/share/catfetch/`

When running via **Method 2 (Cargo)**, make sure to copy your assets so the binary can find them:
```bash
mkdir -p ~/.config/catfetch
cp cats.toml ~/.config/catfetch/
cp -r logo ~/.config/catfetch/
```

---

## Customization
You can easily customize facts or add your own pixel-art images without modifying the source code. The utility checks your home directory first:

1. Create a local configuration folder: `~/.config/catfetch/logo/`
2. Drop your custom `.png` images inside the `logo` directory.
3. Edit `~/.config/catfetch/cats.toml` and fill the `facts` array with your own quotes or text lines.

```toml
facts = [
    "Your custom cat fact goes here.",
    "Another interesting line of text."
]
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
