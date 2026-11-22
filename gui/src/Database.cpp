#include "Database.h"
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QSqlError>
#include <QDir>
#include <QFileInfo>
#include <QDebug>

ButtonMappingModel ProfileModel::getSelectedMapping(int mouseButton) const {
    if (mappingsByButton.contains(mouseButton)) {
        const auto& maps = mappingsByButton[mouseButton];
        for (auto it = maps.begin(); it != maps.end(); ++it) {
            if (it.value().selected) {
                return it.value();
            }
        }
        if (maps.contains("NothingMapping")) {
            return maps["NothingMapping"];
        }
    }
    ButtonMappingModel def;
    def.mouseButton = mouseButton;
    def.discriminator = "NothingMapping";
    def.selected = true;
    return def;
}

void ProfileModel::setSelectedMapping(int mouseButton, const QString& discriminator) {
    if (!mappingsByButton.contains(mouseButton)) return;
    auto& maps = mappingsByButton[mouseButton];
    for (auto it = maps.begin(); it != maps.end(); ++it) {
        it.value().selected = (it.key() == discriminator);
    }
}

void ProfileModel::updateSimulatedKeys(int mouseButton, const QString& keys, int simType, int delay, bool randDelay, bool blockInput) {
    if (!mappingsByButton.contains(mouseButton)) return;
    auto& maps = mappingsByButton[mouseButton];
    if (maps.contains("SimulatedKeystroke")) {
        auto& sim = maps["SimulatedKeystroke"];
        sim.keys = keys;
        sim.simulatedKeystrokeType = simType;
        sim.autoRepeatDelay = delay;
        sim.autoRepeatRandomizeDelayEnabled = randDelay;
        sim.blockOriginalMouseInput = blockInput;
    }
}

Database& Database::instance() {
    static Database inst;
    return inst;
}

Database::Database() {
    m_dbPath = defaultDbPath();
}

QString Database::defaultDbPath() {
#ifdef Q_OS_WIN
    QString appData = QDir::homePath() + "/AppData/Roaming";
    QString newPath = appData + "/RMouseButtonControl/RMouseButtonControl.db";
    if (QFile::exists(newPath)) return newPath;
    QString oldPath = appData + "/YMouseButtonControl/YMouseButtonControl.db";
    if (QFile::exists(oldPath)) return oldPath;
    return newPath;
#else
    QString newPath = QDir::homePath() + "/.config/RMouseButtonControl/RMouseButtonControl.db";
    if (QFile::exists(newPath)) return newPath;
    QString oldPath = QDir::homePath() + "/.config/YMouseButtonControl/YMouseButtonControl.db";
    if (QFile::exists(oldPath)) return oldPath;
    return newPath;
#endif
}

bool Database::open() {
    if (m_opened) return true;

    QFileInfo checkFile(m_dbPath);
    if (!checkFile.exists()) {
        QDir dir(checkFile.dir());
        if (!dir.exists()) {
            dir.mkpath(".");
        }
    }

    QSqlDatabase db = QSqlDatabase::addDatabase("QSQLITE", "rmouse_gui_db");
    db.setDatabaseName(m_dbPath);

    if (!db.open()) {
        qWarning() << "Failed to open SQLite DB at" << m_dbPath << ":" << db.lastError().text();
        return false;
    }

    initSchema(db);

    m_opened = true;
    return true;
}

