--
-- Table structure for table `user_history`
--
CREATE TABLE `user_history` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `user_id` int(11) NOT NULL,
    `quiz_id` int(11) NOT NULL,
    `question_id` int(11) NOT NULL,
    `answer` varchar(20) NOT NULL,
    `correct` tinyint(1) NOT NULL DEFAULT '0',
    PRIMARY KEY (`id`),
    KEY `history_quiz_key` (`quiz_id`),
    KEY `history_question_key` (`question_id`),
    KEY `history_user_key` (`user_id`),
    CONSTRAINT `history_quiz_key` FOREIGN KEY (`quiz_id`) REFERENCES `quizs` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `history_question_key` FOREIGN KEY (`question_id`) REFERENCES `questions` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `history_user_key` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB AUTO_INCREMENT = 3 DEFAULT CHARSET = latin1;

--
-- Dumping data for table `user_history`
--
INSERT INTO
    `user_history` (`id`, `user_id`, `quiz_id`, `question_id`, `answer`, `correct`)
VALUES
    (1, 1, 1, 2, 'Vultures', 1),
    (2, 1, 1, 3, 'Bouquet', 0);