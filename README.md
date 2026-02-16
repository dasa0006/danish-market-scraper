# Danish Market Scraper

### High‑Performance Async Scraping for Housing & Art Auction Data

This project is a production‑grade, Rust‑based scraping system designed to collect structured data from the Danish housing market (e.g., Boligsiden, Nybolig) and Danish art auction houses (e.g., Bruun Rasmussen, Lauritz).
It is built for **speed**, **resilience**, and **scalability**, using a modern async Rust stack.

---

## Features

### Property Scraper

- Extracts housing listings from multiple Danish real‑estate platforms
- Normalizes data across different site structures
- Captures price, address, size, rooms, listing URL, and more

### Art Auction Scraper

- Scrapes auction results from major Danish auction houses
- Extracts artist, title, estimates, hammer prices, lot numbers, and auction dates

### Core Capabilities

- Fully async scraping using **Tokio**
- High‑throughput HTTP requests with **Reqwest**
- HTML parsing via **Scraper**
- Robust retry logic, rate limiting, and error handling
- Configurable concurrency per domain
- Structured logging with **tracing**
- SQLite for development, PostgreSQL for production

---

## Tech Stack

### **Language**

- Rust (nightly)

### **Async Runtime**

- Tokio

### **HTTP Client**

- Reqwest (async)

### **HTML Parsing**

- Scraper

### **Concurrency & Orchestration**

- Futures + FuturesUnordered
- Tokio semaphores
- Governor (rate limiting)
- Tower (retry policies)
- chrono
- tokio-cron-scheduler

### **Database**

- SQLite (development)
- PostgreSQL (production)
- SQLx (async, compile‑time checked queries)

### **Logging & Observability**

- tracing
- tracing‑subscriber

### **Configuration**

- dotenvy
- serde
- serde_json
- config

### **Error Handling**

- thiserror
- anyhow
- derive_more

### **Git Hooks & CommitLint**

- husky-rs
- cargo‑commitlint

---

