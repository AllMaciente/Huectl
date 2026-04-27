# Huectl Documentation

## Índice

- [Temas](themes.md) — Definição de cores e variáveis customizadas
- [Templates](templates.md) — Renderização dinâmica de arquivos de configuração
- [Hooks](hooks.md) — Scripts executados após aplicar um tema
- [Wallpapers](wallpapers.md) — Imagens de fundo

## Instalação

```bash
cargo build --release
cp target/release/huectl ~/.local/bin/
```

## Estrutura de Diretórios

```
~/.config/huectl/
├── themes/        # Arquivos JSON/TOML com definição de temas
├── templates/    # Arquivos .tpl para renderização
├── wallpapers/   # Imagens nomeadas por tema
└── hooks/       # Scripts executados após apply

~/.cache/huectl/
├── current_theme.json  # Tema ativo completo (inclui custom)
├── colors.json     # Mapa flat de cores (para scripts externos)
├── wallpaper     # Caminho do wallpaper atual
└── [arquivos renderizados]
```

## Comandos

```bash
huectl apply <theme>        # Aplica um tema
huectl list             # Lista todos os temas
huectl colors           # Mostra paleta com swatches
huectl preview <theme> # Visualiza sem aplicar
huectl current         # Mostra tema ativo
huectl wallpaper get   # Obtém wallpaper atual
huectl wallpaper set <path>  # Define wallpaper
huectl var <chave>   # Obtém valor de variável
huectl reload         # Recarrega tema atual
```

## Flags de Apply

```bash
huectl apply nord --no-wallpaper  # Pula wallpaper
huectl apply nord --no-templates # Pula templates
```