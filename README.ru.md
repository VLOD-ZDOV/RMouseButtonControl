# RMouseButtonControl

[English version / Английская версия](README.md)

![RMouseButtonControl](https://github.com/user-attachments/assets/a42266b8-dead-4dfd-b894-2deba1e9aa7e)

**RMouseButtonControl** — высокопроизводительная утилита для переназначения кнопок мыши и симуляции макросов. Проект сочетает фоновый сервис на чистом безопасном Rust, быстрый нативный интерфейс на Qt6 по умолчанию и возможность запуска оригинального Fluent Avalonia C# интерфейса.

## Ключевые возможности и архитектура

- **Ядро на чистом Rust:**
  - Написано с `#![forbid(unsafe_code)]` — математическая гарантия отсутствия утечек памяти и сбоев.
  - Прямой драйвер виртуального устройства ядра Linux `/dev/uinput` — субмиллисекундный инпут-лаг, полная совместимость с Wayland, X11 и играми с античитом.
  - Userspace fallback для сред с ограниченным доступом к `/dev/uinput`.
  - Потребление ОЗУ демона — всего ~10 МБ.
- **Два интерфейса на выбор:**
  - **Qt6 нативный GUI (По умолчанию):** Мгновенный запуск (<20 мс), малое потребление памяти (~20 МБ), современное оформление.
  - **Avalonia C# GUI (Опционально):** Оригинальный интерфейс в стиле Fluent.
  - Переключение между интерфейсами доступно в один клик через контекстное меню трея или окно общих настроек.
- **Интеграция с рабочим столом и системным треем:**
  - Стандартное меню StatusNotifierItem для системного трея KDE Plasma, GNOME и FreeDesktop.
  - Быстрое переключение профилей, пауза/возобновление, управление автозапуском и горячая перезагрузка базы.
- **Гибкая симуляция клавиш и макросов:** Поддержка триггеров на нажатие/отпускание, комбинаций клавиш, залипания и автоповтора с настраиваемой задержкой.
- **Автоматическое распознавание окон и процессов:**
  - Переключение профилей по активному окну.
  - Полная поддержка нативных приложений Linux, а также игр под Wine и Proton.
- **5 тем оформления:**
  - Системная (KDE Plasma / Breeze)
  - Тёмная (Modern Dark)
  - Светлая (Clean White)
  - Чёрная (AMOLED Black `#000000`)
  - Fusion
- **Локализация:**
  - Поддержка 8 языков: русский, английский, немецкий, испанский, французский, украинский, португальский и китайский.

## Быстрая установка

### Linux (curl)
```bash
curl -sSL https://raw.githubusercontent.com/VLOD-ZDOV/RMouseButtonControl/master/scripts/install.sh | bash
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/VLOD-ZDOV/RMouseButtonControl/master/scripts/install.ps1 | iex
```

### Windows (winget)
```powershell
winget install --manifest winget/VLOD-ZDOV.RMouseButtonControl.yaml
```

### Версии для Windows
- **Установщик Setup (`.exe`)**: [RMouseButtonControl-Windows-Setup.exe](https://github.com/VLOD-ZDOV/RMouseButtonControl/releases/latest/download/RMouseButtonControl-Windows-Setup.exe) (установка в 1 клик, ярлыки в меню «Пуск», автозапуск).
- **Портативная версия Portable (`.zip`)**: [RMouseButtonControl-Windows-Portable.zip](https://github.com/VLOD-ZDOV/RMouseButtonControl/releases/latest/download/RMouseButtonControl-Windows-Portable.zip) (работает без установки).

> **Примечание:**
> - При закрытии окна настроек программа сворачивается и уходит в системный трей (продолжает работать в фоне). Полный выход доступен через контекстное меню трея.

## Сборка и запуск

### Зависимости
- **Linux:** `cargo`, `rustc`, `cmake`, `g++`, `qt6-base-dev`
- **Опционально (для C# GUI):** `.NET 8.0 SDK`

### Сборка Rust ядра
```bash
cd RMouseButtonControl
cargo build --release
```
Бинарный файл: `target/release/rmousebuttoncontrol`.

### Сборка Qt6 интерфейса
```bash
cmake -B build -S gui -DCMAKE_BUILD_TYPE=Release
cmake --build build -j $(nproc)
```
Бинарный файл: `build/rmouse-gui`.

### Запуск
```bash
# Запуск фонового сервиса
./rmousebuttoncontrol &

# Запуск интерфейса настроек
./rmouse-gui
```
