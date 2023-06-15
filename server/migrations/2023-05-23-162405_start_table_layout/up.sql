-- Your SQL goes here

CREATE TABLE users
(
    name   VARCHAR(100) PRIMARY KEY NOT NULL,
    bearer VARCHAR(100) UNIQUE
);

CREATE TABLE books
(
    name        VARCHAR(100) NOT NULL,
    user_name   VARCHAR(100) NOT NULL,
    description VARCHAR(1000),
    PRIMARY KEY (name, user_name),
    FOREIGN KEY (user_name) REFERENCES users (name) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE accounts
(
    name        VARCHAR(100) NOT NULL,
    description VARCHAR(1000),
    user_name   VARCHAR(100) NOT NULL,
    book_name   VARCHAR(100) NOT NULL,
    PRIMARY KEY (name, book_name, user_name),
    FOREIGN KEY (book_name) REFERENCES books (name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (user_name) REFERENCES users (name) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE currencies
(
    symbol         VARCHAR(10)  NOT NULL,
    decimal_points INTEGER      NOT NULL,
    description    VARCHAR(1000),
    user_name      VARCHAR(100) NOT NULL,
    book_name      VARCHAR(100) NOT NULL,
    PRIMARY KEY (symbol, book_name, user_name),
    FOREIGN KEY (user_name) REFERENCES users (name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (book_name) REFERENCES books (name) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE transactions
(
    id          BIGINT           NOT NULL,
    time        TIMESTAMP        NOT NULL,
    description VARCHAR(1000),
    book_name   VARCHAR(100)     NOT NULL,
    user_name   VARCHAR(100)     NOT NULL,
    PRIMARY KEY (id, book_name, user_name),
    FOREIGN KEY (book_name) REFERENCES books (name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (user_name) REFERENCES users (name) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE postings
(
    id             BIGINT         NOT NULL,
    transaction_id BIGINT         NOT NULL,
    valuta         TIMESTAMP,
    book_name      VARCHAR(100)     NOT NULL,
    user_name      VARCHAR(100)     NOT NULL,
    account_name   VARCHAR(100)     NOT NULL,
    currency       VARCHAR(10)      NOT NULL,
    amount         INTEGER          NOT NULL,
    PRIMARY KEY (id, transaction_id, book_name, user_name),
    FOREIGN KEY (transaction_id) REFERENCES transactions (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (book_name) REFERENCES books (name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (user_name) REFERENCES users (name) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (currency) REFERENCES currencies (symbol) ON DELETE CASCADE ON UPDATE CASCADE
);