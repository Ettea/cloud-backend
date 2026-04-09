# Systemübersicht - Cloud-Backend

Dieses Dokument bietet eine grafische Übersicht darüber, wie der Cloud-Backend-Dienst in ein vollständiges Kundensystem (z. B. für IoT-Überwachung) integriert wird.

## Architektur-Diagramm

Das folgende Diagramm zeigt den Datenfluss von den IoT-Geräten über das Cloud-Backend bis hin zur Visualisierung im Dashboard.

```mermaid
graph TD
    subgraph "Kundenstandort / Feldgeräte"
        D1[IoT Gerät 1] -->|POST /telemetry| API
        D2[IoT Gerät 2] -->|POST /telemetry| API
        D3[IoT Gerät N] -->|POST /telemetry| API
    end

    subgraph "Cloud-Infrastruktur"
        API[Cloud-Backend<br/>(Rust / Axum)]
        DB[(PostgreSQL)]
        
        API -->|Speichern| DB
        API -.->|Abrufen| DB
    end

    subgraph "Benutzeroberfläche"
        UI[Kunden-Dashboard<br/>(Web / Mobile)] -->|GET /telemetry| API
    end

    style API fill:#f9f,stroke:#333,stroke-width:2px
    style DB fill:#77f,stroke:#333,stroke-width:2px
    style UI fill:#7f7,stroke:#333,stroke-width:2px
```

## Erläuterung der Komponenten

1.  **IoT-Geräte**: Senden regelmäßig Telemetriedaten (JSON) über das Internet an das Cloud-Backend.
2.  **Cloud-Backend (Dieses Projekt)**:
    *   Empfängt Daten über eine REST-API (`POST /telemetry`).
    *   Validiert die Daten und speichert sie sicher in der Datenbank.
    *   Stellt Endpunkte bereit, um die Daten wieder abzurufen (`GET /telemetry`).
3.  **PostgreSQL-Datenbank**: Dient als langfristiger Speicher für alle Telemetrieereignisse.
4.  **Kunden-Dashboard**: Eine (optionale) Frontend-Anwendung, die die Daten vom Backend abruft und für den Endbenutzer visualisiert (z. B. Graphen, Tabellen).

## Zustandsprüfung (Monitoring)

Administratoren können den Status des Systems jederzeit über den `/health` Endpunkt überwachen, um sicherzustellen, dass sowohl der Dienst als auch die Datenbankverbindung aktiv sind.
