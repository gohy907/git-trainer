PRAGMA foreign_keys = ON ;

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    work_name TEXT NOT NULL,
    description TEXT NOT NULL,
    extended_description TEXT NOT NULL,
    status INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    passed INTEGER NOT NULL,
    timestamp TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS attempt_tests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    attempt_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    passed INTEGER NOT NULL,
    FOREIGN KEY (attempt_id) REFERENCES attempts (id) ON DELETE CASCADE
);


INSERT INTO users (
    id,
    username,
    created_at
)
VALUES (
    1,
    'gohy',
    2
);

INSERT INTO tasks (
    id,
    name,
    work_name,
    description,
    extended_description,
    status
)
VALUES (
    1,
    'Привет, мир!',
    'hello-world',
    'В этой задаче Вам предстоит создать новый Git репозиторий и сделать в нём первый коммит.',
    'Давайте начнём с чего-нибудь лёгкого.\n
    Создайте в папке "hello-world" новый Git репозиторий, в котором напишите код на C, выводящий на экран строчку "Hello, World!".\n 
    После этого сделайте ровно один коммит, добавляющий этот код, с названием "Initial commit".',
    0
);


INSERT INTO tasks (
    id,
    name,
    work_name,
    description,
    extended_description,
    status
)
VALUES (
    2,
    'Привет, мир!',
    'hello-world',
    'В этой задаче Вам предстоит создать новый\n
    Git репозиторий и сделать в нём первый коммит.',
    'Давайте начнём с чего-нибудь лёгкого.\n
    Создайте в папке "hello-world" новый Git репозиторий, в котором напишите код на C, выводящий на экран строчку "Hello, World!".\n 
    После этого сделайте ровно один коммит, добавляющий этот код, с названием "Initial commit".',
    0
);

INSERT INTO attempts (
id,
user_id,
task_id,
passed,
timestamp)
VALUES (
1,
1,
1,
1,
1) ;
