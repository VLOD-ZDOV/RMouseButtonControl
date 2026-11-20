#include "MainWindow.h"
#include "Localization.h"
#include "SimulatedKeystrokesDialog.h"
#include "GlobalSettingsDialog.h"
#include "ProcessSelectorDialog.h"

#include <QApplication>
#include <QStyleFactory>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QGridLayout>
#include <QFormLayout>
#include <QGroupBox>
#include <QFileDialog>
#include <QMessageBox>
#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonArray>
#include <QFile>
#include <QDir>
#include <QIcon>
#include <QProcess>
#include <QActionGroup>
#include <QDebug>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    setupUi();
    loadData();
    setupTray();
}

MainWindow::~MainWindow() {
    if (m_trayIcon) {
        m_trayIcon->hide();
    }
}

void MainWindow::setupUi() {
    setWindowTitle("RMouseButtonControl");
    setWindowIcon(QIcon(":/mouse.ico"));
    resize(1000, 700);
    setMinimumSize(920, 660);

    QWidget* central = new QWidget(this);
    setCentralWidget(central);

    QVBoxLayout* rootLayout = new QVBoxLayout(central);
    rootLayout->setContentsMargins(12, 12, 12, 12);
    rootLayout->setSpacing(10);

    // Main split layout: Left (Profiles) & Right (Layers / Info)
    QHBoxLayout* mainSplit = new QHBoxLayout();
    mainSplit->setSpacing(14);

    // ==========================================
    // LEFT PANEL (Profiles)
    // ==========================================
    QVBoxLayout* leftPanel = new QVBoxLayout();
    leftPanel->setSpacing(6);

    QLabel* profilesHeader = new QLabel(Tr("Main_AppWindowProfiles"), this);
    QFont hFont = profilesHeader->font();
    hFont.setBold(true);
    profilesHeader->setFont(hFont);
    leftPanel->addWidget(profilesHeader);

    m_profilesList = new QListWidget(this);
    m_profilesList->setFixedWidth(380);
    leftPanel->addWidget(m_profilesList);

    connect(m_profilesList, &QListWidget::currentRowChanged, this, &MainWindow::onProfileSelected);
    connect(m_profilesList, &QListWidget::itemChanged, this, &MainWindow::onProfileItemChanged);

    // Buttons grid below profiles list (2 rows x 4 columns)
    QGridLayout* btnGrid = new QGridLayout();
    btnGrid->setSpacing(4);

    m_addBtn = new QPushButton(Tr("Btn_Add"), this);
    m_copyBtn = new QPushButton(Tr("Btn_Copy"), this);
    m_exportBtn = new QPushButton(Tr("Btn_Export"), this);
    m_upBtn = new QPushButton(Tr("Btn_Up"), this);

    m_editBtn = new QPushButton(Tr("Btn_Edit"), this);
    m_removeBtn = new QPushButton(Tr("Btn_Remove"), this);
    m_importBtn = new QPushButton(Tr("Btn_Import"), this);
    m_downBtn = new QPushButton(Tr("Btn_Down"), this);

    btnGrid->addWidget(m_addBtn, 0, 0);
    btnGrid->addWidget(m_copyBtn, 0, 1);
    btnGrid->addWidget(m_exportBtn, 0, 2);
    btnGrid->addWidget(m_upBtn, 0, 3);

    btnGrid->addWidget(m_editBtn, 1, 0);
    btnGrid->addWidget(m_removeBtn, 1, 1);
    btnGrid->addWidget(m_importBtn, 1, 2);
    btnGrid->addWidget(m_downBtn, 1, 3);

    leftPanel->addLayout(btnGrid);
    mainSplit->addLayout(leftPanel);

    connect(m_addBtn, &QPushButton::clicked, this, &MainWindow::onAddProfile);
    connect(m_copyBtn, &QPushButton::clicked, this, &MainWindow::onCopyProfile);
    connect(m_editBtn, &QPushButton::clicked, this, &MainWindow::onEditProfile);
    connect(m_removeBtn, &QPushButton::clicked, this, &MainWindow::onRemoveProfile);
    connect(m_upBtn, &QPushButton::clicked, this, &MainWindow::onMoveProfileUp);
    connect(m_downBtn, &QPushButton::clicked, this, &MainWindow::onMoveProfileDown);
    connect(m_exportBtn, &QPushButton::clicked, this, &MainWindow::onExportProfile);
    connect(m_importBtn, &QPushButton::clicked, this, &MainWindow::onImportProfile);

    // ==========================================
    // RIGHT PANEL (TabWidget + Profile Info)
    // ==========================================
    QVBoxLayout* rightPanel = new QVBoxLayout();
    rightPanel->setSpacing(8);

    m_tabWidget = new QTabWidget(this);

    // --- Tab 1: Layer 1 ---
    QWidget* layer1Widget = new QWidget();
    QVBoxLayout* layer1Layout = new QVBoxLayout(layer1Widget);
    layer1Layout->setContentsMargins(8, 8, 8, 8);
    layer1Layout->setSpacing(6);

    // Layer header
    QHBoxLayout* layerHeader = new QHBoxLayout();
    QLabel* layerNameLabel = new QLabel(Tr("Layer_Name"), this);
    layerNameLabel->setFixedWidth(140);
    QLineEdit* layerNameEdit = new QLineEdit(Tr("Layer_1Default"), this);
    QPushButton* copyLayerBtn = new QPushButton(Tr("Btn_Copy"), this);
    QPushButton* swapLayerBtn = new QPushButton(Tr("Layer_Swap"), this);
    QPushButton* resetLayerBtn = new QPushButton(Tr("Layer_Reset"), this);
    copyLayerBtn->setEnabled(false);
    swapLayerBtn->setEnabled(false);
    resetLayerBtn->setEnabled(false);

    layerHeader->addWidget(layerNameLabel);
    layerHeader->addWidget(layerNameEdit);
    layerHeader->addWidget(copyLayerBtn);
    layerHeader->addWidget(swapLayerBtn);
    layerHeader->addWidget(resetLayerBtn);
    layer1Layout->addLayout(layerHeader);

    // 9 Mouse Button rows
    QStringList btnLabels = {
        Tr("Mb_Left"),
        Tr("Mb_Right"),
        Tr("Mb_Middle"),
        Tr("Mb_Button4"),
        Tr("Mb_Button5"),
        Tr("Mb_WheelUp"),
        Tr("Mb_WheelDown"),
        Tr("Mb_WheelLeft"),
        Tr("Mb_WheelRight")
    };

    for (int mb = 0; mb < 9; ++mb) {
        QHBoxLayout* row = new QHBoxLayout();
        row->setSpacing(6);

        m_mouseRows[mb].label = new QLabel(btnLabels[mb], this);
        m_mouseRows[mb].label->setFixedWidth(140);

        m_mouseRows[mb].combo = new QComboBox(this);
        m_mouseRows[mb].combo->addItem(Tr("Map_NoChange"));          // idx 0
        m_mouseRows[mb].combo->addItem(Tr("Map_Disabled"));          // idx 1
        m_mouseRows[mb].combo->addItem(Tr("Map_SimulatedUndefined")); // idx 2
        m_mouseRows[mb].combo->addItem(Tr("Map_RightClick"));        // idx 3
        m_mouseRows[mb].combo->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);

        m_mouseRows[mb].gearBtn = new QPushButton(this);
        m_mouseRows[mb].gearBtn->setProperty("gear", true);
        m_mouseRows[mb].gearBtn->setIcon(QIcon(":/gear_icon1.png"));
        m_mouseRows[mb].gearBtn->setFixedSize(32, 28);

        row->addWidget(m_mouseRows[mb].label);
        row->addWidget(m_mouseRows[mb].combo);
        row->addWidget(m_mouseRows[mb].gearBtn);
        layer1Layout->addLayout(row);

        connect(m_mouseRows[mb].combo, QOverload<int>::of(&QComboBox::currentIndexChanged),
                this, [this, mb](int comboIdx) { onButtonComboChanged(mb, comboIdx); });
        connect(m_mouseRows[mb].gearBtn, &QPushButton::clicked,
                this, [this, mb]() { onButtonGearClicked(mb); });
    }

    m_tabWidget->addTab(layer1Widget, Tr("Layer_Tab1"));
    m_tabWidget->addTab(new QWidget(), Tr("Layer_Tab2"));
    m_tabWidget->addTab(new QWidget(), Tr("Layer_TabScrolling"));
    m_tabWidget->addTab(new QWidget(), Tr("Layer_TabOptions"));

    rightPanel->addWidget(m_tabWidget);

    // Profile information box
    QLabel* infoHeader = new QLabel(Tr("Main_ProfileInformation"), this);
    infoHeader->setFont(hFont);
    rightPanel->addWidget(infoHeader);

    QGridLayout* infoGrid = new QGridLayout();
    infoGrid->setContentsMargins(10, 4, 10, 4);
    infoGrid->setSpacing(5);

    QLabel* lDesc = new QLabel(Tr("Info_Description"), this);
    QLabel* lCap = new QLabel(Tr("Info_WindowCaption"), this);
    QLabel* lProc = new QLabel(Tr("Info_Process"), this);
    QLabel* lClass = new QLabel(Tr("Info_WindowClass"), this);
    QLabel* lParent = new QLabel(Tr("Info_ParentClass"), this);
    QLabel* lMatch = new QLabel(Tr("Info_MatchType"), this);

    m_infoDesc = new QLabel("Default", this);
    m_infoCaption = new QLabel("N/A", this);
    m_infoProcess = new QLabel("*", this);
    m_infoClass = new QLabel("N/A", this);
    m_infoParentClass = new QLabel("N/A", this);
    m_infoMatchType = new QLabel("N/A", this);

    infoGrid->addWidget(lDesc, 0, 0);
    infoGrid->addWidget(m_infoDesc, 0, 1);
    infoGrid->addWidget(lCap, 1, 0);
    infoGrid->addWidget(m_infoCaption, 1, 1);
    infoGrid->addWidget(lProc, 2, 0);
    infoGrid->addWidget(m_infoProcess, 2, 1);
    infoGrid->addWidget(lClass, 3, 0);
    infoGrid->addWidget(m_infoClass, 3, 1);
    infoGrid->addWidget(lParent, 4, 0);
    infoGrid->addWidget(m_infoParentClass, 4, 1);
    infoGrid->addWidget(lMatch, 5, 0);
    infoGrid->addWidget(m_infoMatchType, 5, 1);

    rightPanel->addLayout(infoGrid);
    mainSplit->addLayout(rightPanel);
    rootLayout->addLayout(mainSplit);

    // ==========================================
    // BOTTOM ACTION BAR (6 columns)
    // ==========================================
    QHBoxLayout* bottomBar = new QHBoxLayout();
    bottomBar->setSpacing(6);

    m_settingsBtn = new QPushButton(Tr("Main_Settings"), this);
    m_saveProfileBtn = new QPushButton(Tr("Main_SaveProfile"), this);
    m_loadProfileBtn = new QPushButton(Tr("Main_LoadProfile"), this);

    m_currentProfileStatus = new QLineEdit("Profile: Default", this);
    m_currentProfileStatus->setReadOnly(true);

    m_applyBtn = new QPushButton(Tr("Btn_Apply"), this);
    m_closeBtn = new QPushButton(Tr("Btn_Close"), this);

    m_applyBtn->setEnabled(false); // Disabled initially!

    bottomBar->addWidget(m_settingsBtn);
    bottomBar->addWidget(m_saveProfileBtn);
    bottomBar->addWidget(m_loadProfileBtn);
    bottomBar->addWidget(m_currentProfileStatus, 1);
    bottomBar->addWidget(m_applyBtn);
    bottomBar->addWidget(m_closeBtn);

    rootLayout->addLayout(bottomBar);

    connect(m_settingsBtn, &QPushButton::clicked, this, &MainWindow::onOpenSettings);
    connect(m_saveProfileBtn, &QPushButton::clicked, this, &MainWindow::onExportProfile);
    connect(m_loadProfileBtn, &QPushButton::clicked, this, &MainWindow::onImportProfile);
    connect(m_applyBtn, &QPushButton::clicked, this, &MainWindow::onApplyChanges);
    connect(m_closeBtn, &QPushButton::clicked, this, &MainWindow::close);
}

