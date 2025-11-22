-- Adding an account with username 'admin' and pw 'password'

INSERT INTO account (id, username, pw_hash, name, preferred_language)
VALUES (DEFAULT, 'admin'::varchar(20), '$2a$14$vr6ztgyotMXrS6vQiGSa9OnXCtYEagXSiFsr0l5xacDJOhC93Bszq'::varchar(72),
        'Administrator'::varchar(80), 'en'::lang);
