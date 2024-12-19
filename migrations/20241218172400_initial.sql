CREATE TYPE PaymentMethod AS ENUM ('Cash', 'Credit');
CREATE TYPE StaffRole AS ENUM ('Admin', 'HumanResources', 'Manager', 'Trainer', 'Cleaner');
CREATE TYPE AccessLevel AS ENUM ('Premium', 'Family', 'EarlyBird', 'Basic');
CREATE TYPE Period AS ENUM ('Yearly', 'Monthly', 'Weekly', 'Custom');

CREATE TABLE IF NOT EXISTS "users" (
  "id" uuid PRIMARY KEY,
  "name" text NOT NULL,
  "surname" text,
  "email" text UNIQUE NOT NULL,
  "password" text NOT NULL,
  "last_login" timestamptz,
  "created_at" timestamptz NOT NULL,
  "updated_at" timestamptz NOT NULL
);

CREATE TABLE IF NOT EXISTS "subscriptions" (
  "id" uuid PRIMARY KEY,
  "access_level" AccessLevel NOT NULL,
  "price" integer NOT NULL,
  "period" Period NOT NULL,
  "custom_period" text
);

CREATE TABLE IF NOT EXISTS "memberships" (
  "id" uuid PRIMARY KEY,
  "owner" uuid NOT NULL REFERENCES "users" ("id"),
  "created_at" timestamptz NOT NULL,
  "expire_at" timestamptz NOT NULL,
  "type" uuid NOT NULL REFERENCES "subscriptions" ("id"),
  "renew" bool NOT NULL,
  "payment_method" PaymentMethod NOT NULL,
  "join_reason" text
);

CREATE TABLE IF NOT EXISTS "tools" (
  "id" uuid PRIMARY KEY,
  "name" text NOT NULL,
  "kind" text NOT NULL
);

CREATE TABLE IF NOT EXISTS "locations" (
  "id" uuid PRIMARY KEY,
  "country" text NOT NULL,
  "city" text NOT NULL,
  "street" text NOT NULL,
  "working_hours" text
);

CREATE TABLE IF NOT EXISTS "tools_at_location" (
  "location" uuid REFERENCES "locations" ("id"),
  "tool" uuid REFERENCES "tools" ("id"),
  "condition" text,
  "description" text,
  PRIMARY KEY ("location", "tool")
);

CREATE TABLE IF NOT EXISTS "staff" (
  "id" uuid PRIMARY KEY,
  "location" uuid NOT NULL REFERENCES "locations" ("id"),
  "user" uuid NOT NULL REFERENCES "users" ("id"),
  "role" StaffRole NOT NULL,
  "hourly_rate" integer NOT NULL,
  "joined_at" timestamptz NOT NULL,
  "left_at" timestamptz
);

CREATE TABLE IF NOT EXISTS "staff_hours" (
  "id" uuid PRIMARY KEY,
  "staff" uuid NOT NULL REFERENCES "staff" ("id"),
  "clocked_in" timestamptz NOT NULL,
  "clocked_out" timestamptz
);

CREATE TABLE IF NOT EXISTS "tasks" (
  "id" uuid PRIMARY KEY,
  "name" text NOT NULL,
  "description" text
);

CREATE TABLE IF NOT EXISTS "staff_tasks" (
  "id" uuid PRIMARY KEY,
  "hours" uuid NOT NULL REFERENCES "staff_hours" ("id"),
  "task" uuid REFERENCES "tasks" ("id"),
  "memo" text,
  "started_at" timestamptz NOT NULL,
  "ended_at" timestamptz
);

CREATE TABLE IF NOT EXISTS "member_visits" (
  "id" uuid PRIMARY KEY,
  "user" uuid NOT NULL REFERENCES "users" ("id"),
  "membership" uuid NOT NULL REFERENCES "memberships" ("id"),
  "location" uuid NOT NULL REFERENCES "locations" ("id"),
  "visit_time" timestamptz NOT NULL,
  "leave_time" timestamptz
);

CREATE TABLE IF NOT EXISTS "notifications" (
  "id" uuid PRIMARY KEY,
  "user" uuid NOT NULL REFERENCES "users" ("id"),
  "title" text NOT NULL,
  "body" text NOT NULL,
  "created_at" timestamptz NOT NULL,
  "read_at" timestamptz
);

CREATE TABLE IF NOT EXISTS "warnings" (
  "id" uuid PRIMARY KEY,
  "notification" uuid NOT NULL REFERENCES "notifications" ("id"),
  "author" uuid NOT NULL REFERENCES "staff" ("id"),
  "reason" text NOT NULL,
  "severity" integer NOT NULL
);
