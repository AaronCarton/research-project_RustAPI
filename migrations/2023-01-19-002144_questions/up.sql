--
-- Table structure for table `question`
--
CREATE TABLE `questions` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `question` varchar(150) NOT NULL,
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
    ),
    (
        10,
        'Which Muppet is portrayed with uncovered human hands?',
        'Dr. Teeth,Sam Eagle,Swedish Chef,Beaker',
        'Swedish Chef'
    ),
    (
        11,
        'Before starring on The Muppet Show, which Muppet first appeared in commercials for Purina?',
        'Fozzie Bear,Rowlf,Gonzo,Scooter',
        'Rowlf'
    ),
    (
        12,
        'Which of these stars cowrote the Muppet movie they starred in?',
        'Tina Fey,Tim Curry,Micheal Caine,Jason Segel',
        'Jason Segel'
    ),
    (
        13,
        'What EGOT (Emmy, Grammy, Oscar, and Tony) winner won an Emmy for guest hosting The Muppet Show?',
        'Whoopi Goldberg,Rita Moreno,Mel Brooks,Audrey Hepburn',
        'Rita Moreno'
    ),
    (
        14,
        'Which Muppet said about his act, "Well, you gotta have sole. And if you cant get sole, use halibut \"?',
        'Sweetums,Rizzo the Rat,Lew Zealand,Pepe the King Prawn',
        'Lew Zealand'
    ),
    (
        15,
        'Veins look blue because the unoxygenated blood in them is blue.',
        'true,false',
        'false'
    ),
    (
        16,
        'Human embryos have a tail for several weeks during gestation.',
        'true,false',
        'true'
    ),
    (
        17,
        'Because its cells are dense with energy-generating mitochondria, which of your muscles never tires?',
        'triceps,tongue,gluteus maximus,heart',
        'heart'
    ),
    (
        18,
        'Which of these body parts continues to get bigger with age?',
        'spine,thumb,ear,femur',
        'ear'
    ),
    (
        19,
        'Logical thinkers are left-brain dominant, while artistic people are right-brain dominant.',
        'true,false',
        'false'
    ),
    (
        20,
        'Stress can cause hair to turn gray.',
        'true,false',
        'true'
    ),
    (
        21,
        'Where are red blood cells created?',
        'heart,bones,brain,spleen',
        'bones'
    ),
    (
        22,
        '\"The funny bone\" is actually what type of anatomical part?',
        'organ,bone,nerve,muscle',
        'nerve'
    ),
    (
        23,
        'Which of these body parts has the most bones?',
        'skull,hand,rib cage,foot',
        'hand'
    ),
    (
        24,
        'Lunulae are small whitish half circles visible on what body part?',
        'fingernails,elbows,kneecaps,nose',
        'fingernails'
    ),
    (
        25,
        'The paper-thin tympanic membrane can be found in what body part?',
        'lung,colon,throat,ear',
        'ear'
    ),
    (
        26,
        'Pronounced either soft or hard, the G in GIF stands for what word?',
        'gesture,google,goofy,graphics',
        'graphics'
    ),
    (
        27,
        'Describing the U.S. Navy’s special operations force, the A in SEALs is a substitute for what word?',
        'adept,aquatic,accuracy,air',
        'air'
    ),
    (
        28,
        'What word does the letter b in scuba refer to?',
        'breathing,beneath,bubbling,bulky',
        'breathing'
    ),
    (
        29,
        'What word does the S stand for in NASA?',
        'sensitive,science,space,special',
        'space'
    ),
    (
        30,
        'Surprisingly, the care in care package began as an acronym. What word does the c stand for?',
        'cooperative,compact,catastrophe,chocolate',
        'cooperative'
    ),
    (
        31,
        'The T in SWAT is a shortening of what word?',
        'trained,technical,tenacious,tactics',
        'tactics'
    ),
    (
        32,
        'In CAPTCHA, the name of a test used by websites to make sure users are not robots, the T stands for what word?',
        'topical,turing,timed,tableau',
        'turing'
    ),
    (
        33,
        'What does the SO in SONAR stand for?',
        'solid,sound,source,soft',
        'sound'
    ),
    (
        34,
        'What word is represented by the P in ASAP?',
        'possible,procrastinate,purpose,prevent',
        'possible'
    ),
    (
        35,
        'What word does the d in radar stand for?',
        'distinguishing,dinging,distributing,detecting',
        'detecting'
    );