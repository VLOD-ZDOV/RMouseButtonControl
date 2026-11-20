#pragma once

#include <QMainWindow>
#include <QListWidget>
#include <QTabWidget>
#include <QComboBox>
#include <QPushButton>
#include <QLineEdit>
#include <QLabel>
#include <QVector>
#include "Database.h"

class MainWindow : public QMainWindow {
    Q_OBJECT
public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow() override;

private slots:
    void onProfileSelected(int index);
    void onProfileItemChanged(QListWidgetItem* item);
    void onAddProfile();
    void onCopyProfile();
    void onEditProfile();
    void onRemoveProfile();
    void onMoveProfileUp();
    void onMoveProfileDown();
    void onExportProfile();
    void onImportProfile();
    void onButtonComboChanged(int mbIndex, int comboIndex);
    void onButtonGearClicked(int mbIndex);
    void onOpenSettings();
    void onApplyChanges();

private:
    void setupUi();
    void loadData();
    void populateProfilesList();
    void displayCurrentProfile();
    void applyTheme(int themeIndex);
    void setDirty(bool dirty);

    QVector<ProfileModel> m_profiles;
    int m_currentProfileIndex = 0;
    SettingsModel m_settings;

    // Left pane
    QListWidget* m_profilesList = nullptr;
    QPushButton* m_addBtn = nullptr;
    QPushButton* m_copyBtn = nullptr;
    QPushButton* m_exportBtn = nullptr;
    QPushButton* m_upBtn = nullptr;
    QPushButton* m_editBtn = nullptr;
    QPushButton* m_removeBtn = nullptr;
    QPushButton* m_importBtn = nullptr;
    QPushButton* m_downBtn = nullptr;

    // Right pane
    QTabWidget* m_tabWidget = nullptr;
    struct MouseRow {
        QLabel* label = nullptr;
        QComboBox* combo = nullptr;
        QPushButton* gearBtn = nullptr;
    };
    MouseRow m_mouseRows[9];

    // Profile info labels
    QLabel* m_infoDesc = nullptr;
    QLabel* m_infoCaption = nullptr;
    QLabel* m_infoProcess = nullptr;
    QLabel* m_infoClass = nullptr;
    QLabel* m_infoParentClass = nullptr;
    QLabel* m_infoMatchType = nullptr;

    // Bottom bar
    QPushButton* m_settingsBtn = nullptr;
    QPushButton* m_saveProfileBtn = nullptr;
    QPushButton* m_loadProfileBtn = nullptr;
    QLineEdit* m_currentProfileStatus = nullptr;
    QPushButton* m_applyBtn = nullptr;
    QPushButton* m_closeBtn = nullptr;

    bool m_dirty = false;
};
