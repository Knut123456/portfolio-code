DROP DATABASE IF EXISTS PORTFOLIO;
CREATE DATABASE PORTFOLIO;

USE PORTFOLIO;

CREATE TABLE projects (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    img_url VARCHAR(2083) NOT NULL,
    link_url VARCHAR(2083),
    description TEXT
);
