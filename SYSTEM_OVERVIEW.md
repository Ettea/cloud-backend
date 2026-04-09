# System Overview - Cloud Backend

This document provides a graphical overview of how the Cloud Backend service integrates 
into a complete customer system (e.g., for IoT monitoring).

## Architecture Diagram

The following diagram shows the data flow from IoT devices through the Cloud Backend to 
the dashboard visualization.

```mermaid
graph TD
    subgraph "Customer Site / Field Devices"
        D1[IoT Device 1] -->|POST /telemetry| API
        D2[IoT Device 2] -->|POST /telemetry| API
        D3[IoT Device N] -->|POST /telemetry| API
    end

    subgraph "Cloud Infrastructure"
        API[Cloud Backend<br/>(Rust / Axum)]
        DB[(PostgreSQL)]
        
        API -->|Store| DB
        API -.->|Retrieve| DB
    end

    subgraph "User Interface"
        UI[Customer Dashboard<br/>(Web / Mobile)] -->|GET /telemetry| API
    end

    style API fill:#f9f,stroke:#333,stroke-width:2px
    style DB fill:#77f,stroke:#333,stroke-width:2px
    style UI fill:#7f7,stroke:#333,stroke-width:2px
```

## Component Description

1.  **IoT Devices**: Periodically send telemetry data (JSON) over the internet to the 
2. Cloud Backend.
2.  **Cloud Backend (This Project)**:
    *   Receives data via a REST API (`POST /telemetry`).
    *   Validates the data and stores it securely in the database.
    *   Provides endpoints to retrieve the data again (`GET /telemetry`).
3.  **PostgreSQL Database**: Serves as a long-term storage for all telemetry events.
4.  **Customer Dashboard**: An (optional) frontend application that retrieves data from 
5. the backend and visualizes it for the end user (e.g., graphs, tables).

## Health Monitoring

Administrators can monitor the system status at any time via the `/health` endpoint to 
ensure both the service and the database connection are active.
