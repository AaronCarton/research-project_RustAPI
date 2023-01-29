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
    CONSTRAINT `question_id_key` FOREIGN KEY (`question_id`) REFERENCES `questions` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `quiz_id_key` FOREIGN KEY (`quiz_id`) REFERENCES `quizs` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
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
    (8, 1, 7),
    (9, 2, 10),
    (10, 2, 11),
    (11, 2, 12),
    (12, 2, 13),
    (13, 2, 14),
    (14, 3, 15),
    (15, 3, 16),
    (16, 3, 17),
    (17, 3, 18),
    (18, 3, 19),
    (19, 3, 20),
    (20, 3, 21),
    (21, 3, 22),
    (22, 3, 23),
    (23, 3, 24),
    (24, 3, 25),
    (25, 4, 26),
    (26, 4, 27),
    (27, 4, 28),
    (28, 4, 29),
    (29, 4, 30),
    (30, 4, 31),
    (31, 4, 32),
    (32, 4, 33),
    (33, 4, 34),
    (34, 4, 35);