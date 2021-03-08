-- Create Users Table
CREATE TABLE users(
	   id uuid NOT NULL,
	   PRIMARY KEY (id),
       CPF VARCHAR NOT NULL UNIQUE,
	   name TEXT NOT NULL,
       email TEXT NOT NULL UNIQUE,
       password VARCHAR NOT NULL,
	   balance FLOAT DEFAULT 0.0,
	   register_date timestamptz NOT NULL
);