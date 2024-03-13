-- create users table
CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    username TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    roles TEXT NOT NULL, 
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- populate users table
INSERT INTO users (
        username,
        email,
        password,
        active,
        roles,
        created_at,
        updated_at
    )
VALUES (
        'admin',
        'admin@admin.com',
        -- password: 123, hash(pswd1234pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF)
        '$argon2id$v=19$m=19456,t=2,p=1$4kx2m+rGA/0PwdfpTf76rQ$ulte1snxcbr21N6MMK83iUBmDRB6zhzmN3JVLp8+lz0',
        'true',
        'admin',
        now(),
        now()
    );