# System Architecture

The following diagram illustrates the data flow and infrastructure of the Cloud Backend system.

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
