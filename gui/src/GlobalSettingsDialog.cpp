#include "GlobalSettingsDialog.h"
#include "Localization.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QFormLayout>
#include <QDialogButtonBox>
#include <QProcess>
#include <QFile>
#include <QDir>
#include <QTextStream>
#include <QIcon>

static QString autostartPath() {
#ifdef Q_OS_WIN
    return QDir::homePath() + "/AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup/RMouseButtonControl.lnk";
#else
    return QDir::homePath() + "/.config/autostart/RMouseButtonControl.desktop";
#endif
}

static bool isAutostartEnabled() {
    return QFile::exists(autostartPath());
}

static void setAutostartEnabled(bool enable) {
    QString path = autostartPath();
    if (!enable) {
        if (QFile::exists(path)) {
            QFile::remove(path);
        }
    } else {
        QFileInfo info(path);
        QDir().mkpath(info.dir().absolutePath());
        QFile file(path);
        if (file.open(QIODevice::WriteOnly | QIODevice::Text)) {
            QTextStream out(&file);
            out << "[Desktop Entry]\n"
                << "Type=Application\n"
                << "Exec=rmousebuttoncontrol\n"
                << "Hidden=false\n"
                << "NoDisplay=false\n"
                << "X-GNOME-Autostart-enabled=true\n"
                << "Name=RMouseButtonControl\n"
                << "Comment=RMouseButtonControl Daemon\n";
        }
    }
}

GlobalSettingsDialog::GlobalSettingsDialog(const SettingsModel& settings, QWidget *parent)
    : QDialog(parent), m_initialSettings(settings), m_settings(settings)
{
    m_initialAutostart = isAutostartEnabled();
    setupUi();
}

void GlobalSettingsDialog::setupUi() {
    setWindowTitle(Tr("Set_Title"));
    setWindowIcon(QIcon(":/mouse.ico"));
    setMinimumWidth(420);

    QVBoxLayout* rootLayout = new QVBoxLayout(this);
    rootLayout->setContentsMargins(15, 15, 15, 15);
    rootLayout->setSpacing(12);

    m_startMinimizedCheck = new QCheckBox(Tr("Set_StartMinimized"), this);
    m_startMinimizedCheck->setChecked(m_settings.startMinimized);
    rootLayout->addWidget(m_startMinimizedCheck);

    m_autostartCheck = new QCheckBox(Tr("Set_StartMenu"), this);
    m_autostartCheck->setChecked(m_initialAutostart);
    rootLayout->addWidget(m_autostartCheck);

    m_loggingCheck = new QCheckBox(Tr("Set_Logging"), this);
    m_loggingCheck->setChecked(m_settings.loggingEnabled);
    rootLayout->addWidget(m_loggingCheck);

    QFormLayout* form = new QFormLayout();
    m_themeCombo = new QComboBox(this);
    m_themeCombo->addItem(Tr("Theme_System"), 0);
    m_themeCombo->addItem(Tr("Theme_Dark"), 1);
    m_themeCombo->addItem(Tr("Theme_Light"), 2);
    m_themeCombo->addItem("Чёрная (AMOLED Black)", 3);
    m_themeCombo->addItem(Tr("Theme_Fusion"), 4);

    int themeIdx = m_settings.theme;
    if (themeIdx < 0 || themeIdx > 4) themeIdx = 0;
    m_themeCombo->setCurrentIndex(themeIdx);
    form->addRow(Tr("Set_Theme"), m_themeCombo);

    m_languageCombo = new QComboBox(this);
    auto langs = Localization::instance().availableLanguages();
    int selLangIdx = 0;
    for (int i = 0; i < langs.size(); ++i) {
        m_languageCombo->addItem(langs[i].first, langs[i].second);
        if (m_settings.language == langs[i].second) {
            selLangIdx = i;
        }
    }
    m_languageCombo->setCurrentIndex(selLangIdx);
    form->addRow(Tr("Set_Language"), m_languageCombo);

    m_guiCombo = new QComboBox(this);
    m_guiCombo->addItem("Qt (Быстрый нативный)", "qt");
    m_guiCombo->addItem("C# Avalonia (Оригинальный)", "csharp");
    if (m_settings.preferredGui == "csharp" || m_settings.preferredGui == "avalonia") {
        m_guiCombo->setCurrentIndex(1);
    } else {
        m_guiCombo->setCurrentIndex(0);
    }
    form->addRow("Интерфейс (GUI):", m_guiCombo);

    QPushButton* openCSharpBtn = new QPushButton("Открыть C# Avalonia интерфейс", this);
    connect(openCSharpBtn, &QPushButton::clicked, this, []() {
        QProcess::startDetached("ymouse-settings");
    });
    form->addRow("", openCSharpBtn);

    rootLayout->addLayout(form);

    QDialogButtonBox* bbox = new QDialogButtonBox(this);
    m_applyBtn = bbox->addButton(Tr("Btn_Apply"), QDialogButtonBox::ApplyRole);
    QPushButton* cancelBtn = bbox->addButton(Tr("Btn_Cancel"), QDialogButtonBox::RejectRole);
    m_applyBtn->setEnabled(false); // Disabled initially!
    rootLayout->addWidget(bbox);

    connect(m_startMinimizedCheck, &QCheckBox::toggled, this, &GlobalSettingsDialog::checkDirty);
    connect(m_autostartCheck, &QCheckBox::toggled, this, &GlobalSettingsDialog::checkDirty);
    connect(m_loggingCheck, &QCheckBox::toggled, this, &GlobalSettingsDialog::checkDirty);
    connect(m_themeCombo, QOverload<int>::of(&QComboBox::currentIndexChanged), this, &GlobalSettingsDialog::checkDirty);
    connect(m_languageCombo, QOverload<int>::of(&QComboBox::currentIndexChanged), this, &GlobalSettingsDialog::checkDirty);
    connect(m_guiCombo, QOverload<int>::of(&QComboBox::currentIndexChanged), this, &GlobalSettingsDialog::checkDirty);

    connect(m_applyBtn, &QPushButton::clicked, this, [this]() {
        m_settings.startMinimized = m_startMinimizedCheck->isChecked();
        m_settings.loggingEnabled = m_loggingCheck->isChecked();
        m_settings.theme = m_themeCombo->currentData().toInt();
        m_settings.language = m_languageCombo->currentData().toString();
        m_settings.preferredGui = m_guiCombo->currentData().toString();
        setAutostartEnabled(m_autostartCheck->isChecked());
        accept();
    });
    connect(cancelBtn, &QPushButton::clicked, this, &QDialog::reject);
}

void GlobalSettingsDialog::checkDirty() {
    bool dirty = false;
    if (m_startMinimizedCheck->isChecked() != m_initialSettings.startMinimized) dirty = true;
    if (m_autostartCheck->isChecked() != m_initialAutostart) dirty = true;
    if (m_loggingCheck->isChecked() != m_initialSettings.loggingEnabled) dirty = true;
    if (m_themeCombo->currentData().toInt() != m_initialSettings.theme) dirty = true;
    if (m_languageCombo->currentData().toString() != m_initialSettings.language) dirty = true;
    if (m_guiCombo->currentData().toString() != m_initialSettings.preferredGui) dirty = true;

    m_applyBtn->setEnabled(dirty);
}

SettingsModel GlobalSettingsDialog::getSettings() const {
    return m_settings;
}
