CREATE TABLE `person` (
                          `ID` int NOT NULL AUTO_INCREMENT,
                          `name` varchar(45) CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci NOT NULL,
                          `age` int NOT NULL,
                          PRIMARY KEY (`ID`),
                          UNIQUE KEY `ID_UNIQUE` (`ID`)
) ENGINE=InnoDB AUTO_INCREMENT=19 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;-- Your SQL goes here
