# Templates

Templates permite renderizar arquivos de configuração dinamicamente baseados no tema ativo.

## Como Funciona

1. Crie arquivos com extensão `.tpl` em `~/.config/huectl/templates/`
2. Cada `huectl apply` processa todos os templates
3. A saída vai para `~/.cache/huectl/` sem a extensão `.tpl`

Exemplo:

```
~/.config/huectl/templates/
├── alacritty.toml.tpl    →  ~/.cache/huectl/alacritty.toml
├── waybar.css.tpl       →  ~/.cache/huectl/waybar.css
└── dunst.conf.tpl       →  ~/.cache/huectl/dunst.conf
```

## Variáveis Disponíveis

### Cores (padrão)

| Variável | Descrição |
|---------|-----------|
| `{{color0}}` a `{{color15}}` | Paleta ANSI (16 cores) |
| `{{background}}` | Cor de fundo |
| `{{foreground}}` | Cor do texto |
| `{{cursor}}` | Cor do cursor |

### Cores sem `#`

Para valores sem o prefixo `#` (útil em RGB decimal):

| Variável | Descrição |
|---------|-----------|
| `{{color0_strip}}` a `{{color15_strip}}` | Paleta sem `#` |
| `{{background_strip}}` | Fundo sem `#` |
| `{{foreground_strip}}` | Texto sem `#` |
| `{{cursor_strip}}` | Cursor sem `#` |

### Meta

| Variável | Descrição |
|---------|-----------|
| `{{theme_name}}` | Nome do tema ativo |

### Custom (variáveis personalizadas)

Se o tema tiver a seção `custom`, use `{{custom_chave}}`:

```json
{
  "custom": {
    "opacity": "0.92",
    "font": "JetBrains Mono"
  }
}
```

No template:
```toml
opacity = {{custom_opacity}}
font = "{{custom_font}}"
```

### Exemplo: Hyprland

```toml
# ~/.config/huectl/templates/hyprland.conf.tpl
general {
    gaps_in = {{custom_gap}}
    gaps_out = {{custom_gap}}
    border_size = {{custom_border_width}}
}

decoration {
    blur {
        enabled = {{custom_blur}}
    }
    col.active_border = rgb({{color4_strip}})
    col.inactive_border = rgb({{color8_strip}})
}
```

### Exemplo: Waybar CSS

```css
/* ~/.config/huectl/templates/waybar.css.tpl */
window#waybar {
    background: {{background}};
    color: {{foreground}};
    font-family: {{custom_font}};
    font-size: {{custom_font_size}}px;
}

#workspaces {
    background: {{color0}};
}

#workspaces button {
    color: {{color4}};
}
```

### Exemplo: Dunst

```conf
# ~/.config/huectl/templates/dunst.conf.tpl
[global]
    opacity = {{custom_opacity}}

[urgency_low]
    background = "{{background}}"
    foreground = "{{foreground}}"

[urgency_normal]
    background = "{{background}}"
    foreground = "{{foreground}}"

[urgency_critical]
    background = "{{color1}}"
    foreground = "{{color15}}"
```

## Skip Templates

Para aplicar um tema sem processar templates:

```bash
huectl apply nord --no-templates
```