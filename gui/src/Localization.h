#pragma once

#include <QString>
#include <QJsonObject>
#include <QVector>
#include <QPair>

class Localization {
public:
    static Localization& instance();

    void setLanguage(const QString& langCode);
    QString currentLanguage() const;

    QString tr(const QString& key) const;

    QVector<QPair<QString, QString>> availableLanguages() const;

private:
    Localization();
    void loadTranslations();

    QString m_currentLanguage;
    QJsonObject m_translations;
};

inline QString Tr(const QString& key) {
    return Localization::instance().tr(key);
}