void Database::initSchema(QSqlDatabase& db) {
    QSqlQuery q(db);
    q.exec("CREATE TABLE IF NOT EXISTS Profiles ("
           "Id INTEGER PRIMARY KEY AUTOINCREMENT, "
           "IsDefault INTEGER NOT NULL, "
           "Checked INTEGER NOT NULL, "
           "DisplayPriority INTEGER NOT NULL, "
           "Name TEXT NOT NULL, "
           "Description TEXT NOT NULL, "
           "WindowCaption TEXT NOT NULL, "
           "Process TEXT NOT NULL, "
           "WindowClass TEXT NOT NULL, "
           "ParentClass TEXT NOT NULL, "
           "MatchType TEXT NOT NULL)");

    q.exec("CREATE TABLE IF NOT EXISTS Settings ("
           "Id INTEGER PRIMARY KEY AUTOINCREMENT, "
           "Name TEXT NOT NULL, "
           "Discriminator TEXT NOT NULL, "
           "BoolValue INTEGER, "
           "IntValue INTEGER, "
           "StringValue TEXT)");

    q.exec("CREATE TABLE IF NOT EXISTS Themes ("
           "Id INTEGER PRIMARY KEY AUTOINCREMENT, "
           "Name TEXT NOT NULL, "
           "Background TEXT NOT NULL, "
           "Highlight TEXT NOT NULL)");

    q.exec("CREATE TABLE IF NOT EXISTS ButtonMappings ("
           "Id INTEGER PRIMARY KEY AUTOINCREMENT, "
           "Keys TEXT, "
           "MouseButton INTEGER NOT NULL, "
           "ProfileId INTEGER NOT NULL, "
           "SimulatedKeystrokeType INTEGER, "
           "AutoRepeatDelay INTEGER, "
           "AutoRepeatRandomizeDelayEnabled INTEGER, "
           "Selected INTEGER NOT NULL, "
           "BlockOriginalMouseInput INTEGER, "
           "Discriminator TEXT NOT NULL, "
           "FOREIGN KEY(ProfileId) REFERENCES Profiles(Id) ON DELETE CASCADE)");

    QSqlQuery countQ("SELECT COUNT(*) FROM Profiles", db);
    if (countQ.next() && countQ.value(0).toInt() == 0) {
        q.prepare("INSERT INTO Profiles (IsDefault, Checked, DisplayPriority, Name, Description, WindowCaption, Process, WindowClass, ParentClass, MatchType) "
                  "VALUES (1, 1, 0, 'Default', 'Default Profile', 'N/A', '*', 'N/A', 'N/A', 'N/A')");
        q.exec();
        int defProfileId = q.lastInsertId().toInt();

        for (int mb = 0; mb < 9; ++mb) {
            QStringList reqDiscs = {"NothingMapping", "DisabledMapping", "SimulatedKeystroke", "RightClick"};
            for (const QString& d : reqDiscs) {
                QSqlQuery ins(db);
                ins.prepare("INSERT INTO ButtonMappings (MouseButton, ProfileId, Keys, SimulatedKeystrokeType, "
                            "AutoRepeatDelay, AutoRepeatRandomizeDelayEnabled, BlockOriginalMouseInput, Selected, Discriminator) "
                            "VALUES (?, ?, NULL, 4, 33, 0, 0, ?, ?)");
                ins.addBindValue(mb);
                ins.addBindValue(defProfileId);
                ins.addBindValue(d == "NothingMapping" ? 1 : 0);
                ins.addBindValue(d);
                ins.exec();
            }
        }

        q.exec("INSERT OR IGNORE INTO Themes (Id, Name, Background, Highlight) VALUES (1, 'Default', 'SystemAltHighColor', 'SystemAccentColor')");
        q.exec("INSERT OR IGNORE INTO Themes (Id, Name, Background, Highlight) VALUES (2, 'Light', 'White', 'Yellow')");
        q.exec("INSERT OR IGNORE INTO Themes (Id, Name, Background, Highlight) VALUES (3, 'Dark', 'Black', '#3700b3')");

        q.exec("INSERT OR IGNORE INTO Settings (Id, Name, Discriminator, BoolValue, IntValue, StringValue) VALUES (1, 'StartMinimized', 'SettingBool', 1, NULL, NULL)");
        q.exec("INSERT OR IGNORE INTO Settings (Id, Name, Discriminator, BoolValue, IntValue, StringValue) VALUES (2, 'Theme', 'SettingInt', NULL, 0, NULL)");
        q.exec("INSERT OR IGNORE INTO Settings (Id, Name, Discriminator, BoolValue, IntValue, StringValue) VALUES (3, 'Language', 'SettingString', NULL, NULL, 'system')");
        q.exec("INSERT OR IGNORE INTO Settings (Id, Name, Discriminator, BoolValue, IntValue, StringValue) VALUES (4, 'PreferredGui', 'SettingString', NULL, NULL, 'qt')");
    }
}

void Database::close() {
    if (m_opened) {
        QSqlDatabase::removeDatabase("rmouse_gui_db");
        m_opened = false;
    }
}

