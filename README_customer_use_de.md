# Benutzerhandbuch für Kunden - Cloud-Backend

Dieses Dokument beschreibt, wie der Cloud-Backend-Dienst für einen Kunden oder in 
einer Produktionsumgebung bereitgestellt und verwendet wird.

## Überblick

Das Cloud-Backend ist ein leistungsstarker Dienst zur Erfassung von Telemetriedaten. 
Er sammelt Daten von IoT-Geräten und bietet eine API für den Datenabruf.

Eine grafische Darstellung der Systemarchitektur finden Sie in der [Systemübersicht](SYSTEM_UEBERSICHT.md).

## Anforderungen für den Kunden

Um diesen Dienst zu nutzen, muss der Kunde über Folgendes verfügen:

1.  **Infrastruktur**:
    *   **Hosting**: Ein Server (Linux empfohlen) oder eine Container-Plattform 
    * (Docker/Kubernetes), um den Dienst auszuführen.
    *   **Datenbank**: Eine PostgreSQL-Instanz (v14+) für die Datenpersistenz.

2.  **Netzwerkkonfiguration**:
    *   Eine öffentlich zugängliche IP-Adresse oder ein Domänenname.
    *   Port **3000** (Standard) muss für den eingehenden Datenverkehr von Geräten geöffnet sein.

3.  **Umgebungsvariablen**:
    *   `DATABASE_URL`: Verbindungszeichenfolge für PostgreSQL 
    * (z. B. `postgres://benutzer:passwort@host:5432/db_name`).
    *   `RUST_LOG`: Protokollierungsebene (empfohlen: `info`).

## Bereitstellungsschritte

1.  **Release-Binary erstellen**:
    Kompilieren Sie das Projekt für die Produktion:
    ```bash
    cargo build --release
    ```
    Die ausführbare Datei befindet sich unter `target/release/cloud-backend`.

2.  **Konfiguration**:
    Erstellen Sie eine `.env`-Datei oder setzen Sie die Umgebungsvariablen direkt auf dem Server.

3.  **Dienst ausführen**:
    Führen Sie die Binärdatei aus. Der Dienst übernimmt automatisch die Erstellung 
4. der Datenbanktabellen (Migrationen) beim ersten Start.
    ```bash
    ./cloud-backend
    ```

## Geräteintegration

Geräte sollten Telemetriedaten über einen HTTP-POST-Aufruf an den Endpunkt `/telemetry` senden.

**Anfrageformat**:
*   **Methode**: `POST`
*   **URL**: `http://<server-adresse>:3000/telemetry`
*   **Content-Type**: `application/json`
*   **Nutzlast (Payload)**:
    ```json
    {
      "device_id": "eindeutige_gerätekennung",
      "timestamp": 1712674400,
      "payload": {
        "temperature": 22.5,
        "humidity": 45
      }
    }
    ```

## Datenabruf

Daten können über die API für die Verwendung in Dashboards oder externen Anwendungen 
abgerufen werden.

**Alle Telemetriedaten abrufen**:
`GET /telemetry`

**Nach Gerät filtern**:
`GET /telemetry?device_id=eindeutige_gerätekennung`

## Wartung & Überwachung

*   **Protokolle (Logs)**: Der Dienst liefert strukturierte Protokolle. Leiten Sie 
* diese zur Überwachung in eine Datei oder einen Protokollierungsdienst um.
*   **Backups**: Stellen Sie sicher, dass die PostgreSQL-Datenbank in regelmäßige Backup-Pläne 
* einbezogen wird.
*   **Zustandsprüfung (Health Check)**: Überwachen Sie `GET /health`, um sicherzustellen,
* dass der Dienst läuft und die Datenbank erreichbar ist.
* 
