--
-- Table structure for table `quiz`
--
CREATE TABLE `quizs` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `name` varchar(25) NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB AUTO_INCREMENT = 2 DEFAULT CHARSET = latin1;

--
-- Dumping data for table `quiz`
--
INSERT INTO
    `quizs` (`id`, `name`)
VALUES
    (1, 'Animal Group Names');