QVector<ProfileModel> Database::loadProfiles() {
    QVector<ProfileModel> profiles;
    if (!open()) return profiles;

    QSqlDatabase db = QSqlDatabase::database("rmouse_gui_db");
    QSqlQuery q(db);

    if (!q.exec("SELECT Id, IsDefault, Checked, DisplayPriority, Name, Description, WindowCaption, Process, WindowClass, ParentClass, MatchType "
                "FROM Profiles ORDER BY DisplayPriority ASC")) {
        qWarning() << "Error reading Profiles:" << q.lastError().text();
        return profiles;
    }

    while (q.next()) {
        ProfileModel p;
        p.id = q.value(0).toInt();
        p.isDefault = q.value(1).toBool();
        p.checked = q.value(2).toBool();
        p.displayPriority = q.value(3).toInt();
        p.name = q.value(4).toString();
        p.description = q.value(5).toString();
        p.windowCaption = q.value(6).toString();
        p.process = q.value(7).toString();
        p.windowClass = q.value(8).toString();
        p.parentClass = q.value(9).toString();
        p.matchType = q.value(10).toString();

        // Load mappings for this profile
        QSqlQuery mq(db);
        mq.prepare("SELECT Id, Keys, MouseButton, SimulatedKeystrokeType, AutoRepeatDelay, AutoRepeatRandomizeDelayEnabled, BlockOriginalMouseInput, Discriminator, Selected "
                   "FROM ButtonMappings WHERE ProfileId = ?");
        mq.addBindValue(p.id);
        if (mq.exec()) {
            while (mq.next()) {
                ButtonMappingModel bm;
                bm.id = mq.value(0).toInt();
                bm.keys = mq.value(1).toString();
                bm.mouseButton = mq.value(2).toInt();
                bm.profileId = p.id;
                bm.simulatedKeystrokeType = mq.value(3).toInt();
                bm.autoRepeatDelay = mq.value(4).toInt();
                bm.autoRepeatRandomizeDelayEnabled = mq.value(5).toBool();
                bm.blockOriginalMouseInput = mq.value(6).toBool();
                bm.discriminator = mq.value(7).toString();
                bm.selected = mq.value(8).toBool();

                p.mappingsByButton[bm.mouseButton][bm.discriminator] = bm;
            }
        }

        // Ensure 9 buttons exist with 4 standard discriminators
        for (int mb = 0; mb < 9; ++mb) {
            auto& discMap = p.mappingsByButton[mb];
            QStringList reqDiscs = {"NothingMapping", "DisabledMapping", "SimulatedKeystroke", "RightClick"};
            bool hasSelected = false;
            for (const QString& d : reqDiscs) {
                if (!discMap.contains(d)) {
                    ButtonMappingModel newBm;
                    newBm.mouseButton = mb;
                    newBm.profileId = p.id;
                    newBm.discriminator = d;
                    newBm.selected = (d == "NothingMapping");
                    discMap[d] = newBm;
                }
                if (discMap[d].selected) hasSelected = true;
            }
            if (!hasSelected) {
                discMap["NothingMapping"].selected = true;
            }
        }

        profiles.append(p);
    }

    return profiles;
}

