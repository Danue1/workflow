CREATE TABLE
  "workflow" (
    "workflow_id" UUID NOT NULL,
    "namespace_id" UUID NOT NULL,
    "name" VARCHAR(64) NOT NULL,
    "display_name" varchar(64) NOT NULL,
    "cron" VARCHAR(64),
    "input" JSONB NOT NULL DEFAULT '{}',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMPTZ,
    "deactivated_at" TIMESTAMPTZ,
    PRIMARY KEY ("workflow_id"),
    FOREIGN KEY ("namespace_id") REFERENCES "namespace" ("namespace_id")
  );

CREATE UNIQUE INDEX "unique__workflow__namespace_id__name" ON "workflow" ("namespace_id", "name");

CREATE TABLE
  "workflow_execution" (
    "workflow_execution_id" UUID NOT NULL,
    "workflow_id" UUID NOT NULL,
    "state" VARCHAR(64) NOT NULL, -- PENDING | IN_PROGRESS | COMPLETED | FAILED | CANCELLED
    "input" JSONB NOT NULL DEFAULT '{}',
    "output" JSONB NOT NULL DEFAULT '{}',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "started_at" TIMESTAMPTZ,
    "ended_at" TIMESTAMPTZ,
    PRIMARY KEY ("workflow_execution_id"),
    FOREIGN KEY ("workflow_id") REFERENCES "workflow" ("workflow_id")
  );