void MainWindow::setDirty(bool dirty) {
    m_dirty = dirty;
    m_applyBtn->setEnabled(dirty);
}

void MainWindow::loadData() {
    Database& db = Database::instance();
    m_settings = db.loadSettings();
    applyTheme(m_settings.theme);

    m_profiles = db.loadProfiles();
    if (m_profiles.isEmpty()) {
        ProfileModel def;
        def.name = "Default";
        def.description = "Default";
        def.isDefault = true;
        def.checked = true;
        def.process = "*";
        db.addProfile(def);
        m_profiles.append(def);
    }

    populateProfilesList();
    setDirty(false);
}

void MainWindow::populateProfilesList() {
    m_profilesList->blockSignals(true);
    m_profilesList->clear();

    for (const auto& p : m_profiles) {
        QListWidgetItem* item = new QListWidgetItem(p.description.isEmpty() ? p.name : p.description, m_profilesList);
        item->setFlags(item->flags() | Qt::ItemIsUserCheckable | Qt::ItemIsSelectable | Qt::ItemIsEnabled);
        item->setCheckState(p.checked ? Qt::Checked : Qt::Unchecked);
    }

    if (m_currentProfileIndex >= m_profiles.size()) {
        m_currentProfileIndex = 0;
    }

    m_profilesList->setCurrentRow(m_currentProfileIndex);
    m_profilesList->blockSignals(false);

    displayCurrentProfile();
}

