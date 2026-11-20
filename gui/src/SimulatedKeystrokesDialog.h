#pragma once

#include <QDialog>
#include <QLineEdit>
#include <QComboBox>
#include <QCheckBox>
#include <QSpinBox>
#include <QToolButton>
#include <QLabel>
#include "Database.h"

class SimulatedKeystrokesDialog : public QDialog {
    Q_OBJECT
public:
    explicit SimulatedKeystrokesDialog(int mouseButton, const ButtonMappingModel& mapping, QWidget *parent = nullptr);

    ButtonMappingModel getMapping() const;

private slots:
    void onInsertTag(const QString& tag);
    void onActionTypeChanged(int index);

private:
    void setupUi();
    void buildTagsMenu();

    int m_mouseButton;
    ButtonMappingModel m_mapping;

    QLineEdit* m_customKeysEdit = nullptr;
    QToolButton* m_tagsButton = nullptr;
    QComboBox* m_actionTypeCombo = nullptr;
    QCheckBox* m_blockInputCheck = nullptr;
    QSpinBox* m_repeatDelaySpin = nullptr;
    QCheckBox* m_randomizeDelayCheck = nullptr;
    QLineEdit* m_descEdit = nullptr;
};
