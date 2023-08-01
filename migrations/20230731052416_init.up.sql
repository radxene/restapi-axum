-- Add up migration script here
-- (BIGSERIAL | BIGINT | INT8) == 9_223_372_036_854_775_807
CREATE TABLE "authors" (
  "id" bigserial NOT NULL,
  "last_name" VARCHAR(255) NOT NULL,
  "first_name" VARCHAR(255) NOT NULL,
  CONSTRAINT "authors_pkey" PRIMARY KEY ("id")
);
CREATE TABLE "books" (
  "id" bigserial NOT NULL,
  "title" varchar(255) NOT NULL,
  "isbn" varchar(255) NOT NULL,
  "author_id" int8 NOT NULL,
  "published" bool DEFAULT false,
  "created_at" timestamptz NULL DEFAULT now(),
  "updated_at" timestamptz NULL DEFAULT now(),
  CONSTRAINT "books_pkey" PRIMARY KEY ("id"),
  CONSTRAINT "fk_author" FOREIGN KEY("author_id") REFERENCES "authors"("id")
);