void MainWindow::displayCurrentProfile() {
    if (m_currentProfileIndex < 0 || m_currentProfileIndex >= m_profiles.size()) return;

    const ProfileModel& cur = m_profiles[m_currentProfileIndex];

    m_currentProfileStatus->setText(QString("Profile: %1").arg(cur.name));
    m_infoDesc->setText(cur.description);
    m_infoCaption->setText(cur.windowCaption);
    m_infoProcess->setText(cur.process);
    m_infoClass->setText(cur.windowClass);
    m_infoParentClass->setText(cur.parentClass);
    m_infoMatchType->setText(cur.matchType);

    bool isDef = cur.isDefault;
    m_removeBtn->setEnabled(!isDef);
    m_editBtn->setEnabled(!isDef);
    m_exportBtn->setEnabled(!isDef);
    m_upBtn->setEnabled(!isDef && m_currentProfileIndex > 1);
    m_downBtn->setEnabled(!isDef && m_currentProfileIndex < m_profiles.size() - 1);

    // Update 9 mouse button combos
    for (int mb = 0; mb < 9; ++mb) {
        ButtonMappingModel bm = cur.getSelectedMapping(mb);

        auto* combo = m_mouseRows[mb].combo;
        combo->blockSignals(true);

        // Update Simulated Keys text if keys exist
        ButtonMappingModel simBm = cur.mappingsByButton.value(mb).value("SimulatedKeystroke");
        if (!simBm.keys.isEmpty()) {
            QString fmt = Tr("Map_SimulatedKeysFmt");
            fmt.replace("{0}", "%1");
            combo->setItemText(2, fmt.arg(simBm.keys));
        } else {
            combo->setItemText(2, Tr("Map_SimulatedUndefined"));
        }

        if (bm.discriminator == "DisabledMapping") {
            combo->setCurrentIndex(1);
        } else if (bm.discriminator == "SimulatedKeystroke") {
            combo->setCurrentIndex(2);
        } else if (bm.discriminator == "RightClick") {
            combo->setCurrentIndex(3);
        } else {
            combo->setCurrentIndex(0);
        }

        combo->blockSignals(false);
    }
}

void MainWindow::onProfileSelected(int index) {
    if (index >= 0 && index < m_profiles.size()) {
        m_currentProfileIndex = index;
        displayCurrentProfile();
    }
}

void MainWindow::onProfileItemChanged(QListWidgetItem* item) {
    int row = m_profilesList->row(item);
    if (row >= 0 && row < m_profiles.size()) {
        bool checked = (item->checkState() == Qt::Checked);
        if (m_profiles[row].checked != checked) {
            m_profiles[row].checked = checked;
            setDirty(true);
        }
    }
}

