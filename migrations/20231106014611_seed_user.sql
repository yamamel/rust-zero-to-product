-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    'eff1dbad-56d2-4b2b-b6c3-62e349fb11fa',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$zFG26EcbrBHM0yNwTNySdg$A+5aQb3FcGCYE7CWuhKxjOoPT57UqHC91dgysmcsVjM'
);