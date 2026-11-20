#include "SimulatedKeystrokesDialog.h"
#include "Localization.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QGridLayout>
#include <QMenu>
#include <QDialogButtonBox>
#include <QPushButton>
#include <QTextEdit>
#include <QIcon>

// Map between UI index and EF Core enum value:
// 0: MouseButtonPressed (EF 4)
// 1: MouseButtonReleased (EF 5)
// 2: DuringMouse (EF 1)
// 3: InAnotherThreadPressed (EF 2)
// 4: InAnotherThreadReleased (EF 3)
// 5: RepeatedlyWhileButtonDown (EF 6)
// 6: StickyRepeat (EF 8)
// 7: StickyHold (EF 7)
// 8: AsMousePressedAndReleased (EF 0)

static int efEnumToUiIndex(int ef) {
    switch (ef) {
        case 4: return 0;
        case 5: return 1;
        case 1: return 2;
        case 2: return 3;
        case 3: return 4;
        case 6: return 5;
        case 8: return 6;
        case 7: return 7;
        case 0: return 8;
        default: return 0;
    }
}

static int uiIndexToEfEnum(int idx) {
    switch (idx) {
        case 0: return 4;
        case 1: return 5;
        case 2: return 1;
        case 3: return 2;
        case 4: return 3;
        case 5: return 6;
        case 6: return 8;
        case 7: return 7;
        case 8: return 0;
        default: return 4;
    }
}

SimulatedKeystrokesDialog::SimulatedKeystrokesDialog(int mouseButton, const ButtonMappingModel& mapping, QWidget *parent)
    : QDialog(parent), m_mouseButton(mouseButton), m_mapping(mapping)
{
    setupUi();
}