void MainWindow::onButtonComboChanged(int mbIndex, int comboIndex) {
    if (m_currentProfileIndex < 0 || m_currentProfileIndex >= m_profiles.size()) return;

    QString disc;
    switch (comboIndex) {
        case 1: disc = "DisabledMapping"; break;
        case 2: disc = "SimulatedKeystroke"; break;
        case 3: disc = "RightClick"; break;
        default: disc = "NothingMapping"; break;
    }

    m_profiles[m_currentProfileIndex].setSelectedMapping(mbIndex, disc);
    setDirty(true);

    if (comboIndex == 2) {
        ButtonMappingModel simBm = m_profiles[m_currentProfileIndex].mappingsByButton.value(mbIndex).value("SimulatedKeystroke");
        if (simBm.keys.isEmpty()) {
            onButtonGearClicked(mbIndex);
        }
    }
}

void MainWindow::onButtonGearClicked(int mbIndex) {
    if (m_currentProfileIndex < 0 || m_currentProfileIndex >= m_profiles.size()) return;

    ProfileModel& cur = m_profiles[m_currentProfileIndex];
    ButtonMappingModel simBm = cur.mappingsByButton.value(mbIndex).value("SimulatedKeystroke");

    SimulatedKeystrokesDialog dlg(mbIndex, simBm, this);
    if (dlg.exec() == QDialog::Accepted) {
        ButtonMappingModel updated = dlg.getMapping();
        cur.mappingsByButton[mbIndex]["SimulatedKeystroke"] = updated;
        cur.setSelectedMapping(mbIndex, "SimulatedKeystroke");

        auto* combo = m_mouseRows[mbIndex].combo;
        combo->blockSignals(true);
        QString fmt = Tr("Map_SimulatedKeysFmt");
        fmt.replace("{0}", "%1");
        combo->setItemText(2, fmt.arg(updated.keys));
        combo->setCurrentIndex(2);
        combo->blockSignals(false);

        setDirty(true);
    }
}

void MainWindow::onAddProfile() {
    ProcessSelectorDialog dlg(this);
    if (dlg.exec() == QDialog::Accepted) {
        SelectedProcessInfo info = dlg.getSelectedProcess();

        ProfileModel p;
        p.name = info.name;
        p.description = info.description;
        p.process = info.process;
        p.windowCaption = info.windowCaption.isEmpty() ? "N/A" : info.windowCaption;
        p.displayPriority = m_profiles.size();
        p.checked = true;
        p.isDefault = false;

        Database::instance().addProfile(p);
        m_profiles.append(p);
        m_currentProfileIndex = m_profiles.size() - 1;

        populateProfilesList();
        setDirty(true);
    }
}

void MainWindow::onCopyProfile() {
    if (m_currentProfileIndex < 0 || m_currentProfileIndex >= m_profiles.size()) return;

    const ProfileModel& src = m_profiles[m_currentProfileIndex];
    ProcessSelectorDialog dlg(this);
    if (dlg.exec() == QDialog::Accepted) {
        SelectedProcessInfo info = dlg.getSelectedProcess();

        ProfileModel p = src;
        p.id = 0;
        p.isDefault = false;
        p.name = info.name;
        p.description = info.description;
        p.process = info.process;
        p.displayPriority = m_profiles.size();

        Database::instance().addProfile(p);
        for (int mb = 0; mb < 9; ++mb) {
            for (auto it = src.mappingsByButton.value(mb).begin(); it != src.mappingsByButton.value(mb).end(); ++it) {
                p.mappingsByButton[mb][it.key()] = it.value();
                p.mappingsByButton[mb][it.key()].profileId = p.id;
            }
        }
        Database::instance().saveProfile(p);

        m_profiles.append(p);
        m_currentProfileIndex = m_profiles.size() - 1;
        populateProfilesList();
        setDirty(true);
    }
}

void MainWindow::onEditProfile() {
    if (m_currentProfileIndex < 0 || m_currentProfileIndex >= m_profiles.size()) return;
    ProfileModel& cur = m_profiles[m_currentProfileIndex];
    if (cur.isDefault) return;

    ProcessSelectorDialog dlg(this);
    if (dlg.exec() == QDialog::Accepted) {
        SelectedProcessInfo info = dlg.getSelectedProcess();
        cur.name = info.name;
        cur.description = info.description;
        cur.process = info.process;
        populateProfilesList();
        setDirty(true);
    }
}

void MainWindow::onRemoveProfile() {
    if (m_currentProfileIndex < 0 || m_currentProfileIndex >= m_profiles.size()) return;
    ProfileModel& cur = m_profiles[m_currentProfileIndex];
    if (cur.isDefault) return;

    if (QMessageBox::question(this, Tr("Btn_Remove"),
        QString("Are you sure you want to remove profile '%1'?").arg(cur.name)) == QMessageBox::Yes) {
        Database::instance().deleteProfile(cur.id);
        m_profiles.removeAt(m_currentProfileIndex);
        if (m_currentProfileIndex >= m_profiles.size()) {
            m_currentProfileIndex = m_profiles.size() - 1;
        }
        populateProfilesList();
        setDirty(true);
    }
}

void MainWindow::onMoveProfileUp() {
    if (m_currentProfileIndex <= 1 || m_currentProfileIndex >= m_profiles.size()) return;

    int prev = m_currentProfileIndex - 1;
    if (m_profiles[prev].isDefault) return;

    std::swap(m_profiles[m_currentProfileIndex].displayPriority, m_profiles[prev].displayPriority);
    m_profiles.swapItemsAt(m_currentProfileIndex, prev);
    m_currentProfileIndex = prev;
    populateProfilesList();
    setDirty(true);
}

