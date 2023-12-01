CREATE TABLE IF NOT EXISTS posts (
    post_id SERIAL PRIMARY KEY NOT NULL,
    text VARCHAR(255) NOT NULL,
    parent_id INT REFERENCES posts (post_id),
    likes INT NOT NULL DEFAULT 0
)
