# Text / Log File Viewer — Rust + Iced GUI

## 📖 Project Overview
A desktop utility for analyzing text/log files with a modern GUI. Built in Rust using the Iced framework. Tested and runs on **macOS**, **Linux**, and **Windows 11**. Key features include keyword filtering, file metadata display, activity logging, and MySQL integration.

**Group Members**  
- Intuch Choojaroenchaikit 69011348
-
-

---

## 🌟 Features
- **File Analysis**: Count lines/matches for a keyword in any text file
- **Three Screens**:
  - **Viewer**: Display file content with matching lines highlighted
  - **Details**: Show file metadata (path, size, extension)
  - **Activity**: History of all searches (saved to MySQL)
- **MySQL Integration**: Persists file records and search logs
- **Error Handling**: Graceful handling of invalid paths/MySQL issues

---

## 🛠️ Tech Stack
- **Rust** (stable channel)
- **Iced 0.12** for GUI
- **SQLx** (MySQL driver)
- **Tokio** (async runtime)

---

## 🚀 How to Run

### Prerequisites
- Rust installed (`cargo --version` ≥ 1.70)
- A MySQL server running (either locally or via Docker — see below)

### GUI
```bash
cargo run
```

---

## ⚙️ Database Setup

### Option A — Local MySQL (macOS / Linux / Windows 11)
1. Install and start MySQL (e.g. `brew install mysql` on macOS, or download the installer on Windows 11).
2. Create the database:
   ```sql
   CREATE DATABASE save_log;
   ```
3. Apply the schema:
   ```bash
   mysql -uroot < mysql.sql
   ```
4. Set the connection string in `.env`:
   ```env
   DATABASE_URL=mysql://root:password@127.0.0.1:3306/save_log
   ```
   (Replace `password` with your root password, or leave it empty if your MySQL allows blank root passwords.)

### Option B — MySQL via Docker (recommended, cross-platform)
Start MySQL and phpMyAdmin with one command:
```bash
docker compose up -d
```
This creates a `mysql:8.4` container with the `save_log` database and applies `docker/init.sql` automatically. phpMyAdmin is available at `http://localhost:8080` (user: `root`, password: empty).

Then set the connection string in `.env`:
```env
DATABASE_URL=mysql://root:@127.0.0.1:3306/save_log
```

> The app runs without MySQL too — it just won't persist activity history.

---

## 🧪 Testing
- 5 unit tests in `src/unit_test.rs` (file operations + DB):
  ```bash
  cargo test
  ```

---

## 📸 Screenshots
1. Viewer screen with file content

2. Details screen with metadata

3. Activity screen with search history


---

## 🛤️ Project Structure
```
rust_6767/
├── src/
│   ├── main.rs          # CLI/GUI entry point
│   ├── data.rs          # File reading, line/match counting, MySQL logic
│   ├── ui.rs            # Iced GUI implementation (App, update, view)
│   ├── unit_test.rs     # Unit tests
│   └── screens/         # Alternative GUI screens (viewer, details, activity)
│       ├── mod.rs
│       ├── viewer.rs
│       ├── details.rs
│       └── activity.rs
├── docker/
│   ├── init.sql         # MySQL schema (file_records, search_logs)
│   └── docker-compose.yml # MySQL + phpMyAdmin stack
├── mysql.sql            # Same schema, for manual MySQL setup
├── .env                 # DATABASE_URL
├── Cargo.toml           # Dependencies (iced 0.12, sqlx, tokio)
└── README.md
```

---

## 💡 What We Learned
- Building reactive GUIs with Iced's state/message pattern
- Async MySQL operations in Rust
- Error handling for file/database operations
- Team collaboration via Git
