# Temas

Temas definem a identidade visual do seu ambiente Linux — cores, plano de fundo e variáveis customizadas.

## Estrutura de Arquivo

Um tema é um arquivo JSON ou TOML em `~/.config/huectl/themes/`.

### JSON

```json
{
  "name": "nord",
  "colors": {
    "color0": "#2e3440",
    "color1": "#bf616a",
    "color2": "#a3be8c",
    "color3": "#ebcb8b",
    "color4": "#81a1c1",
    "color5": "#b48ead",
    "color6": "#88c0d0",
    "color7": "#e5e9f0",
    "color8": "#4c566a",
    "color9": "#bf616a",
    "color10": "#a3be8c",
    "color11": "#ebcb8b",
    "color12": "#81a1c1",
    "color13": "#b48ead",
    "color14": "#8fbcbb",
    "color15": "#eceff4",
    "background": "#2e3440",
    "foreground": "#d8dee9",
    "cursor": "#d8dee9"
  },
  "custom": {
    "opacity": "0.92",
    "font": "JetBrains Mono",
    "font_size": "13",
    "gap": "8",
    "border_width": "2",
    "blur": "true"
  }
}
```

### TOML

```toml
name = "nord"

[colors]
color0  = "#2e3440"
color1  = "#bf616a"
# ... demais cores

[custom]
opacity = "0.92"
font = "JetBrains Mono"
font_size = "13"
gap = "8"
border_width = "2"
blur = "true"
```

## Campos

### `name` (obrigatório)

Nome do tema. Deve corresponder ao nome do arquivo sem extensão.

### `colors` (obrigatório)

Paleta de 19 cores:

- `color0` a `color15` — paleta ANSI (16 cores)
- `background` — cor de fundo
- `foreground` — cor do texto
- `cursor` — cor do cursor

Todas as cores devem usar formato hexadecimal `#RRGGBB`.

### `custom` (opcional)

Variáveis arbitrárias de chave/valor. Disponíveis nos templates como `{{custom_chave}}` e nos hooks como `$HUECTL_custom_chave`.

## listando Temas

```bash
huectl list
```

Mostra todos os temas disponíveis, indicando qual está ativo no momento.

## Aplicando um Tema

```bash
huectl apply nord
```

## Preview

Visualize um tema sem aplicá-lo:

```bash
huectl preview nord
```

## Verificando Tema Atual

```bash
huectl current
```

Mostra o tema ativo e seu wallpaper (se houver).