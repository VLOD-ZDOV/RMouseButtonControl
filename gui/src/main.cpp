#include <QApplication>
#include "MainWindow.h"
#include "Localization.h"
#include "Database.h"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    app.setApplicationName("RMouseButtonControl");
    app.setApplicationDisplayName(""); // Prevent window manager from appending a secondary name with a dash
    app.setDesktopFileName("RMouseButtonControl");

    SettingsModel sm = Database::instance().loadSettings();
    Localization::instance().setLanguage(sm.language);

    MainWindow w;
    w.show();

    return app.exec();
}
