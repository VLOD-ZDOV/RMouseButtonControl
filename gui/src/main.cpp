#include <QApplication>
#include <QIcon>
#include <QSystemTrayIcon>
#include <cstring>
#include "MainWindow.h"
#include "Localization.h"
#include "Database.h"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    app.setApplicationName("RMouseButtonControl");
    app.setApplicationDisplayName(""); // Prevent window manager from appending a secondary name with a dash
    app.setDesktopFileName("RMouseButtonControl");
    app.setWindowIcon(QIcon(":/mouse.png"));

    if (QSystemTrayIcon::isSystemTrayAvailable()) {
        app.setQuitOnLastWindowClosed(false);
    }

    SettingsModel sm = Database::instance().loadSettings();
    Localization::instance().setLanguage(sm.language);

    MainWindow w;

    bool startMinimized = sm.startMinimized;
    for (int i = 1; i < argc; ++i) {
        if (std::strcmp(argv[i], "--minimized") == 0 || std::strcmp(argv[i], "-m") == 0) {
            startMinimized = true;
        }
    }

    if (!startMinimized || !QSystemTrayIcon::isSystemTrayAvailable()) {
        w.show();
    }

    return app.exec();
}