void MainWindow::onMoveProfileDown() {
    if (m_currentProfileIndex < 1 || m_currentProfileIndex >= m_profiles.size() - 1) return;

    int next = m_currentProfileIndex + 1;
    std::swap(m_profiles[m_currentProfileIndex].displayPriority, m_profiles[next].displayPriority);
    m_profiles.swapItemsAt(m_currentProfileIndex, next);
    m_currentProfileIndex = next;
    populateProfilesList();
    setDirty(true);
}

void MainWindow::onExportProfile() {
    if (m_currentProfileIndex < 0 || m_currentProfileIndex >= m_profiles.size()) return;
    const ProfileModel& cur = m_profiles[m_currentProfileIndex];

    QString fileName = QFileDialog::getSaveFileName(this, Tr("Main_SaveProfile"),
        QDir::homePath() + QString("/%1.json").arg(cur.name), "Profile JSON (*.json)");
    if (fileName.isEmpty()) return;

    QJsonObject root;
    root["name"] = cur.name;
    root["description"] = cur.description;
    root["process"] = cur.process;
    root["checked"] = cur.checked;

    QJsonArray mapsArr;
    for (int mb = 0; mb < 9; ++mb) {
        ButtonMappingModel bm = cur.getSelectedMapping(mb);
        QJsonObject mapObj;
        mapObj["mouseButton"] = mb;
        mapObj["discriminator"] = bm.discriminator;
        mapObj["keys"] = bm.keys;
        mapObj["simType"] = bm.simulatedKeystrokeType;
        mapObj["delay"] = bm.autoRepeatDelay;
        mapObj["rand"] = bm.autoRepeatRandomizeDelayEnabled;
        mapObj["block"] = bm.blockOriginalMouseInput;
        mapsArr.append(mapObj);
    }
    root["mappings"] = mapsArr;

    QFile file(fileName);
    if (file.open(QIODevice::WriteOnly)) {
        file.write(QJsonDocument(root).toJson(QJsonDocument::Indented));
    }
}

void MainWindow::onImportProfile() {
    QString fileName = QFileDialog::getOpenFileName(this, Tr("Main_LoadProfile"),
        QDir::homePath(), "Profile JSON (*.json)");
    if (fileName.isEmpty()) return;

    QFile file(fileName);
    if (!file.open(QIODevice::ReadOnly)) return;

    QJsonDocument doc = QJsonDocument::fromJson(file.readAll());
    if (!doc.isObject()) return;

    QJsonObject root = doc.object();
    ProfileModel p;
    p.name = root["name"].toString("Imported Profile");
    p.description = root["description"].toString("Imported");
    p.process = root["process"].toString("*");
    p.checked = root["checked"].toBool(true);
    p.displayPriority = m_profiles.size();

    Database::instance().addProfile(p);

    QJsonArray mapsArr = root["mappings"].toArray();
    for (const auto& val : mapsArr) {
        QJsonObject mapObj = val.toObject();
        int mb = mapObj["mouseButton"].toInt();
        QString disc = mapObj["discriminator"].toString("NothingMapping");
        p.setSelectedMapping(mb, disc);
        if (disc == "SimulatedKeystroke") {
            p.updateSimulatedKeys(
                mb,
                mapObj["keys"].toString(),
                mapObj["simType"].toInt(4),
                mapObj["delay"].toInt(33),
                mapObj["rand"].toBool(false),
                mapObj["block"].toBool(false)
            );
        }
    }

    Database::instance().saveProfile(p);
    m_profiles.append(p);
    m_currentProfileIndex = m_profiles.size() - 1;
    populateProfilesList();
    setDirty(true);
}

void MainWindow::onOpenSettings() {
    GlobalSettingsDialog dlg(m_settings, this);
    if (dlg.exec() == QDialog::Accepted) {
        m_settings = dlg.getSettings();
        Database::instance().saveSettings(m_settings);
        applyTheme(m_settings.theme);

        Localization::instance().setLanguage(m_settings.language);
        updateTrayMenu();
    }
}

void MainWindow::onApplyChanges() {
    Database::instance().saveAllProfiles(m_profiles);
    setDirty(false);
    qDebug() << "[GUI] Saved profiles to SQLite DB. Daemon will auto-reload.";
}

