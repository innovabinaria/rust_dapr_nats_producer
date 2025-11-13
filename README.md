# 🚀 Rust + Dapr + NATS Integration Example

![Build](https://img.shields.io/badge/build-passing-brightgreen)

This project demonstrates how to build a microservice in Rust using Dapr and NATS JetStream for pub/sub messaging.

## 🚀 Technologies Used

- **Rust 2021 Edition**
- **Axum**: Asynchronous web framework.
- **Tokio**: Asynchronous runtime.
- **Serde / Serde JSON**: Data serialization and deserialization.
- **Dapr**: Microservices runtime.
- **NATS**: Message broker for distributed systems.

## 📦 Prerequisites

- Docker
- Dapr CLI
- Rust & Cargo
- PowerShell (for Windows users) Optional
- `nats-box` (for stream and message management)

---

## ▶️ Run with Dapr

First, make sure you have a NATS component file named for example `components/nats-pubsub.yaml` with the following content:

```yaml
apiVersion: dapr.io/v1alpha1
kind: Component
metadata:
  name: nats-pubsub
  namespace: default
spec:
  type: pubsub.jetstream
  version: v1
  metadata:
    - name: natsURL
      value: "nats://localhost:4222"
    - name: name
      value: "mynats"
    - name: maxReconnect
      value: "10"
    - name: reconnectWait
      value: "2s"
    - name: durable
      value: "true"
    - name: queueGroup
      value: "mygroup"
```

## ✅ Steps to Run

### 1. Run NATS with JetStream enabled

```bash
docker run -d --name nats-server -p 4222:4222 -p 8222:8222 nats -js
```

### 2. Launch the NATS CLI Tool (nats-box)

```bash
docker run -it --rm natsio/nats-box
```

### 3. Create a stream inside the container

```bash
nats --server nats://host.docker.internal:4222 stream add my-stream --subjects my-topic --storage file --retention limits
```

### 4. Run the Rust application with Dapr

```bash
dapr run --app-id rust-nats-producer --app-port 3000 --dapr-http-port 3500 --resources-path ./components -- cargo run
```

### 5. Send a message through the application (PowerShell)

```powershell
$body = @{
  id = "1"
  value = "Hello NATS from Dapr and Rust"
} | ConvertTo-Json

Invoke-RestMethod -Method POST `
  -Uri "http://localhost:3000/send" `
  -Body $body `
  -ContentType "application/json"
```

### 6. View messages in NATS (inside the container)

```bash
nats --server nats://host.docker.internal:4222 stream view my-stream
```

## 🧠 Notes

- Make sure `nats-server` is running and reachable from your host using `host.docker.internal` if you're on Docker Desktop (Windows/macOS).
- The Rust application uses Axum as the web framework and Dapr for message publishing.
- The address https://127.0.0.1 is used as the basis for DAPR.

Enjoy building event-driven apps with Rust and Dapr!

Author: Victor Aguayo.
