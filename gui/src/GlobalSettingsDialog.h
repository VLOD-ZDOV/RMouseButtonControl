#pragma once

#include <QDialog>
#include <QCheckBox>
#include <QComboBox>
#include <QPushButton>
#include "Database.h"

class GlobalSettingsDialog : public QDialog {
    Q_OBJECT
public:
    explicit GlobalSettingsDialog(const SettingsModel& settings, QWidget *parent = nullptr);

    SettingsModel getSettings() const;

    static bool isAutostartEnabled();
    static void setAutostartEnabled(bool enable);

private slots:
    void checkDirty();

private:
    void setupUi();

    SettingsModel m_initialSettings;
    SettingsModel m_settings;
    bool m_initialAutostart = false;

    QCheckBox* m_startMinimizedCheck = nullptr;
    QCheckBox* m_autostartCheck = nullptr;
    QCheckBox* m_loggingCheck = nullptr;
    QComboBox* m_themeCombo = nullptr;
    QComboBox* m_languageCombo = nullptr;
    QComboBox* m_guiCombo = nullptr;
    QPushButton* m_applyBtn = nullptr;
};
