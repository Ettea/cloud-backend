# Customer Usage Guide - Cloud Backend

This document outlines how to deploy and use the Cloud Backend service for a customer 
or in a production environment.

## Overview

The Cloud Backend is a high-performance telemetry ingestion service. It collects data 
from IoT devices and provides an API for retrieval.

A graphical representation of the system architecture can be found in the [System Overview](SYSTEM_OVERVIEW.md).

## Requirements for the Customer

To use this service, the customer needs to provide or have the following:

1.  **Infrastructure**:
    *   **Hosting**: A server (Linux recommended) or a container platform 
    * (Docker/Kubernetes) to run the service.
    *   **Database**: A PostgreSQL (v14+) instance for data persistence.

2.  **Network Configuration**:
    *   A public-facing IP or Domain Name.
    *   Port **3000** (default) must be open for incoming traffic from devices.

3.  **Environment Variables**:
    *   `DATABASE_URL`: Connection string for PostgreSQL 
    * (e.g., `postgres://user:password@host:5432/db_name`).
    *   `RUST_LOG`: Log level (recommended: `info`).

## Deployment Steps

1.  **Build the Release Binary**:
    Compile the project for production:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/cloud-backend`.

2.  **Configuration**:
    Create a `.env` file or set environment variables directly on the server.

3.  **Run the Service**:
    Execute the binary. The service automatically handles database table creation 
4. (migrations) on the first start.
    ```bash
    ./cloud-backend
    ```

## Device Integration

Devices should send telemetry data to the `/telemetry` endpoint using HTTP POST.

**Request Format**:
*   **Method**: `POST`
*   **URL**: `http://<server-address>:3000/telemetry`
*   **Content-Type**: `application/json`
*   **Payload**:
    ```json
    {
      "device_id": "unique_device_identifier",
      "timestamp": 1712674400,
      "payload": {
        "temperature": 22.5,
        "humidity": 45
      }
    }
    ```

## Data Retrieval

Data can be retrieved via the API for use in dashboards or external applications.

**Get All Telemetry**:
`GET /telemetry`

**Filter by Device**:
`GET /telemetry?device_id=unique_device_identifier`

## Maintenance & Monitoring

*   **Logs**: The service provides structured logs. Redirect them to a file or a 
* logging service for monitoring.
*   **Backups**: Ensure the PostgreSQL database is included in regular backup schedules.
*   **Health Check**: Monitor `GET /health` to ensure the service is running and the 
* database is reachable.
