INSERT INTO clients (id, login, age, gender, location)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (id) DO UPDATE SET login    = excluded.login,
                               age      = excluded.age,
                               gender   = excluded.gender,
                               location = excluded.location
RETURNING id, login, age, gender as "gender: StoredClientGender", location