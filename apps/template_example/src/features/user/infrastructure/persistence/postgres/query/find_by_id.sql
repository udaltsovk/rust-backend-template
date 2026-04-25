SELECT
    id,
    name,
    surname,
    email,
    password_hash,
    avatar_url,
    target_settings AS "target_settings: StoredUserTargetSettings"
FROM users
WHERE id = $1
