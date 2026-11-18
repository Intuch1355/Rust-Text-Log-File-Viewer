CREATE TABLE file_records (
    id INT AUTO_INCREMENT PRIMARY KEY,
    file_name VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    extension VARCHAR(50),
    file_size BIGINT UNSIGNED NOT NULL,
    total_lines INT UNSIGNED NOT NULL,
    saved_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE search_logs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    file_id INT NOT NULL,
    keyword VARCHAR(100) NOT NULL,
    match_count INT UNSIGNED NOT NULL,
    searched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (file_id) REFERENCES file_records(id)
);
