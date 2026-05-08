# 📦 REST API Project (Rust + Axum)

A lightweight, modular, and fully async REST API built with **Rust**, **Axum**, and **SQLx**.  
This project demonstrates clean architecture, type‑safe database access, structured error handling, and idiomatic Rust patterns for building modern backend services.

---

## 🚀 Features

- **Axum‑based HTTP server** with clean routing  
- **PostgreSQL** database integration via SQLx (async + compile‑time checked queries)  
- **Layered architecture** (routes → handlers → services → DB)  
- **Centralized error handling** with custom `DBError` enum  
- **Environment‑based configuration** using `.env`  
- **Hot‑reload support** via `bacon` or `cargo-watch`  
- **Fully async** using Tokio runtime  

---

## 🛠 Prerequisites

- **Rust** (latest stable)  
- **Cargo**  
- **PostgreSQL**  
- **SQLx CLI** (optional but recommended)

Install SQLx CLI:

```bash
cargo install sqlx-cli

