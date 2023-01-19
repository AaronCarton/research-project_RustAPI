--
-- Table structure for table `user`
--
CREATE TABLE `user` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `uid` varchar(40) NOT NULL,
    `role` int(11) NOT NULL DEFAULT '0',
    `username` varchar(20) NOT NULL,
    `score` int(11) NOT NULL DEFAULT '0',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uid` (`uid`)
) ENGINE = InnoDB AUTO_INCREMENT = 2 DEFAULT CHARSET = latin1;

--
-- Dumping data for table `user`
--
INSERT INTO
    `user` (`id`, `uid`, `role`, `username`, `score`)
VALUES
    (1, 'IwdPPTODpGhiKKVactBVYTflGX13', 0, 'aaron', 4);