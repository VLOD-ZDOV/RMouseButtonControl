#pragma once

#include <QDialog>
#include <QTableWidget>
#include <QLineEdit>
#include <QPushButton>

struct SelectedProcessInfo {
    QString name;
    QString process; // e.g. "game.exe" or "firefox"
    QString description;
    QString windowCaption;
};

class ProcessSelectorDialog : public QDialog {
    Q_OBJECT
public:
    explicit ProcessSelectorDialog(QWidget *parent = nullptr);

    SelectedProcessInfo getSelectedProcess() const;

private slots:
    void refreshProcessList();
    void onFilterChanged(const QString& filter);
    void onTableSelectionChanged();
    void onBrowseClicked();

private:
    void setupUi();

    QLineEdit* m_filterEdit = nullptr;
    QTableWidget* m_table = nullptr;
    QLineEdit* m_appEdit = nullptr;
    QLineEdit* m_descEdit = nullptr;

    SelectedProcessInfo m_result;
};
