# image_ffi_project

CLI-приложение для обработки изображений с поддержкой динамически подключаемых плагинов через FFI.

## Сборка
```bash
cargo build
```

## Использование

```bash
# blur plugin
cargo run --package image_processor --bin image_processor -- -i "examples/sample.png" -o "examples/blur.png" -p blur_plugin --params "examples/params.json"

# mirror plugin
cargo run --package image_processor --bin image_processor -- -i "examples/sample.png" -o "examples/mirror.png" -p mirror_plugin --params "examples/params.json"
```

## Плагины

### blur_plugin
Размытие изображения — принимает файл с параметрами в виде json: radius (радиус размытия) и iterations (количество итераций)
```json
{
  "radius": 5,
  "iterations": 2
}
```

### mirror_plugin
Зеркальный разворот изображения — принимает файл с bool параметрами в виде json: horizontal (отразить по горизонтали) и vertical (отразить по вертикали)
```json
{
  "horizontal": true,
  "vertical": true
}
```
## Структура проекта

```

image_ffi_project/
├── Cargo.toml                  # Workspace
├── image_processor/
│   ├── Cargo.toml              # Image processor package
│   └── src/
│       ├── main.rs             # бинарник
│       ├── lib.rs              # подключение плагина и обработка
│       ├── error.rs            # ошибки
│       ├── args.rs             # аргументы
│       └── plugin_loader.rs    # подключение плагина
├── mirror_plugin/              # Mirror plugin package
│       ├── Cargo.toml
│       ├── src/lib.rs          # реализация плагина
│       └── src/params.rs       # структура параметров плагина
├── blur_plugin/                # Blur plugin package
│       ├── Cargo.toml
│       ├── src/lib.rs          # реализация плагина
│       └── src/params.rs       # структура параметров плагина
└── README.md
```