bool Database::saveProfile(const ProfileModel& p) {
    if (!open()) return false;
    QSqlDatabase db = QSqlDatabase::database("rmouse_gui_db");

    QSqlQuery q(db);
    q.prepare("UPDATE Profiles SET Checked = ?, DisplayPriority = ?, Name = ?, Description = ?, "
              "WindowCaption = ?, Process = ?, WindowClass = ?, ParentClass = ?, MatchType = ? "
              "WHERE Id = ?");
    q.addBindValue(p.checked ? 1 : 0);
    q.addBindValue(p.displayPriority);
    q.addBindValue(p.name);
    q.addBindValue(p.description);
    q.addBindValue(p.windowCaption);
    q.addBindValue(p.process);
    q.addBindValue(p.windowClass);
    q.addBindValue(p.parentClass);
    q.addBindValue(p.matchType);
    q.addBindValue(p.id);

    if (!q.exec()) {
        qWarning() << "Failed to update profile" << p.id << ":" << q.lastError().text();
        return false;
    }

    // Save mappings
    for (auto itMb = p.mappingsByButton.begin(); itMb != p.mappingsByButton.end(); ++itMb) {
        int mb = itMb.key();
        const auto& discMap = itMb.value();
        for (auto itD = discMap.begin(); itD != discMap.end(); ++itD) {
            const ButtonMappingModel& bm = itD.value();
            if (bm.id > 0) {
                QSqlQuery mq(db);
                mq.prepare("UPDATE ButtonMappings SET Keys = ?, SimulatedKeystrokeType = ?, AutoRepeatDelay = ?, "
                           "AutoRepeatRandomizeDelayEnabled = ?, BlockOriginalMouseInput = ?, Selected = ? "
                           "WHERE Id = ?");
                mq.addBindValue(bm.keys.isEmpty() ? QVariant(QMetaType(QMetaType::QString)) : bm.keys);
                mq.addBindValue(bm.simulatedKeystrokeType);
                mq.addBindValue(bm.autoRepeatDelay);
                mq.addBindValue(bm.autoRepeatRandomizeDelayEnabled ? 1 : 0);
                mq.addBindValue(bm.blockOriginalMouseInput ? 1 : 0);
                mq.addBindValue(bm.selected ? 1 : 0);
                mq.addBindValue(bm.id);
                mq.exec();
            } else {
                QSqlQuery ins(db);
                ins.prepare("INSERT INTO ButtonMappings (MouseButton, ProfileId, Keys, SimulatedKeystrokeType, "
                            "AutoRepeatDelay, AutoRepeatRandomizeDelayEnabled, BlockOriginalMouseInput, Selected, Discriminator) "
                            "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)");
                ins.addBindValue(mb);
                ins.addBindValue(p.id);
                ins.addBindValue(bm.keys.isEmpty() ? QVariant(QMetaType(QMetaType::QString)) : bm.keys);
                ins.addBindValue(bm.simulatedKeystrokeType);
                ins.addBindValue(bm.autoRepeatDelay);
                ins.addBindValue(bm.autoRepeatRandomizeDelayEnabled ? 1 : 0);
                ins.addBindValue(bm.blockOriginalMouseInput ? 1 : 0);
                ins.addBindValue(bm.selected ? 1 : 0);
                ins.addBindValue(bm.discriminator);
                ins.exec();
            }
        }
    }

    return true;
}

bool Database::saveAllProfiles(const QVector<ProfileModel>& profiles) {
    if (!open()) return false;
    QSqlDatabase db = QSqlDatabase::database("rmouse_gui_db");
    db.transaction();
    for (const auto& p : profiles) {
        if (!saveProfile(p)) {
            db.rollback();
            return false;
        }
    }
    return db.commit();
}