void SimulatedKeystrokesDialog::setupUi() {
    setWindowTitle(Tr("Sk_Title"));
    setWindowIcon(QIcon(":/mouse.ico"));
    setMinimumWidth(650);

    QVBoxLayout* rootLayout = new QVBoxLayout(this);
    rootLayout->setContentsMargins(15, 15, 15, 15);
    rootLayout->setSpacing(10);

    // 1. Enter custom keys label
    QLabel* keysLabel = new QLabel(Tr("Sk_EnterCustomKeys"), this);
    rootLayout->addWidget(keysLabel);

    // 2. Custom keys edit + {} split button
    QHBoxLayout* keysLayout = new QHBoxLayout();
    m_customKeysEdit = new QLineEdit(this);
    m_customKeysEdit->setText(m_mapping.keys);
    keysLayout->addWidget(m_customKeysEdit);

    m_tagsButton = new QToolButton(this);
    m_tagsButton->setText("{}");
    m_tagsButton->setPopupMode(QToolButton::InstantPopup);
    buildTagsMenu();
    keysLayout->addWidget(m_tagsButton);
    rootLayout->addLayout(keysLayout);

    // 3. How to send
    QHBoxLayout* howLayout = new QHBoxLayout();
    QLabel* howLabel = new QLabel(Tr("Sk_HowToSend"), this);
    howLabel->setFixedWidth(300);
    m_actionTypeCombo = new QComboBox(this);
    m_actionTypeCombo->addItem(Tr("Skt_AsPressed"));
    m_actionTypeCombo->addItem(Tr("Skt_AsReleased"));
    m_actionTypeCombo->addItem(Tr("Skt_During"));
    m_actionTypeCombo->addItem(Tr("Skt_ThreadPressed"));
    m_actionTypeCombo->addItem(Tr("Skt_ThreadReleased"));
    m_actionTypeCombo->addItem(Tr("Skt_Repeat"));
    m_actionTypeCombo->addItem(Tr("Skt_StickyRepeat"));
    m_actionTypeCombo->addItem(Tr("Skt_StickyHold"));
    m_actionTypeCombo->addItem(Tr("Skt_AsPressedReleased"));

    m_actionTypeCombo->setCurrentIndex(efEnumToUiIndex(m_mapping.simulatedKeystrokeType));
    howLayout->addWidget(howLabel);
    howLayout->addWidget(m_actionTypeCombo);
    rootLayout->addLayout(howLayout);

    connect(m_actionTypeCombo, QOverload<int>::of(&QComboBox::currentIndexChanged),
            this, &SimulatedKeystrokesDialog::onActionTypeChanged);

    // 4. Block input & Auto repeat delay
    QHBoxLayout* optsLayout = new QHBoxLayout();
    m_blockInputCheck = new QCheckBox(Tr("Sk_BlockInput"), this);
    m_blockInputCheck->setChecked(m_mapping.blockOriginalMouseInput);
    optsLayout->addWidget(m_blockInputCheck);

    optsLayout->addStretch();
    QLabel* delayLabel = new QLabel(Tr("Sk_AutoRepeatDelay"), this);
    optsLayout->addWidget(delayLabel);

    m_repeatDelaySpin = new QSpinBox(this);
    m_repeatDelaySpin->setRange(1, 10000);
    m_repeatDelaySpin->setValue(m_mapping.autoRepeatDelay > 0 ? m_mapping.autoRepeatDelay : 33);
    m_repeatDelaySpin->setSuffix(" ms");
    optsLayout->addWidget(m_repeatDelaySpin);
    rootLayout->addLayout(optsLayout);

    // 5. Randomize delay
    m_randomizeDelayCheck = new QCheckBox(Tr("Sk_RandomizeDelay"), this);
    m_randomizeDelayCheck->setChecked(m_mapping.autoRepeatRandomizeDelayEnabled);
    rootLayout->addWidget(m_randomizeDelayCheck);

    // 6. Informational description box
    QTextEdit* infoBox = new QTextEdit(this);
    infoBox->setReadOnly(true);
    infoBox->setMaximumHeight(140);
    infoBox->setText(
        "Examples of key tags:\n"
        "  {CTRL}c = Ctrl+C (copy)\n"
        "  {LMB} = Left Mouse Button click\n"
        "  {SHIFT}{ENTER} = Shift+Enter\n"
        "  {WAIT:50} = Pause for 50 milliseconds\n"
        "\nSupported special keys: {ESC}, {TAB}, {ENTER}, {SPACE}, {BACKSPACE}, {DEL}, {UP}, {DOWN}, "
        "{LEFT}, {RIGHT}, {F1}..{F12}, {VOLUP}, {VOLDN}, {MUTE}, {LMB}, {RMB}, {MMB}, {MB4}, {MB5}."
    );
    rootLayout->addWidget(infoBox);

    // 7. Description for dropdown
    QHBoxLayout* descLayout = new QHBoxLayout();
    QLabel* descLabel = new QLabel(Tr("Sk_DescriptionDropdown"), this);
    descLabel->setFixedWidth(300);
    m_descEdit = new QLineEdit(this);
    descLayout->addWidget(descLabel);
    descLayout->addWidget(m_descEdit);
    rootLayout->addLayout(descLayout);

    // 8. Buttons OK / Cancel
    QDialogButtonBox* bbox = new QDialogButtonBox(QDialogButtonBox::Ok | QDialogButtonBox::Cancel, this);
    rootLayout->addWidget(bbox);

    connect(bbox, &QDialogButtonBox::accepted, this, [this]() {
        m_mapping.keys = m_customKeysEdit->text();
        m_mapping.simulatedKeystrokeType = uiIndexToEfEnum(m_actionTypeCombo->currentIndex());
        m_mapping.blockOriginalMouseInput = m_blockInputCheck->isChecked();
        m_mapping.autoRepeatDelay = m_repeatDelaySpin->value();
        m_mapping.autoRepeatRandomizeDelayEnabled = m_randomizeDelayCheck->isChecked();
        m_mapping.selected = true;
        m_mapping.discriminator = "SimulatedKeystroke";
        accept();
    });
    connect(bbox, &QDialogButtonBox::rejected, this, &QDialog::reject);

    onActionTypeChanged(m_actionTypeCombo->currentIndex());
}

void SimulatedKeystrokesDialog::onActionTypeChanged(int index) {
    int ef = uiIndexToEfEnum(index);
    // Auto-repeat delay is relevant for RepeatedlyWhileButtonDown (6) and StickyRepeat (8)
    bool isRepeat = (ef == 6 || ef == 8);
    m_repeatDelaySpin->setEnabled(isRepeat);
    m_randomizeDelayCheck->setEnabled(isRepeat);
}