void MainWindow::applyTheme(int themeIndex) {
    switch (themeIndex) {
        case 0: { // System (KDE Breeze native)
            qApp->setStyleSheet("");
            qApp->setStyle(QStyleFactory::create("Breeze") ? QStyleFactory::create("Breeze") : QStyleFactory::create("Fusion"));
            qApp->setPalette(qApp->style()->standardPalette());
            break;
        }
        case 1: { // Modern Dark (with deep rich black sidebar as user liked)
            qApp->setStyle(QStyleFactory::create("Fusion"));
            QString darkQss = R"(
                QWidget {
                    background-color: #1f2226;
                    color: #eff0f1;
                    font-size: 10pt;
                }
                QListWidget {
                    background-color: #121416;
                    border: 1px solid #2d3238;
                    border-radius: 6px;
                    padding: 4px;
                }
                QListWidget::item {
                    padding: 6px 10px;
                    margin: 2px 0px;
                    border-radius: 4px;
                    color: #e0e4e8;
                }
                QListWidget::item:selected {
                    background-color: #3daee9;
                    color: #ffffff;
                    font-weight: 500;
                }
                QListWidget::item:hover:!selected {
                    background-color: #22262b;
                }
                QTabWidget::pane {
                    background-color: #1b1e21;
                    border: 1px solid #2d3238;
                    border-radius: 6px;
                    top: -1px;
                }
                QTabBar::tab {
                    background-color: #24282d;
                    border: 1px solid #2d3238;
                    border-bottom: none;
                    border-top-left-radius: 6px;
                    border-top-right-radius: 6px;
                    padding: 7px 16px;
                    margin-right: 2px;
                    color: #a0a6ad;
                }
                QTabBar::tab:selected {
                    background-color: #1b1e21;
                    border-top: 2px solid #3daee9;
                    color: #ffffff;
                    font-weight: 500;
                }
                QTabBar::tab:hover:!selected {
                    background-color: #2c3137;
                    color: #e0e4e8;
                }
                QLineEdit, QSpinBox, QComboBox {
                    background-color: #141618;
                    border: 1px solid #323840;
                    border-radius: 5px;
                    padding: 5px 10px;
                    color: #eff0f1;
                    min-height: 22px;
                }
                QLineEdit:focus, QSpinBox:focus, QComboBox:focus {
                    border: 1px solid #3daee9;
                    background-color: #17191c;
                }
                QComboBox::drop-down {
                    border: none;
                    width: 24px;
                }
                QPushButton {
                    background-color: #282c31;
                    border: 1px solid #383e46;
                    border-radius: 5px;
                    padding: 5px 12px;
                    min-height: 22px;
                    color: #eff0f1;
                    font-weight: 500;
                }
                QPushButton:hover {
                    background-color: #33383f;
                    border-color: #3daee9;
                }
                QPushButton:pressed {
                    background-color: #1e2125;
                }
                QPushButton:disabled {
                    background-color: #181a1d;
                    border-color: #222529;
                    color: #555b62;
                }
                /* Distinct, visible Gear Button */
                QPushButton[gear="true"] {
                    background-color: #2a2e34;
                    border: 1px solid #3e454f;
                    border-radius: 4px;
                    padding: 2px;
                }
                QPushButton[gear="true"]:hover {
                    background-color: #383e46;
                    border-color: #3daee9;
                }
                QTextEdit {
                    background-color: #141618;
                    border: 1px solid #2d3238;
                    border-radius: 5px;
                    color: #d0d5db;
                }
            )";
            qApp->setStyleSheet(darkQss);
            break;
        }
        case 2: { // Modern Light (Clean White Fluent)
            qApp->setStyle(QStyleFactory::create("Fusion"));
            QString lightQss = R"(
                QWidget {
                    background-color: #f7f9fa;
                    color: #1f2328;
                    font-size: 10pt;
                }
                QListWidget {
                    background-color: #ffffff;
                    border: 1px solid #d0d7de;
                    border-radius: 6px;
                    padding: 4px;
                }
                QListWidget::item {
                    padding: 6px 10px;
                    margin: 2px 0px;
                    border-radius: 4px;
                    color: #24292f;
                }
                QListWidget::item:selected {
                    background-color: #0969da;
                    color: #ffffff;
                    font-weight: 500;
                }
                QListWidget::item:hover:!selected {
                    background-color: #f3f4f6;
                }
                QTabWidget::pane {
                    background-color: #ffffff;
                    border: 1px solid #d0d7de;
                    border-radius: 6px;
                    top: -1px;
                }
                QTabBar::tab {
                    background-color: #eaeef2;
                    border: 1px solid #d0d7de;
                    border-bottom: none;
                    border-top-left-radius: 6px;
                    border-top-right-radius: 6px;
                    padding: 7px 16px;
                    margin-right: 2px;
                    color: #57606a;
                }
                QTabBar::tab:selected {
                    background-color: #ffffff;
                    border-top: 2px solid #0969da;
                    color: #24292f;
                    font-weight: 500;
                }
                QTabBar::tab:hover:!selected {
                    background-color: #f0f3f6;
                }
                QLineEdit, QSpinBox, QComboBox {
                    background-color: #ffffff;
                    border: 1px solid #d0d7de;
                    border-radius: 5px;
                    padding: 5px 10px;
                    color: #1f2328;
                    min-height: 22px;
                }
                QLineEdit:focus, QSpinBox:focus, QComboBox:focus {
                    border: 1px solid #0969da;
                }
                QComboBox::drop-down {
                    border: none;
                    width: 24px;
                }
                QPushButton {
                    background-color: #f6f8fa;
                    border: 1px solid #d0d7de;
                    border-radius: 5px;
                    padding: 5px 12px;
                    min-height: 22px;
                    color: #24292f;
                    font-weight: 500;
                }
                QPushButton:hover {
                    background-color: #f3f4f6;
                    border-color: #0969da;
                }
                QPushButton:pressed {
                    background-color: #ebecf0;
                }
                QPushButton:disabled {
                    background-color: #f6f8fa;
                    border-color: #e1e4e8;
                    color: #8c959f;
                }
                QPushButton[gear="true"] {
                    background-color: #ffffff;
                    border: 1px solid #d0d7de;
                    border-radius: 4px;
                    padding: 2px;
                }
                QPushButton[gear="true"]:hover {
                    background-color: #f3f4f6;
                    border-color: #0969da;
                }
                QTextEdit {
                    background-color: #ffffff;
                    border: 1px solid #d0d7de;
                    border-radius: 5px;
                    color: #24292f;
                }
            )";
            qApp->setStyleSheet(lightQss);
            break;
        }
        case 3: { // AMOLED Black (Pure Black with crisp, highly-visible buttons)
            qApp->setStyle(QStyleFactory::create("Fusion"));
            QString blackQss = R"(
                QWidget {
                    background-color: #000000;
                    color: #ffffff;
                    font-size: 10pt;
                }
                QListWidget {
                    background-color: #050505;
                    border: 1px solid #222222;
                    border-radius: 6px;
                    padding: 4px;
                }
                QListWidget::item {
                    padding: 6px 10px;
                    margin: 2px 0px;
                    border-radius: 4px;
                    color: #eeeeee;
                }
                QListWidget::item:selected {
                    background-color: #0078d4;
                    color: #ffffff;
                    font-weight: 600;
                }
                QListWidget::item:hover:!selected {
                    background-color: #161616;
                }
                QTabWidget::pane {
                    background-color: #080808;
                    border: 1px solid #252525;
                    border-radius: 6px;
                    top: -1px;
                }
                QTabBar::tab {
                    background-color: #141414;
                    border: 1px solid #282828;
                    border-bottom: none;
                    border-top-left-radius: 6px;
                    border-top-right-radius: 6px;
                    padding: 7px 16px;
                    margin-right: 2px;
                    color: #888888;
                }
                QTabBar::tab:selected {
                    background-color: #080808;
                    border-top: 2px solid #0078d4;
                    color: #ffffff;
                    font-weight: 600;
                }
                QTabBar::tab:hover:!selected {
                    background-color: #1e1e1e;
                    color: #ffffff;
                }
                QLineEdit, QSpinBox, QComboBox {
                    background-color: #101010;
                    border: 1px solid #333333;
                    border-radius: 5px;
                    padding: 5px 10px;
                    color: #ffffff;
                    min-height: 22px;
                }
                QLineEdit:focus, QSpinBox:focus, QComboBox:focus {
                    border: 1px solid #0078d4;
                    background-color: #141414;
                }
                QComboBox::drop-down {
                    border: none;
                    width: 24px;
                }
                /* Buttons in AMOLED - clearly framed and high contrast */
                QPushButton {
                    background-color: #1c1c1c;
                    border: 1px solid #383838;
                    border-radius: 5px;
                    padding: 5px 12px;
                    min-height: 22px;
                    color: #ffffff;
                    font-weight: 500;
                }
                QPushButton:hover {
                    background-color: #282828;
                    border-color: #0078d4;
                }
                QPushButton:pressed {
                    background-color: #101010;
                }
                QPushButton:disabled {
                    background-color: #0c0c0c;
                    border-color: #1a1a1a;
                    color: #444444;
                }
                QPushButton[gear="true"] {
                    background-color: #222222;
                    border: 1px solid #444444;
                    border-radius: 4px;
                    padding: 2px;
                }
                QPushButton[gear="true"]:hover {
                    background-color: #303030;
                    border-color: #0078d4;
                }
                QTextEdit {
                    background-color: #080808;
                    border: 1px solid #252525;
                    border-radius: 5px;
                    color: #e0e0e0;
                }
            )";
            qApp->setStyleSheet(blackQss);
            break;
        }
        case 4: { // Fusion
            qApp->setStyleSheet("");
            qApp->setStyle(QStyleFactory::create("Fusion"));
            qApp->setPalette(qApp->style()->standardPalette());
            break;
        }
    }
}

