# Personal Finance Tracker

Welcome to the **Personal Finance Tracker** project! This application helps users manage and track their personal finances with an intuitive interface and robust backend services. Below is an overview of the project's architecture and components.

---

## Table of Contents

- [Frontend](#frontend)
- [Backend](#backend)
- [Dockerization](#dockerization)
- [Getting Started](#getting-started)
- [Future Enhancements](#future-enhancements)

---

## Frontend

### Technology Stack

- **Framework**: [Next.js](https://nextjs.org/)
- **Language**: JavaScript/TypeScript
- **Styling**: Tailwind CSS

### Features

- Responsive design for a seamless user experience across devices.
- Intuitive interface for entering, updating, and viewing financial data.
- Integration with backend APIs for real-time data updates.

### Location

The frontend code is located in the `frontend/finance-tracker` directory.

### Key Highlights

- Optimized for performance and SEO using Next.js features such as static site generation (SSG) and server-side rendering (SSR).
- Communicates with the Rust backend via RESTful APIs.

---

## Backend

### Technology Stack

- **Framework**: [Rust](https://www.rust-lang.org/) with [Actix-web](https://actix.rs/)
- **Database**: PostgreSQL
- **ORM**: SQLx for database interaction

### Features

- RESTful API endpoints for CRUD operations on financial data.

### Key Highlights

- Uses SQLx for type-safe database interactions.
- Modular architecture with `controllers` for endpoints and `services` for business logic.
- Integration with Flyway for database migrations.

---

## Dockerization

### Overview

The application is containerized to simplify deployment and ensure consistency across environments.

### Docker Setup

- **Frontend**: Dockerized to serve the Next.js application via a Node.js runtime.
- **Backend**: Dockerized to run the Rust Actix-web server.
- **Database**: PostgreSQL containerized for local development and production use.

### Key Commands

- **Build Containers**:
    ```bash
# Build the Docker containers
docker-compose build

# Start the containers in detached mode
docker-compose up -d
