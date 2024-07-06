CREATE TABLE
  "activity" (
    "activity_id" UUID NOT NULL,
    "workflow_execution_id" UUID NOT NULL,
    "name" VARCHAR(64) NOT NULL,
    "state" VARCHAR(64) NOT NULL, -- PENDING | IN_PROGRESS | COMPLETED | FAILED | CANCELLED
    "input" JSONB NOT NULL DEFAULT '{}',
    "output" JSONB NOT NULL DEFAULT '{}',
    "dependencies" JSONB NOT NULL DEFAULT '[]',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "scheduled_at" TIMESTAMPTZ,
    "started_at" TIMESTAMPTZ,
    "ended_at" TIMESTAMPTZ,
    PRIMARY KEY ("activity_id"),
    FOREIGN KEY ("workflow_execution_id") REFERENCES "workflow_execution" ("workflow_execution_id") ON DELETE CASCADE
  );
