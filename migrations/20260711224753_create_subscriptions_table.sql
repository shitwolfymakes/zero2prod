-- migrations/20260711224753_create_subscriptions_table.sql
-- Create Subscriptions table
CREATE TABLE subscriptions(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestampz NOT NULL,
);