int Database::addProfile(ProfileModel& p) {
    if (!open()) return 0;
    QSqlDatabase db = QSqlDatabase::database("rmouse_gui_db");

    QSqlQuery q(db);
    q.prepare("INSERT INTO Profiles (IsDefault, Checked, DisplayPriority, Name, Description, WindowCaption, Process, WindowClass, ParentClass, MatchType) "
              "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
    q.addBindValue(p.isDefault ? 1 : 0);
    q.addBindValue(p.checked ? 1 : 0);
    q.addBindValue(p.displayPriority);
    q.addBindValue(p.name);
    q.addBindValue(p.description);
    q.addBindValue(p.windowCaption);
    q.addBindValue(p.process);
    q.addBindValue(p.windowClass);
    q.addBindValue(p.parentClass);
    q.addBindValue(p.matchType);

    if (!q.exec()) {
        qWarning() << "Failed to insert profile:" << q.lastError().text();
        return 0;
    }

    p.id = q.lastInsertId().toInt();

    // Create 36 default button mapping rows
    for (int mb = 0; mb < 9; ++mb) {
        QStringList reqDiscs = {"NothingMapping", "DisabledMapping", "SimulatedKeystroke", "RightClick"};
        for (const QString& d : reqDiscs) {
            ButtonMappingModel bm;
            bm.mouseButton = mb;
            bm.profileId = p.id;
            bm.discriminator = d;
            bm.selected = (d == "NothingMapping");
            bm.simulatedKeystrokeType = 4;
            bm.autoRepeatDelay = 33;
            bm.autoRepeatRandomizeDelayEnabled = false;
            bm.blockOriginalMouseInput = false;

            QSqlQuery ins(db);
            ins.prepare("INSERT INTO ButtonMappings (MouseButton, ProfileId, Keys, SimulatedKeystrokeType, "
                        "AutoRepeatDelay, AutoRepeatRandomizeDelayEnabled, BlockOriginalMouseInput, Selected, Discriminator) "
                        "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)");
            ins.addBindValue(mb);
            ins.addBindValue(p.id);
            ins.addBindValue(QVariant(QMetaType(QMetaType::QString)));
            ins.addBindValue(bm.simulatedKeystrokeType);
            ins.addBindValue(bm.autoRepeatDelay);
            ins.addBindValue(0);
            ins.addBindValue(0);
            ins.addBindValue(bm.selected ? 1 : 0);
            ins.addBindValue(d);
            ins.exec();
            bm.id = ins.lastInsertId().toInt();

            p.mappingsByButton[mb][d] = bm;
        }
    }

    return p.id;
}

bool Database::deleteProfile(int profileId) {
    if (!open()) return false;
    QSqlDatabase db = QSqlDatabase::database("rmouse_gui_db");
    QSqlQuery dq(db);
    dq.prepare("DELETE FROM ButtonMappings WHERE ProfileId = ?");
    dq.addBindValue(profileId);
    dq.exec();

    QSqlQuery q(db);
    q.prepare("DELETE FROM Profiles WHERE Id = ?");
    q.addBindValue(profileId);
    return q.exec();
}

SettingsModel Database::loadSettings() {
    SettingsModel sm;
    if (!open()) return sm;
    QSqlDatabase db = QSqlDatabase::database("rmouse_gui_db");

    QSqlQuery q(db);
    if (q.exec("SELECT Name, BoolValue, IntValue, StringValue FROM Settings")) {
        while (q.next()) {
            QString name = q.value(0).toString();
            if (name == "StartMinimized") {
                sm.startMinimized = q.value(1).toBool();
            } else if (name == "LoggingEnabled" || name == "Logging") {
                sm.loggingEnabled = q.value(1).toBool();
            } else if (name == "Theme") {
                sm.theme = q.value(2).toInt();
            } else if (name == "Language") {
                sm.language = q.value(3).toString();
            } else if (name == "PreferredGui") {
                sm.preferredGui = q.value(3).toString();
            }
        }
    }
    return sm;
}

bool Database::saveSettings(const SettingsModel& sm) {
    if (!open()) return false;
    QSqlDatabase db = QSqlDatabase::database("rmouse_gui_db");

    QSqlQuery q(db);
    q.prepare("UPDATE Settings SET BoolValue = ? WHERE Name = 'StartMinimized'");
    q.addBindValue(sm.startMinimized ? 1 : 0);
    q.exec();

    // Check if LoggingEnabled exists, if not insert
    QSqlQuery lq(db);
    lq.prepare("SELECT COUNT(*) FROM Settings WHERE Name = 'LoggingEnabled'");
    lq.exec();
    int lcount = 0;
    if (lq.next()) lcount = lq.value(0).toInt();

    if (lcount > 0) {
        q.prepare("UPDATE Settings SET BoolValue = ? WHERE Name = 'LoggingEnabled'");
        q.addBindValue(sm.loggingEnabled ? 1 : 0);
        q.exec();
    } else {
        q.prepare("INSERT INTO Settings (Name, Discriminator, BoolValue) VALUES ('LoggingEnabled', 'SettingBool', ?)");
        q.addBindValue(sm.loggingEnabled ? 1 : 0);
        q.exec();
    }

    q.prepare("UPDATE Settings SET IntValue = ? WHERE Name = 'Theme'");
    q.addBindValue(sm.theme);
    q.exec();

    q.prepare("UPDATE Settings SET StringValue = ? WHERE Name = 'Language'");
    q.addBindValue(sm.language);
    q.exec();

    // Check if PreferredGui exists
    QSqlQuery pq(db);
    pq.prepare("SELECT COUNT(*) FROM Settings WHERE Name = 'PreferredGui'");
    pq.exec();
    int pcount = 0;
    if (pq.next()) pcount = pq.value(0).toInt();

    if (pcount > 0) {
        q.prepare("UPDATE Settings SET StringValue = ? WHERE Name = 'PreferredGui'");
        q.addBindValue(sm.preferredGui);
        q.exec();
    } else {
        q.prepare("INSERT INTO Settings (Name, Discriminator, StringValue) VALUES ('PreferredGui', 'SettingString', ?)");
        q.addBindValue(sm.preferredGui);
        q.exec();
    }

    return true;
}
