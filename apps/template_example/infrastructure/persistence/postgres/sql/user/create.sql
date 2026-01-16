INSERT INTO users (
    id,
    name,
    surname,
    email,
    password_hash,
    avatar_url,
    target_settings
)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING
    id,
    name,
    surname,
    email,
    password_hash,
    avatar_url,
    target_settings AS "target_settings: StoredUserTargetSettings"
