CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "username" TEXT NOT NULL UNIQUE,
    "email" TEXT UNIQUE,
    "password" TEXT,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