void SimulatedKeystrokesDialog::buildTagsMenu() {
    QMenu* menu = new QMenu(m_tagsButton);

    auto addCategory = [this, menu](const QString& title, const QVector<QPair<QString, QString>>& items) {
        QMenu* sub = menu->addMenu(title);
        for (const auto& item : items) {
            QAction* act = sub->addAction(item.first);
            connect(act, &QAction::triggered, this, [this, tag = item.second]() {
                onInsertTag(tag);
            });
        }
    };

    addCategory(Tr("Sk_MenuModifier"), {
        {"Control", "{CTRL}"},
        {"Right Control", "{RCTRL}"},
        {"Alt", "{ALT}"},
        {"Right Alt", "{RALT}"},
        {"Shift", "{SHIFT}"},
        {"Right Shift", "{RSHIFT}"},
        {"Windows Key", "{LWIN}"},
        {"Right Windows Key", "{RWIN}"}
    });

    addCategory(Tr("Sk_MenuStandard"), {
        {"Escape", "{ESC}"},
        {"Enter", "{ENTER}"},
        {"Tab", "{TAB}"},
        {"Backspace", "{BACKSPACE}"},
        {"Space", "{SPACE}"},
        {"Delete", "{DEL}"},
        {"Insert", "{INS}"},
        {"Caps Lock", "{CAPSLOCK}"}
    });

    addCategory(Tr("Sk_MenuDirection"), {
        {"Up Arrow", "{UP}"},
        {"Down Arrow", "{DOWN}"},
        {"Left Arrow", "{LEFT}"},
        {"Right Arrow", "{RIGHT}"},
        {"Page Up", "{PAGEUP}"},
        {"Page Down", "{PAGEDOWN}"},
        {"Home", "{HOME}"},
        {"End", "{END}"}
    });

    QVector<QPair<QString, QString>> fkeys;
    for (int i = 1; i <= 12; ++i) {
        fkeys.append(qMakePair(QString("F%1").arg(i), QString("{F%1}").arg(i)));
    }
    addCategory(Tr("Sk_MenuFunction"), fkeys);

    QVector<QPair<QString, QString>> numkeys;
    for (int i = 0; i <= 9; ++i) {
        numkeys.append(qMakePair(QString("Numpad %1").arg(i), QString("{NUM%1}").arg(i)));
    }
    numkeys.append(qMakePair(QString("Numpad +"), QString("{NUM+}")));
    numkeys.append(qMakePair(QString("Numpad -"), QString("{NUM-}")));
    numkeys.append(qMakePair(QString("Numpad *"), QString("{NUM*}")));
    numkeys.append(qMakePair(QString("Numpad /"), QString("{NUM/}")));
    addCategory(Tr("Sk_MenuNumeric"), numkeys);

    addCategory(Tr("Sk_MenuMedia"), {
        {"Volume Up", "{VOLUP}"},
        {"Volume Down", "{VOLDN}"},
        {"Mute", "{MUTE}"},
        {"Play / Pause", "{PLAY}"},
        {"Stop", "{STOP}"},
        {"Next Track", "{NEXT}"},
        {"Previous Track", "{PREV}"}
    });

    addCategory(Tr("Sk_MenuBrowser"), {
        {"Back", "{BACK}"},
        {"Forward", "{FORWARD}"},
        {"Refresh", "{REFRESH}"},
        {"Home", "{BROWSERHOME}"}
    });

    addCategory(Tr("Sk_MenuMouse"), {
        {"Left Button", "{LMB}"},
        {"Right Button", "{RMB}"},
        {"Middle Button", "{MMB}"},
        {"Button 4 (XButton1)", "{MB4}"},
        {"Button 5 (XButton2)", "{MB5}"}
    });

    m_tagsButton->setMenu(menu);
}

void SimulatedKeystrokesDialog::onInsertTag(const QString& tag) {
    if (m_customKeysEdit) {
        m_customKeysEdit->insert(tag);
        m_customKeysEdit->setFocus();
    }
}

ButtonMappingModel SimulatedKeystrokesDialog::getMapping() const {
    return m_mapping;
}
