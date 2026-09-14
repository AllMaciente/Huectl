# Wallpapers

Wallpapers são imagens de fundo automaticamente aplicadas junto com o tema.

## Estrutura

O huectl suporta duas formas de organizar wallpapers:

### 1. Arquivo único (compatível com versões anteriores)

Coloque uma imagem em `~/.config/huectl/wallpapers/` nomeada pelo tema:

```
~/.config/huectl/wallpapers/
├── nord.png
├── gruvbox.jpg
├── catppuccin.webp
└── dracula.gif
```

### 2. Coleção por tema (novo)

Crie uma pasta com o nome do tema contendo múltiplos wallpapers:

```
~/.config/huectl/wallpapers/
├── nord/
│   ├── morning.png
│   ├── evening.jpg
│   └── night.webp
├── gruvbox/
│   ├── light.png
│   └── dark.jpg
└── catppuccin.webp
```

Se **ambos** existirem (arquivo `nord.png` E pasta `nord/`), o arquivo único continua sendo o default do `huectl apply`, e a pasta fica disponível para seleção nomeada via `huectl wallpaper set`.

## Aplicando

Quando você executar `huectl apply nord`, ele procurará automaticamente por `nord.png` (ou `.jpg`, `.webp`, `.gif`, `.bmp`, `.tiff`). Se não existir arquivo único mas existir a pasta `nord/`, o primeiro arquivo em ordem alfabética dentro da pasta será usado como default.

```bash
huectl apply nord
# → aplica automaticamente o wallpaper do tema
```

Para aplicar um tema sem alterar o wallpaper:

```bash
huectl apply nord --no-wallpaper
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

Extensões suportadas: `.jpg`, `.jpeg`, `.png`, `.webp`, `.gif`, `.bmp`, `.tiff` (case-insensitive).

## Listando wallpapers de um tema

```bash
# Lista wallpapers do tema ativo (útil para rofi/dmenu)
huectl wallpaper list

# Lista wallpapers de um tema específico
huectl wallpaper list nord
```

A saída é "crua" (um nome por linha, sem cores/decoração), pensada para ser consumida por seletores como `rofi -dmenu`:

```bash
huectl wallpaper list | rofi -dmenu | xargs huectl wallpaper set
```

## Definindo wallpaper

### Por caminho de arquivo (compatível)
```bash
huectl wallpaper set ~/Pictures/my-bg.png
```

### Por nome (novo - resolve dentro da pasta do tema ativo)
```bash
# Se o tema ativo for 'nord' e existir ~/.config/huectl/wallpapers/nord/morning.png
huectl wallpaper set morning
```

## Obtendo wallpaper atual

```bash
huectl wallpaper get
# → /caminho/absoluto/do/wallpaper/atual
```

## Cache

O caminho do wallpaper atual é salvo em `~/.cache/huectl/wallpaper`.

O comando `huectl reload` reaplica o wallpaper salvo no cache, funcionando independentemente de ter vindo de arquivo único ou de dentro de uma pasta de coleção.

## Comportamento do apply com pasta de coleção

- Se existe arquivo único `tema.ext` → usa o arquivo único (comportamento atual, sem mudanças)
- Se existe apenas pasta `tema/` → usa o primeiro arquivo em ordem alfabética dentro da pasta
- Se existe ambos → arquivo único tem prioridade no `apply`; pasta disponível para `wallpaper set <nome>`
- Se não existe nada → avisa que não há wallpaper, mas não falha o apply