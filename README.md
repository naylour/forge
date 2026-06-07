# forge

Утилита для первичной настройки проектов на базе [ingot](https://github.com/naylour/ingot).

После клонирования ingot-проекта запусти forge один раз — он установит git-хуки и сгенерирует age-ключ для доступа к секретам.

## Установка

```bash
mise use github:naylour/forge
```

## Использование

```bash
forge
```

Или через mise:

```bash
mise run setup
```

forge выполнит:

1. Установку git-хуков через `hk`
2. Генерацию age-ключа по пути `~/.config/age/ingot-secret.txt` если его ещё нет
3. Вывод публичного ключа — отправь его владельцу репозитория чтобы получить доступ к секретам

## Требования

В PATH должны быть доступны:

- [hk](https://github.com/jdx/hk)
- [age](https://github.com/FiloSottile/age)

Оба обычно устанавливаются через `mise.toml` проекта.
