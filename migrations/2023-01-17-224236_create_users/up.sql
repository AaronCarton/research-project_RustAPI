-- Your SQL goes here
CREATE TABLE users (
    `id` INT NOT NULL AUTO_INCREMENT,
    `uid` VARCHAR(40) NOT NULL,
    `role` INT NOT NULL DEFAULT '0',
    `username` VARCHAR(20) NOT NULL,
    `score` INT NOT NULL DEFAULT '0',
    PRIMARY KEY (`id`),
    UNIQUE (`uid`)
);