void MainWindow::setupTray() {
    if (!QSystemTrayIcon::isSystemTrayAvailable()) {
        return;
    }

    m_trayIcon = new QSystemTrayIcon(QIcon(":/mouse.png"), this);
    m_trayIcon->setToolTip("RMouseButtonControl");

    m_trayMenu = new QMenu(this);
    connect(m_trayMenu, &QMenu::aboutToShow, this, &MainWindow::updateTrayMenu);

    updateTrayMenu();

    m_trayIcon->setContextMenu(m_trayMenu);

    connect(m_trayIcon, &QSystemTrayIcon::activated, this, [this](QSystemTrayIcon::ActivationReason reason) {
        if (reason == QSystemTrayIcon::Trigger || reason == QSystemTrayIcon::DoubleClick) {
            if (isVisible() && !isMinimized()) {
                hide();
            } else {
                showNormal();
                activateWindow();
                raise();
            }
        }
    });

    m_trayIcon->show();
}

void MainWindow::updateTrayMenu() {
    if (!m_trayMenu) return;
    m_trayMenu->clear();

    // 1. Setup window (opens GUI configurator)
    QAction* setupAction = m_trayMenu->addAction(QIcon(":/mouse.png"), Tr("Tray_Setup"));
    connect(setupAction, &QAction::triggered, this, [this]() {
        showNormal();
        activateWindow();
        raise();
    });

    m_trayMenu->addSeparator();

    // 2. GUI Selector Submenu
    QMenu* guiMenu = m_trayMenu->addMenu(Tr("Tray_Interface"));

    QAction* openQtAction = guiMenu->addAction("Открыть Qt интерфейс");
    connect(openQtAction, &QAction::triggered, this, [this]() {
        showNormal();
        activateWindow();
        raise();
    });

    QAction* openCSharpAction = guiMenu->addAction("Открыть C# Avalonia интерфейс");
    connect(openCSharpAction, &QAction::triggered, this, [this]() {
        launchCSharpGui();
    });

    guiMenu->addSeparator();

    QActionGroup* guiGroup = new QActionGroup(guiMenu);
    QAction* defQtAction = guiMenu->addAction("По умолчанию: Qt");
    defQtAction->setCheckable(true);
    defQtAction->setChecked(m_settings.preferredGui != "csharp" && m_settings.preferredGui != "avalonia");
    guiGroup->addAction(defQtAction);
    connect(defQtAction, &QAction::triggered, this, [this]() {
        m_settings.preferredGui = "qt";
        Database::instance().saveSettings(m_settings);
    });

    QAction* defCSharpAction = guiMenu->addAction("По умолчанию: C# Avalonia");
    defCSharpAction->setCheckable(true);
    defCSharpAction->setChecked(m_settings.preferredGui == "csharp" || m_settings.preferredGui == "avalonia");
    guiGroup->addAction(defCSharpAction);
    connect(defCSharpAction, &QAction::triggered, this, [this]() {
        m_settings.preferredGui = "csharp";
        Database::instance().saveSettings(m_settings);
    });

    m_trayMenu->addSeparator();

    // 3. Autostart checkbox
    bool isAutostart = GlobalSettingsDialog::isAutostartEnabled();
    QAction* autostartAction = m_trayMenu->addAction(Tr("Tray_RunAtStartup"));
    autostartAction->setCheckable(true);
    autostartAction->setChecked(isAutostart);
    connect(autostartAction, &QAction::toggled, this, [](bool checked) {
        GlobalSettingsDialog::setAutostartEnabled(checked);
    });

    m_trayMenu->addSeparator();

    // 4. Profiles Submenu
    QString profilesTitle = QString("%1 (%2)").arg(Tr("Tray_Profiles")).arg(m_profiles.size());
    QMenu* profilesMenu = m_trayMenu->addMenu(profilesTitle);

    for (int i = 0; i < m_profiles.size(); ++i) {
        const auto& p = m_profiles[i];
        QString title = p.name;
        if (p.isDefault) {
            title += " (Default)";
        }
        QMenu* pSub = profilesMenu->addMenu(title);

        QAction* selectAction = pSub->addAction("Выбрать профиль");
        connect(selectAction, &QAction::triggered, this, [this, i]() {
            showNormal();
            activateWindow();
            raise();
            if (i >= 0 && i < m_profiles.size()) {
                m_currentProfileIndex = i;
                displayCurrentProfile();
                if (m_profilesList && i < m_profilesList->count()) {
                    m_profilesList->setCurrentRow(i);
                }
            }
        });
        pSub->addSeparator();

        bool hasMappings = false;
        static const char* btnNames[] = {
            "Кнопка 1 (Left)", "Кнопка 2 (Right)", "Кнопка 3 (Middle)",
            "Кнопка 4 (XButton 1)", "Кнопка 5 (XButton 2)",
            "Колесо вверх", "Колесо вниз", "Наклон влево", "Наклон вправо"
        };
        for (int mb = 0; mb < 9; ++mb) {
            ButtonMappingModel bm = p.getSelectedMapping(mb);
            if (!bm.discriminator.isEmpty() && bm.discriminator != "NothingMapping") {
                QString desc = QString("  • %1: %2")
                    .arg(btnNames[mb])
                    .arg(getActionDisplayName(bm.discriminator, bm.keys));
                QAction* mapAct = pSub->addAction(desc);
                mapAct->setEnabled(false);
                hasMappings = true;
            }
        }
        if (!hasMappings) {
            QAction* noAct = pSub->addAction("  (Нет переназначений)");
            noAct->setEnabled(false);
        }
    }

    // 5. Reload Profiles
    QAction* reloadAction = m_trayMenu->addAction(Tr("Tray_ReloadProfiles"));
    connect(reloadAction, &QAction::triggered, this, [this]() {
        loadData();
    });

    m_trayMenu->addSeparator();

    // 6. Exit
    QAction* exitAction = m_trayMenu->addAction(Tr("Tray_Exit"));
    connect(exitAction, &QAction::triggered, this, [this]() {
        m_forceQuit = true;
        close();
        QApplication::quit();
    });
}

