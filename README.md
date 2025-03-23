# image_resize

[TOC]

Командная утилита для изменения размера изображений

### Использование

Доступные команды получаются через

```bash
image_resizer --help 
```

или

```bash
image_resizer -h
```

параметры команды доступны через короткую справку

```bash
image_resizer {command} -h
```

или полную справку

```bash
image_resizer {command} --help
```

### Доступные переменные среды:

`RUST_LOG` - уровень логов, по умолчанию `info`. Доступные
значения  https://docs.rs/env_logger/latest/env_logger/#enabling-logging

### Сборка из исходного кода

1) [Установка Rust](https://www.rust-lang.org/tools/install) (после комлиляции rust больше не нужен)
2) Установка библиотеки для работы с SSL

Linux

```bash
sudo apt install openssl
```

Mac

```bash
sudo brew install openssl
```

3) Клонирование и сборка из исходников

```bash
git clone repo

cd ./image_resizer
```

```bash
cargo build --release
```

После сборки исполняемый файл - `./target/release/image_resizer` нужно скопировать, все остальное можно удалять