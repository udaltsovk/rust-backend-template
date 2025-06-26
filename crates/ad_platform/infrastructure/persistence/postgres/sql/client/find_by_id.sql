SELECT id, login, age, gender AS "gender: StoredClientGender", location
FROM clients
WHERE id = $1