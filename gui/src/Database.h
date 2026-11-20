#pragma once

#include <QString>
#include <QVector>
#include <QMap>

struct ButtonMappingModel {
    int id = 0;
    int mouseButton = 0; // 0..8
    int profileId = 0;
    QString keys;
    int simulatedKeystrokeType = 4; // EF enum: 4=MouseButtonPressed, 6=RepeatedlyWhileButtonDown, 7=StickyHold, etc.
    int autoRepeatDelay = 33;
    bool autoRepeatRandomizeDelayEnabled = false;
    bool blockOriginalMouseInput = false;
    QString discriminator = "NothingMapping"; // "NothingMapping", "DisabledMapping", "SimulatedKeystroke", "RightClick"
    bool selected = false;
};

struct ProfileModel {
    int id = 0;
    bool isDefault = false;
    bool checked = true;
    int displayPriority = 0;
    QString name;
    QString description;
    QString windowCaption = "N/A";
    QString process = "*";
    QString windowClass = "N/A";
    QString parentClass = "N/A";
    QString matchType = "N/A";

    QMap<int, QMap<QString, ButtonMappingModel>> mappingsByButton;

    ButtonMappingModel getSelectedMapping(int mouseButton) const;
    void setSelectedMapping(int mouseButton, const QString& discriminator);
    void updateSimulatedKeys(int mouseButton, const QString& keys, int simType, int delay, bool randDelay, bool blockInput);
};

struct SettingsModel {
    bool startMinimized = true;
    bool loggingEnabled = false;
    int theme = 0; // 0: System (KDE), 1: Dark, 2: Light, 3: Black, 4: Fusion
    QString language = "system";
    QString preferredGui = "qt"; // "qt" or "csharp"
};

class Database {
public:
    static Database& instance();

    bool open();
    void close();

    QVector<ProfileModel> loadProfiles();
    bool saveProfile(const ProfileModel& profile);
    bool saveAllProfiles(const QVector<ProfileModel>& profiles);
    int addProfile(ProfileModel& profile);
    bool deleteProfile(int profileId);

    SettingsModel loadSettings();
    bool saveSettings(const SettingsModel& settings);

    static QString defaultDbPath();

private:
    Database();
    bool m_opened = false;
    QString m_dbPath;
};
