# RMouseButtonControl

[English](#english) | [Русский](#русский)

---

<a name="english"></a>
## English

**RMouseButtonControl** is a high-performance mouse remapping and macro automation utility. It combines a pure safe Rust background daemon with a fast native Qt6 graphical interface and an optional Fluent Avalonia C# interface.

### Highlights & Architecture

- **High-Performance Rust Core Engine:**
  - Written with `#![forbid(unsafe_code)]` for guaranteed memory safety and stability.
  - Native Linux Kernel `/dev/uinput` virtual device driver for sub-millisecond input simulation, fully compatible with Wayland and gaming environments with anti-cheat.
  - Userspace fallback driver for environments without direct `/dev/uinput` access.
  - Low memory footprint (~10 MB RAM daemon) and negligible CPU usage.
- **Dual GUI Frontends (Seamless Switcher):**
  - **Qt6 Native GUI (Default):** Instant startup (<20 ms), lightweight (~20 MB RAM), responsive layout, and polished controls.
  - **Avalonia C# GUI (Optional):** Original Fluent interface design with rich controls.
  - Switch between GUIs at any time via the tray menu or settings dialog.
- **Desktop & Tray Integration:**
  - Standard FreeDesktop / KDE Plasma `StatusNotifierItem` system tray menu.
  - Profile switcher, pause/resume toggle, autostart management, and one-click database reload.
- **7 Macro Simulation Modes:**
  1. As mouse button is pressed
  2. As mouse button is released
  3. During (press on down, release on up)
  4. In another thread as mouse button is pressed
  5. In another thread as mouse button is released
  6. Repeatedly while button is held down (with configurable millisecond delay and 10% random jitter)
  7. Sticky modes (hold or repeat until pressed again)
- **Automatic Process & Window Matching:**
  - Profiles activate automatically based on the active foreground window.
  - Full support for native Linux applications as well as Wine and Proton games.
- **5 Appearance Themes:**
  - System (KDE Breeze native)
  - Modern Dark (Harmonious Fluent/Breeze Dark)
  - Modern Light (Clean White Fluent)
  - AMOLED Black (Pure black `#000000` with high-contrast elements)
  - Fusion
- **Multi-language Support:**
  - Localized into 8 languages: English, Russian, German, Spanish, French, Ukrainian, Portuguese, and Simplified Chinese.

### Installation & Build

#### Prerequisites
- **Linux:** `cargo`, `rustc`, `cmake`, `g++`, `qt6-base-dev`
- **Optional (for C# GUI):** `.NET 8.0 SDK`

#### Building the Rust Core Daemon
```bash
cd RMouseButtonControl
cargo build --release
```
The daemon binary will be produced in `target/release/rmousebuttoncontrol`.

#### Building the Qt6 Native GUI
```bash
cmake -B build -S gui -DCMAKE_BUILD_TYPE=Release
cmake --build build -j $(nproc)
```
The GUI binary will be produced in `build/rmouse-gui`.

#### Running
```bash
# Start background daemon
./rmousebuttoncontrol &

# Open settings GUI
./rmouse-gui
```

---

<a name="русский"></a>
## Русский

**RMouseButtonControl** — высокопроизводительная утилита для переназначения кнопок мыши и симуляции макросов. Проект сочетает фоновый сервис на чистом безопасном Rust, быстрый нативный интерфейс на Qt6 по умолчанию и возможность запуска оригинального Fluent Avalonia C# интерфейса.

### Ключевые возможности и архитектура

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
- **7 режимов симуляции макросов:**
  1. При нажатии кнопки мыши
  2. При отпускании кнопки мыши
  3. В течение (нажатие при опускании, отпускание при подъёме)
  4. В отдельном потоке при нажатии
  5. В отдельном потоке при отпускании
  6. Повторение при удержании кнопки (с настраиваемой задержкой в мс и 10% случайным джиттером)
  7. Залипание (удержание или повтор до повторного нажатия)
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

### Сборка и запуск

#### Зависимости
- **Linux:** `cargo`, `rustc`, `cmake`, `g++`, `qt6-base-dev`
- **Опционально (для C# GUI):** `.NET 8.0 SDK`

#### Сборка Rust ядра
```bash
cd RMouseButtonControl
cargo build --release
```
Бинарный файл: `target/release/rmousebuttoncontrol`.

#### Сборка Qt6 интерфейса
```bash
cmake -B build -S gui -DCMAKE_BUILD_TYPE=Release
cmake --build build -j $(nproc)
```
Бинарный файл: `build/rmouse-gui`.

#### Запуск
```bash
# Запуск фонового сервиса
./rmousebuttoncontrol &

# Запуск интерфейса настроек
./rmouse-gui
```
