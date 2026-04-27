# Wallpapers

Wallpapers são imagens de fundo automaticamente aplicadas junto com o tema.

## Estrutura

Coloque imagens em `~/.config/huectl/wallpapers/` nomeadas pelo tema:

```
~/.config/huectl/wallpapers/
├── nord.png
├── gruvbox.jpg
├── catppuccin.webp
└── dracula.gif
```

## Aplicando

Quando você executar `huectl apply nord`, ele procurará automaticamente por `nord.png` (ou `.jpg`, `.webp`, `.gif`, `.bmp`).

```bash
huectl apply nord
# → aplica automáticamente nord.png como wallpaper
```

## Wallpapers Suportados

O huectl tenta vários programas para definir o wallpaper (em ordem):

| Programa | Args |
|----------|------|
| awww | `img` |
| swaybg | `--image` |
| feh | `--bg-scale` |
| nitrogen | `--set-scaled --save` |
| gsettings | `set org.gnome.desktop.background picture-uri` |
| xwallpaper | `--zoom` |
| hsetroot | `-fill` |

O primeiro encontrado no sistema é usado.

## Skip Wallpaper

Para aplicar um tema sem alterar o wallpaper:

```bash
huectl apply nord --no-wallpaper
```

## Obtendo Wallpaper Atual

```bash
huectl wallpaper get
# → ~/.config/huectl/wallpapers/nord.png
```

## Definindo Manualmente

```bash
huectl wallpaper set ~/Pictures/my-bg.png
```

## Cache

O caminho do wallpaper atual é salvo em `~/.cache/huectl/wallpaper`.