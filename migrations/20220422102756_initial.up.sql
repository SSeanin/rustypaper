-- Add up migration script here
CREATE TABLE IF NOT EXISTS "user"
(
    user_id         uuid UNIQUE              NOT NULL DEFAULT gen_random_uuid(),
    first_name      VARCHAR(255)             NOT NULL,
    last_name       VARCHAR(255)             NOT NULL,
    email           VARCHAR (255)            NOT NULL UNIQUE,
    password        VARCHAR (2048)           NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP WITH TIME ZONE NOT NULL,
                                  PRIMARY KEY (user_id)
    );

CREATE TABLE IF NOT EXISTS "post"
(
    post_id      uuid UNIQUE              NOT NULL DEFAULT gen_random_uuid(),
    title        VARCHAR(255)             NOT NULL,
    content      TEXT                     NOT NULL,
    shortcode    CHAR(20)                 NOT NULL UNIQUE,
    is_published BOOLEAN                  NOT NULL DEFAULT true,
    author_id    uuid REFERENCES "user"(user_id)   NOT NULL,
    created_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (post_id)
);
