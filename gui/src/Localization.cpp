#include "Localization.h"
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLocale>
#include <QDebug>

Localization& Localization::instance() {
    static Localization inst;
    return inst;
}

Localization::Localization() {
    loadTranslations();

    QString sysLang = QLocale::system().name().toLower();
    if (sysLang.startsWith("ru")) {
        m_currentLanguage = "ru";
    } else if (sysLang.startsWith("de")) {
        m_currentLanguage = "de";
    } else if (sysLang.startsWith("es")) {
        m_currentLanguage = "es";
    } else if (sysLang.startsWith("fr")) {
        m_currentLanguage = "fr";
    } else if (sysLang.startsWith("uk")) {
        m_currentLanguage = "uk";
    } else if (sysLang.startsWith("pt")) {
        m_currentLanguage = "pt";
    } else if (sysLang.startsWith("zh")) {
        m_currentLanguage = "zh";
    } else {
        m_currentLanguage = "en";
    }
}

void Localization::loadTranslations() {
    QFile file(":/translations.json");
    if (file.open(QIODevice::ReadOnly)) {
        QJsonDocument doc = QJsonDocument::fromJson(file.readAll());
        if (doc.isObject()) {
            m_translations = doc.object();
        }
    }
}

void Localization::setLanguage(const QString& langCode) {
    if (langCode == "system" || langCode.isEmpty()) {
        QString sysLang = QLocale::system().name().toLower();
        if (sysLang.startsWith("ru")) m_currentLanguage = "ru";
        else if (sysLang.startsWith("de")) m_currentLanguage = "de";
        else if (sysLang.startsWith("es")) m_currentLanguage = "es";
        else if (sysLang.startsWith("fr")) m_currentLanguage = "fr";
        else if (sysLang.startsWith("uk")) m_currentLanguage = "uk";
        else if (sysLang.startsWith("pt")) m_currentLanguage = "pt";
        else if (sysLang.startsWith("zh")) m_currentLanguage = "zh";
        else m_currentLanguage = "en";
    } else {
        m_currentLanguage = langCode;
    }
}

QString Localization::currentLanguage() const {
    return m_currentLanguage;
}

QString Localization::tr(const QString& key) const {
    QJsonObject curMap = m_translations[m_currentLanguage].toObject();
    if (curMap.contains(key)) {
        return curMap[key].toString();
    }
    // Fallback to English
    QJsonObject enMap = m_translations["en"].toObject();
    if (enMap.contains(key)) {
        return enMap[key].toString();
    }
    return key;
}

QVector<QPair<QString, QString>> Localization::availableLanguages() const {
    return {
        {"System (По умолчанию системы)", "system"},
        {"Русский (Russian)", "ru"},
        {"English", "en"},
        {"Deutsch (German)", "de"},
        {"Español (Spanish)", "es"},
        {"Français (French)", "fr"},
        {"Українська (Ukrainian)", "uk"},
        {"Português (Portuguese)", "pt"},
        {"简体中文 (Chinese)", "zh"}
    };
}
