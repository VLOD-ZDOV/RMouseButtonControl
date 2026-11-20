#include "ProcessSelectorDialog.h"
#include "Localization.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QHeaderView>
#include <QFileDialog>
#include <QDir>
#include <QFile>
#include <QTextStream>
#include <QLabel>
#include <QIcon>
#include <QSet>
#include <unistd.h>

ProcessSelectorDialog::ProcessSelectorDialog(QWidget *parent)
    : QDialog(parent)
{
    setupUi();
    refreshProcessList();
}

void ProcessSelectorDialog::setupUi() {
    setWindowTitle(Tr("Proc_Title"));
    setWindowIcon(QIcon(":/mouse.ico"));
    resize(650, 500);

    QVBoxLayout* rootLayout = new QVBoxLayout(this);
    rootLayout->setContentsMargins(12, 12, 12, 12);
    rootLayout->setSpacing(8);

    // Top: Label + Refresh + Filter
    QHBoxLayout* topLayout = new QHBoxLayout();
    QLabel* topLabel = new QLabel(Tr("Proc_SelectRunning"), this);
    QPushButton* refreshBtn = new QPushButton(Tr("Btn_Refresh"), this);
    m_filterEdit = new QLineEdit(this);
    m_filterEdit->setPlaceholderText(Tr("Proc_FilterWatermark"));

    topLayout->addWidget(topLabel);
    topLayout->addStretch();
    topLayout->addWidget(refreshBtn);
    topLayout->addWidget(m_filterEdit);
    rootLayout->addLayout(topLayout);

    connect(refreshBtn, &QPushButton::clicked, this, &ProcessSelectorDialog::refreshProcessList);
    connect(m_filterEdit, &QLineEdit::textChanged, this, &ProcessSelectorDialog::onFilterChanged);

    // Table
    m_table = new QTableWidget(this);
    m_table->setColumnCount(4);
    m_table->setHorizontalHeaderLabels({
        Tr("Proc_ColProcess"),
        Tr("Proc_ColProcessName"),
        Tr("Proc_ColWindowTitle"),
        Tr("Proc_ColFileName")
    });
    m_table->horizontalHeader()->setStretchLastSection(true);
    m_table->horizontalHeader()->setSectionResizeMode(0, QHeaderView::ResizeToContents);
    m_table->horizontalHeader()->setSectionResizeMode(1, QHeaderView::ResizeToContents);
    m_table->setSelectionBehavior(QAbstractItemView::SelectRows);
    m_table->setSelectionMode(QAbstractItemView::SingleSelection);
    m_table->setEditTriggers(QAbstractItemView::NoEditTriggers);
    rootLayout->addWidget(m_table);

    connect(m_table, &QTableWidget::itemSelectionChanged, this, &ProcessSelectorDialog::onTableSelectionChanged);

    // Browse section
    QLabel* browsePrompt = new QLabel(Tr("Proc_OrBrowse"), this);
    rootLayout->addWidget(browsePrompt);

    QHBoxLayout* appLayout = new QHBoxLayout();
    QLabel* appLabel = new QLabel(Tr("Proc_Application"), this);
    appLabel->setFixedWidth(120);
    m_appEdit = new QLineEdit(this);
    QPushButton* browseBtn = new QPushButton("...", this);
    browseBtn->setFixedWidth(40);
    appLayout->addWidget(appLabel);
    appLayout->addWidget(m_appEdit);
    appLayout->addWidget(browseBtn);
    rootLayout->addLayout(appLayout);

    connect(browseBtn, &QPushButton::clicked, this, &ProcessSelectorDialog::onBrowseClicked);

    // Description section
    QHBoxLayout* descLayout = new QHBoxLayout();
    QLabel* descLabel = new QLabel(Tr("Info_Description"), this);
    descLabel->setFixedWidth(120);
    m_descEdit = new QLineEdit(this);
    descLayout->addWidget(descLabel);
    descLayout->addWidget(m_descEdit);
    rootLayout->addLayout(descLayout);

    // Bottom buttons
    QHBoxLayout* bottomLayout = new QHBoxLayout();
    bottomLayout->addStretch();
    QPushButton* okBtn = new QPushButton(Tr("Btn_Ok"), this);
    QPushButton* cancelBtn = new QPushButton(Tr("Btn_Cancel"), this);
    bottomLayout->addWidget(okBtn);
    bottomLayout->addWidget(cancelBtn);
    rootLayout->addLayout(bottomLayout);

    connect(okBtn, &QPushButton::clicked, this, [this]() {
        QString proc = m_appEdit->text().trimmed();
        if (proc.isEmpty()) proc = "*";

        m_result.process = proc;
        m_result.name = proc;
        m_result.description = m_descEdit->text().trimmed().isEmpty() ? proc : m_descEdit->text().trimmed();
        accept();
    });
    connect(cancelBtn, &QPushButton::clicked, this, &QDialog::reject);
}

