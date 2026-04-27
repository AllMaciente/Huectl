# Hooks

Hooks são scripts executáveis que rodam automaticamente após a aplicação de um tema.

## Estrutura

Coloque scripts executáveis em `~/.config/huectl/hooks/`:

```
~/.config/huectl/hooks/
├── 01-hyprland.sh
├── 02-waybar.sh
└── 03-dunst.sh
```

## Execução

- Todos os arquivos executáveis na pasta são executados em **ordem alfabética**
- Arquivos não-exeutáveis são ignorados silenciosamente
- Subdiretórios não são processados
- Se um hook falha, os próximos continuam executando

## Variáveis de Ambiente

Cada hook recebe variáveis de ambiente prefixadas com `HUECTL_`:

```bash
#!/usr/bin/env bash
# $1 = nome do tema

# Cores
echo "$HUECTL_color4"
echo "$HUECTL_background"
echo "$HUECTL_foreground"

# Cores sem # (stripped)
echo "$HUECTL_color4_strip"

# Meta
echo "$HUECTL_theme_name"

# Custom (suas variáveis)
echo "$HUECTL_custom_font"
echo "$HUECTL_custom_gap"
echo "$HUECTL_custom_blur"
```

### Exemplo: Hook Hyprland

```bash
#!/usr/bin/env bash
# ~/.config/huectl/hooks/01-hyprland.sh

hyprctl keyword general:col.active_border "rgb($HUECTL_color4_strip)"
hyprctl keyword general:col.inactive_border "rgb($HUECTL_color8_strip)"
hyprctl keyword decoration:blur:enabled "$HUECTL_custom_blur"
hyprctl keyword general:gaps_in "$HUECTL_custom_gap"
hyprctl keyword general:gaps_out "$HUECTL_custom_gap"
hyprctl keyword general:border_size "$HUECTL_custom_border_width"
```

### Exemplo: Hook Waybar

```bash
#!/usr/bin/env bash
# ~/.config/huectl/hooks/02-waybar.sh

# Regenera config usando template
huectl render-waybar

# Recarrega waybar
killall -SIGUSR2 waybar 2>/dev/null || true
```

### Exemplo: Hook Dunst

```bash
#!/usr/bin/env bash
# ~/.config/huectl/hooks/03-dunst.sh

# Regenera config
huectl render-dunst

# Reinicia dunst
killall dunst 2>/dev/null || true
dunst &
```

## Subcomando Var

Para obter um valor específico do tema ativo:

```bash
huectl var <chave>
```

Isso imprime o valor **sem newline final**, útil para subshells:

```bash
# Em um hook:
FONT=$(huectl var custom_font)
GAP=$(huectl var custom_gap)
BG=$(huectl var background)

# Diretamente no terminal:
echo "Meu fundo é $(huectl var background)"
```

### Chaves Disponíveis

| Chave | Exemplo |
|-------|--------|
| `color0` a `color15` | `#2e3440` |
| `background` | `#2e3440` |
| `foreground` | `#d8dee9` |
| `cursor` | `#d8dee9` |
| `theme_name` | `nord` |
| `custom_*` | qualquer chave em `custom` |

## Erros

Se um hook falha:
- Um aviso é impresso com o nome do hook e código de saída
- Os próximos hooks continuam executando
- O tema ainda é aplicado com sucesso