--
-- Table structure for table `question`
--
CREATE TABLE `questions` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `question` varchar(50) NOT NULL,
    `answers` text NOT NULL,
    `correct_answer` varchar(20) NOT NULL,
    PRIMARY KEY (`id`),
    KEY `correct_answer` (`correct_answer`)
) ENGINE = InnoDB AUTO_INCREMENT = 10 DEFAULT CHARSET = latin1;

--
-- Dumping data for table `question`
--
INSERT INTO
    `questions` (`id`, `question`, `answers`, `correct_answer`)
VALUES
    (
        2,
        'A group of which animals is referred to as a wake?',
        'Vultures,Crocodiles,Bats,Ants',
        'Vultures'
    ),
    (
        3,
        'A congragation of rhinoceroses is known as a:',
        'Bouquet,Crash,Pod,Band',
        'Crash'
    ),
    (
        4,
        'A group of crows is called a:',
        'Murder,Gaggle,Flock,Haunting',
        'Murder'
    ),
    (
        5,
        'A flutter is a group of:',
        'Butterflies,Penguins,Bees,Camels',
        'Butterflies'
    ),
    (
        6,
        'A cluster of jellyfish is called a:',
        'Team,Company,Smack,Muster',
        'Smack'
    ),
    (
        7,
        'An assembly of owls is referred to as a:',
        'Parliament,Pack,Superfluity,Wisdom',
        'Parliament'
    ),
    (
        8,
        'A group of hyenas is known as a:',
        'Troop,Colony,Cackle,Rookery',
        'Cackle'
    ),
    (
        9,
        'A bunch of otters is a:',
        'Pack,Gang,Drove,Romp',
        'Romp'
    );