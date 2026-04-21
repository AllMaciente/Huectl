# huectl 🎨

A fast, modular theme manager for Linux — manual control, template engine, XDG-compliant.

Inspired by [pywal](https://github.com/dylanaraps/pywal), but without image-based color extraction: **you define your themes, you own your colors.**

```
huectl apply nord
  ✓ Loaded theme 'nord'
  ✓ Colors validated
  ✓ Cache updated
    → waybar.css.tpl     → ~/.cache/huectl/waybar.css
    → alacritty.toml.tpl → ~/.cache/huectl/alacritty.toml
    → dunst.conf.tpl     → ~/.cache/huectl/dunst.conf
  ✓ Processed 3 template(s)
  ✓ Wallpaper applied

✓ Theme 'nord' applied successfully!
```

---

## Features

- **19-color palette** — 16 ANSI + `background`, `foreground`, `cursor`
- **Template engine** — `{{color4}}`, `{{background}}`, any key in your `.tpl` files
- **JSON + TOML** theme formats
- **Wallpaper management** — auto-detect by theme name, supports `swaybg`, `feh`, `nitrogen`, `xwallpaper`
- **Persistent cache** — `colors.json` + `current_theme.json` in `~/.cache/huectl/`
- **Hook system** — `post_apply.sh` runs after every apply
- **XDG-compliant** — respects `~/.config/` and `~/.cache/` standards
- **Zero runtime deps** — single static binary


---

## Usage

```bash
huectl apply <theme>            # Apply a theme
huectl list                     # List all themes
huectl colors                   # Show current palette with swatches
huectl preview <theme>          # Preview without applying
huectl current                  # Show active theme
huectl wallpaper get            # Get current wallpaper path
huectl wallpaper set <path>     # Set wallpaper manually

# Flags for apply:
huectl apply nord --no-wallpaper   # Skip wallpaper
huectl apply nord --no-templates   # Skip template processing
```

---

## Directory Structure

```
~/.config/huectl/
├── themes/
│   ├── nord.json
│   ├── gruvbox.json
│   └── catppuccin.toml       ← TOML also supported
├── templates/
│   ├── alacritty.toml.tpl
│   ├── waybar.css.tpl
│   └── dunst.conf.tpl
├── wallpapers/
│   ├── nord.png              ← matched by theme name
│   └── gruvbox.jpg
└── hooks/
    └── post_apply.sh

~/.cache/huectl/
├── current_theme.json        ← full theme state
├── colors.json               ← flat key→hex map
├── wallpaper                 ← current wallpaper path
├── alacritty.toml            ← rendered templates
├── waybar.css
└── dunst.conf
```

---

## Theme Format

**JSON (`~/.config/huectl/themes/nord.json`):**
```json
{
  "name": "nord",
  "colors": {
    "color0":  "#2e3440",
    "color1":  "#bf616a",
    "color2":  "#a3be8c",
    "color3":  "#ebcb8b",
    "color4":  "#81a1c1",
    "color5":  "#b48ead",
    "color6":  "#88c0d0",
    "color7":  "#e5e9f0",
    "color8":  "#4c566a",
    "color9":  "#bf616a",
    "color10": "#a3be8c",
    "color11": "#ebcb8b",
    "color12": "#81a1c1",
    "color13": "#b48ead",
    "color14": "#8fbcbb",
    "color15": "#eceff4",
    "background": "#2e3440",
    "foreground": "#d8dee9",
    "cursor":     "#d8dee9"
  }
}
```

**TOML (`~/.config/huectl/themes/catppuccin.toml`):**
```toml
name = "catppuccin"

[colors]
color0  = "#1e1e2e"
color1  = "#f38ba8"
# ...
background = "#1e1e2e"
foreground = "#cdd6f4"
cursor     = "#f5e0dc"
```

---

## Template System

Templates live in `~/.config/huectl/templates/` with a `.tpl` extension.
On `huectl apply`, each `.tpl` is rendered and saved to `~/.cache/huectl/` with the `.tpl` extension removed.

**Available placeholders:**

| Placeholder | Description |
|---|---|
| `{{color0}}` … `{{color15}}` | ANSI palette (0–15) |
| `{{background}}` | Background color |
| `{{foreground}}` | Foreground color |
| `{{cursor}}` | Cursor color |
| `{{color0_strip}}` … | Same as above but without `#` prefix |
| `{{theme_name}}` | Name of the active theme |

**Example — `alacritty.toml.tpl`:**
```toml
[colors.primary]
background = "{{background}}"
foreground = "{{foreground}}"

[colors.normal]
black   = "{{color0}}"
red     = "{{color1}}"
green   = "{{color2}}"
yellow  = "{{color3}}"
blue    = "{{color4}}"
magenta = "{{color5}}"
cyan    = "{{color6}}"
white   = "{{color7}}"
```

**Example — `waybar.css.tpl`:**
```css
window#waybar {
    background: {{background}};
    color: {{foreground}};
}
.warning  { color: {{color3}}; }
.critical { color: {{color1}}; }
```

---

## Wallpapers

Place wallpaper files in `~/.config/huectl/wallpapers/` named after the theme:

```
wallpapers/
├── nord.png
├── gruvbox.jpg
└── catppuccin.webp
```

When you run `huectl apply nord`, it will automatically find `nord.png` (or `.jpg`, `.webp`, `.gif`, `.bmp`).

Supported setters (tried in order): `swaybg`, `feh`, `nitrogen`, `gsettings`, `xwallpaper`, `hsetroot`.

---

## Hooks

**`~/.config/huectl/hooks/post_apply.sh`** runs after every successful `huectl apply`.  
The theme name is passed as `$1`.

```bash
#!/usr/bin/env bash
THEME="$1"
CACHE="$HOME/.cache/huectl"

# Reload waybar
killall -SIGUSR2 waybar 2>/dev/null || true

# Symlink alacritty colors
ln -sf "$CACHE/alacritty.toml" "$HOME/.config/alacritty/colors.toml"

# Reload dunst
killall dunst 2>/dev/null
dunst &
```

---

## Cache Files

Scripts and other tools can consume the cache directly:

```bash
# Get current wallpaper
cat ~/.cache/huectl/wallpaper

# Get a specific color
jq -r '.color4' ~/.cache/huectl/colors.json

# Get the active theme name
jq -r '.name' ~/.cache/huectl/current_theme.json

# Use in a shell script
BG=$(jq -r '.background' ~/.cache/huectl/colors.json)
```

---

## Architecture

```
src/
├── main.rs       — Entry point, CLI dispatch
├── cli.rs        — All command implementations + color swatch rendering
├── theme.rs      — Theme struct, JSON/TOML loading, hex validation
├── template.rs   — {{placeholder}} template engine
├── wallpaper.rs  — Wallpaper finder + multi-setter support
├── cache.rs      — colors.json + current_theme.json persistence
└── utils.rs      — XDG paths (config_dir, cache_dir), hook runner
```

---

## License

MIT