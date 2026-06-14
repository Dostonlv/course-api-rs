# course-api

[![Rust](https://img.shields.io/badge/Rust-2024-orange)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-required-blue)](https://www.postgresql.org/)

Simple Rust API for managing students, instructors, courses, and enrollments.

## Contents

- [Tech Stack](#tech-stack)
- [Run](#run)
- [API Docs](#api-docs)
- [Endpoints](#endpoints)
- [Example Requests](#example-requests)
- [Project Structure](#project-structure)
- [Notes](#notes)

## Tech Stack

- Rust
- Axum
- SQLx
- PostgreSQL

## Run

Install SQLx CLI if you do not have it:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Create a `.env` file:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/course_api
```

Run database migrations:

```bash
sqlx migrate run
```

Start the server:

```bash
cargo run
```

Open:

- API: `http://127.0.0.1:3011`
- Swagger UI: `http://127.0.0.1:3011/swagger-ui`

## API Docs

Swagger UI is available after starting the server:

```text
http://127.0.0.1:3011/swagger-ui
```

OpenAPI JSON:

```text
http://127.0.0.1:3011/api-docs/openapi.json
```

## Endpoints

| Resource | Method | Endpoint |
| --- | --- | --- |
| Students | `GET` | `/students` |
| Students | `POST` | `/students` |
| Students | `GET` | `/students/{id}` |
| Students | `PUT` | `/students/{id}` |
| Students | `DELETE` | `/students/{id}` |
| Instructors | `GET` | `/instructors` |
| Instructors | `POST` | `/instructors` |
| Instructors | `GET` | `/instructors/{id}` |
| Instructors | `PUT` | `/instructors/{id}` |
| Instructors | `DELETE` | `/instructors/{id}` |
| Courses | `GET` | `/courses` |
| Courses | `POST` | `/courses` |
| Courses | `GET` | `/courses/{id}` |
| Courses | `PUT` | `/courses/{id}` |
| Courses | `DELETE` | `/courses/{id}` |
| Enrollments | `GET` | `/enrollments` |
| Enrollments | `POST` | `/enrollments` |
| Enrollments | `GET` | `/enrollments/{id}` |
| Enrollments | `PUT` | `/enrollments/{id}` |
| Enrollments | `DELETE` | `/enrollments/{id}` |

## Example Requests

Create a student:

```bash
curl -X POST http://127.0.0.1:3011/students \
  -H "Content-Type: application/json" \
  -d '{"full_name":"Ali Valiyev","email":"ali@example.com"}'
```

Create an instructor:

```bash
curl -X POST http://127.0.0.1:3011/instructors \
  -H "Content-Type: application/json" \
  -d '{"full_name":"Vali Karimov","email":"vali@example.com","expertise":"Rust"}'
```

Create a course:

```bash
curl -X POST http://127.0.0.1:3011/courses \
  -H "Content-Type: application/json" \
  -d '{"title":"Rust Basics","description":"Intro course","instructor_id":1}'
```

Create an enrollment:

```bash
curl -X POST http://127.0.0.1:3011/enrollments \
  -H "Content-Type: application/json" \
  -d '{"student_id":1,"course_id":1}'
```

## Project Structure

```text
course-api/
├── migrations/
│   └── 01_init.sql
├── src/
│   ├── db/
│   ├── entities/
│   ├── routes/
│   ├── lib.rs
│   └── main.rs
├── Cargo.toml
└── README.md
```

## Notes

- PostgreSQL must be running before starting the app.
- The database from `DATABASE_URL` must already exist.
- Migrations are stored in the `migrations` folder.
- The app listens on `127.0.0.1:3011`.
