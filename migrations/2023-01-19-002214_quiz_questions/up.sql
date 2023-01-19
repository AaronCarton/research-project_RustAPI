--
-- Table structure for table `quiz_questions`
--
CREATE TABLE `quiz_questions` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `quiz_id` int(11) NOT NULL,
    `question_id` int(11) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `question_id` (`question_id`),
    KEY `quiz_id_key` (`quiz_id`),
    CONSTRAINT `question_id_key` FOREIGN KEY (`question_id`) REFERENCES `question` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `quiz_id_key` FOREIGN KEY (`quiz_id`) REFERENCES `quiz` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB AUTO_INCREMENT = 9 DEFAULT CHARSET = latin1;

--
-- Dumping data for table `quiz_questions`
--
INSERT INTO
    `quiz_questions` (`id`, `quiz_id`, `question_id`)
VALUES
    (1, 1, 9),
    (2, 1, 6),
    (3, 1, 3),
    (4, 1, 5),
    (5, 1, 4),
    (6, 1, 8),
    (7, 1, 2),
    (8, 1, 7);