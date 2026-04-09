# Cloud Backend

A Rust-based telemetry ingestion and retrieval service for IoT devices.

## Overview

Cloud Backend is a lightweight, high-performance service designed 
to collect telemetry data from remote devices and provide a simple API 
for data retrieval. It is built using the Axum web framework, Tokio runtime, 
and SQLx for robust database interactions with PostgreSQL.

## Features

- **Telemetry Ingestion**: Receive JSON telemetry payloads from devices.
- **Data Persistence**: Store telemetry in a PostgreSQL database with 
- automatic migrations.
- **Retrieval API**: Query telemetry data with optional filtering by device ID.
- **Health Monitoring**: Simple endpoint for service health checks.
- **Structured Logging**: Integrated tracing for observability and debugging.

## Prerequisites

- **Rust**: 2024 edition (v1.85.0 or later recommended).
- **PostgreSQL**: A running instance for data storage.
- **SQLx CLI**: Optional, for database management.

## Getting Started

1.  **Clone the repository**:
    ```bash
    git clone <repository-url>
    cd cloud-backend
    ```

2.  **Configure environment**:
    Create a `.env` file in the root directory:
    ```env
    DATABASE_URL=postgres://user:password@localhost/cloud_backend
    ```

3.  **Run migrations**:
    The application automatically attempts to run migrations on startup if a database 
   connection is available.

4.  **Run the application**:
    ```bash
    cargo run
    ```
    The server will start on `http://127.0.0.1:3000`.

## API Documentation

### Health Check
- **Endpoint**: `GET /health`
- **Description**: Returns the service status.

### Post Telemetry
- **Endpoint**: `POST /telemetry`
- **Description**: Submits telemetry data for a device.
- **Payload**:
  ```json
  {
    "device_id": "string",
    "timestamp": number,
    "payload": object
  }
  ```

### Get Telemetry
- **Endpoint**: `GET /telemetry`
- **Description**: Retrieves telemetry data.
- **Query Parameters**:
    - `device_id` (optional): Filter results by device ID.

## Versioning

Current Version: **1.0.0**
- Initial stable release.
- Core telemetry ingestion and retrieval functionality.
