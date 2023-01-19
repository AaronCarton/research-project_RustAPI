--
-- Table structure for table `user_history`
--
CREATE TABLE `user_history` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `user_id` int(11) NOT NULL,
    `question_id` int(11) NOT NULL,
    `answer` varchar(20) NOT NULL,
    PRIMARY KEY (`id`),
    KEY `history_question_key` (`question_id`),
    KEY `history_user_key` (`user_id`),
    CONSTRAINT `history_question_key` FOREIGN KEY (`question_id`) REFERENCES `question` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT `history_user_key` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB AUTO_INCREMENT = 3 DEFAULT CHARSET = latin1;

--
-- Dumping data for table `user_history`
--
INSERT INTO
    `user_history` (`id`, `user_id`, `question_id`, `answer`)
VALUES
    (1, 1, 2, 'Vultures'),
    (2, 1, 3, 'Bouquet');