void MainWindow::launchCSharpGui() {
    QStringList candidates = {
        QCoreApplication::applicationDirPath() + "/csharp-gui/YMouseButtonControl.exe",
        QCoreApplication::applicationDirPath() + "/YMouseButtonControl.exe",
        QDir::homePath() + "/.local/bin/ymouse-settings",
        QDir::homePath() + "/.local/lib/ymouse/YMouseButtonControl",
        "ymouse-settings",
        "YMouseButtonControl"
    };
    for (const QString& path : candidates) {
        if (QFile::exists(path)) {
            QProcess::startDetached(path, {"--settings"});
            return;
        }
    }
    QProcess::startDetached("ymouse-settings", {"--settings"});
}

QString MainWindow::getActionDisplayName(const QString& discriminator, const QString& keys) {
    if (discriminator == "DisabledMapping") {
        return Tr("Map_Disabled");
    } else if (discriminator == "SimulatedKeystroke") {
        return keys.isEmpty() ? Tr("Map_SimulatedUndefined") : QString("%1 (%2)").arg(Tr("Map_SimulatedUndefined"), keys);
    } else if (discriminator == "RightClick") {
        return Tr("Map_RightClick");
    }
    return Tr("Map_NoChange");
}

void MainWindow::closeEvent(QCloseEvent *event) {
    if (m_forceQuit || !m_trayIcon || !m_trayIcon->isVisible()) {
        event->accept();
    } else {
        hide();
        event->ignore();
    }
}
