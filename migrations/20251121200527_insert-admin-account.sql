-- Adding an account with username 'admin' and pw 'password'

INSERT INTO account (id, username, pw_hash, name, preferred_language)
VALUES (DEFAULT, 'admin'::varchar(20), '$2a$12$2W3AcX2RnI3ZJSwrvWbar.x6FL.nK63niONl.d.mv39bTG5Ru/E9G'::varchar(72),
        'Administrator'::varchar(80), 'en'::lang);
