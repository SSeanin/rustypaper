-- Add up migration script here
CREATE TABLE IF NOT EXISTS post
(
    post_id      uuid UNIQUE  NOT NULL    DEFAULT gen_random_uuid(),
    title        VARCHAR(255) NOT NULL,
    content      TEXT         NOT NULL,
    shortcode    CHAR(20) UNIQUE,
    is_published BOOLEAN      NOT NULL    DEFAULT true,
    created_at   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (post_id)
);
