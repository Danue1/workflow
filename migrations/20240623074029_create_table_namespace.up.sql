CREATE TABLE
  "namespace" (
    "namespace_id" UUID NOT NULL,
    "name" VARCHAR(64) NOT NULL,
    "display_name" varchar(64) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMPTZ,
    PRIMARY KEY ("namespace_id")
  );

CREATE UNIQUE INDEX "unique__namespace__name" ON "namespace" ("name");
