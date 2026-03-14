-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
        '9f9c9b0c-1b2a-4b3c-4d5e-6f7a8b9c0a1b',
        'admin',
        '$argon2id$v=19$m=15000,t=2,p=1$YFX5dDKCjOSX63awWLLECg$pVWm76ZwY+8LRHeedWqEPiluMvhsGDyCL4zlJrovW6k'
    );