void ProcessSelectorDialog::refreshProcessList() {
    m_table->setRowCount(0);

    QDir procDir("/proc");
    QStringList pids = procDir.entryList(QDir::Dirs | QDir::NoDotAndDotDot);

    QSet<QString> seen;
    int row = 0;

    for (const QString& pidStr : pids) {
        bool ok = false;
        pidStr.toLongLong(&ok);
        if (!ok) continue;

        QString exeLink = QString("/proc/%1/exe").arg(pidStr);
        char buf[1024];
        ssize_t len = readlink(exeLink.toUtf8().constData(), buf, sizeof(buf) - 1);
        if (len <= 0) continue;
        buf[len] = '\0';
        QString exePath = QString::fromUtf8(buf);

        // Get command line
        QString cmdPath = QString("/proc/%1/cmdline").arg(pidStr);
        QFile cmdFile(cmdPath);
        QString cmdLine;
        if (cmdFile.open(QIODevice::ReadOnly)) {
            QByteArray raw = cmdFile.readAll();
            raw.replace('\0', ' ');
            cmdLine = QString::fromUtf8(raw).trimmed();
        }

        // Determine process name (check for Wine .exe)
        QString procName = QFileInfo(exePath).fileName();
        if (exePath.contains("wine") || exePath.contains("proton")) {
            QStringList tokens = cmdLine.split(' ', Qt::SkipEmptyParts);
            for (const QString& t : tokens) {
                if (t.endsWith(".exe", Qt::CaseInsensitive)) {
                    procName = QFileInfo(t).fileName();
                    break;
                }
            }
        }

        if (seen.contains(procName)) continue;
        seen.insert(procName);

        m_table->insertRow(row);
        m_table->setItem(row, 0, new QTableWidgetItem(procName));
        m_table->setItem(row, 1, new QTableWidgetItem(procName));
        m_table->setItem(row, 2, new QTableWidgetItem(""));
        m_table->setItem(row, 3, new QTableWidgetItem(exePath));
        row++;
    }

    m_table->sortItems(0, Qt::AscendingOrder);
}

void ProcessSelectorDialog::onFilterChanged(const QString& filter) {
    for (int r = 0; r < m_table->rowCount(); ++r) {
        bool match = false;
        for (int c = 0; c < m_table->columnCount(); ++c) {
            auto* item = m_table->item(r, c);
            if (item && item->text().contains(filter, Qt::CaseInsensitive)) {
                match = true;
                break;
            }
        }
        m_table->setRowHidden(r, !match);
    }
}

void ProcessSelectorDialog::onTableSelectionChanged() {
    auto selectedItems = m_table->selectedItems();
    if (selectedItems.isEmpty()) return;

    int row = selectedItems.first()->row();
    QString procName = m_table->item(row, 0)->text();
    m_appEdit->setText(procName);
    m_descEdit->setText(procName);
}

void ProcessSelectorDialog::onBrowseClicked() {
    QString fileName = QFileDialog::getOpenFileName(this, Tr("Proc_Title"), QDir::homePath(), "Applications (*)");
    if (!fileName.isEmpty()) {
        QString baseName = QFileInfo(fileName).fileName();
        m_appEdit->setText(baseName);
        m_descEdit->setText(baseName);
    }
}

SelectedProcessInfo ProcessSelectorDialog::getSelectedProcess() const {
    return m_result;
}
