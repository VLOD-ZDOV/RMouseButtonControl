# RMouseButtonControl

[Русская версия / Russian version](README.ru.md)

![RMouseButtonControl](https://github.com/user-attachments/assets/a42266b8-dead-4dfd-b894-2deba1e9aa7e)

**RMouseButtonControl** is a high-performance mouse remapping and macro automation utility. It combines a pure safe Rust background daemon with a fast native Qt6 graphical interface and an optional Fluent Avalonia C# interface.

## Highlights & Architecture

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
- **Flexible Macro & Keystroke Simulation:** Support for press/release triggers, chorded hotkeys, sticky keys, and auto-repeat with configurable delay.
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

## Quick Install

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

### Windows Packages
- **Setup Installer (`.exe`)**: [RMouseButtonControl-Windows-Setup.exe](https://github.com/VLOD-ZDOV/RMouseButtonControl/releases/latest/download/RMouseButtonControl-Windows-Setup.exe) (1-click wizard, Start Menu shortcuts, autostart).
- **Portable (`.zip`)**: [RMouseButtonControl-Windows-Portable.zip](https://github.com/VLOD-ZDOV/RMouseButtonControl/releases/latest/download/RMouseButtonControl-Windows-Portable.zip) (run without installation).

> **Note:**
> - Closing the settings window minimizes the application into the system tray where it continues running in the background. To exit completely, use the tray icon context menu.

## Installation & Build

### Prerequisites
- **Linux:** `cargo`, `rustc`, `cmake`, `g++`, `qt6-base-dev`
- **Optional (for C# GUI):** `.NET 8.0 SDK`

### Building the Rust Core Daemon
```bash
cd RMouseButtonControl
cargo build --release
```
The daemon binary will be produced in `target/release/rmousebuttoncontrol`.

### Building the Qt6 Native GUI
```bash
cmake -B build -S gui -DCMAKE_BUILD_TYPE=Release
cmake --build build -j $(nproc)
```
The GUI binary will be produced in `build/rmouse-gui`.

### Running
```bash
# Start background daemon
./rmousebuttoncontrol &

# Open settings GUI
./rmouse-gui
```
