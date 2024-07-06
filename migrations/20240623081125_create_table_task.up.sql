CREATE TABLE
  "task_queue" (
    "task_queue_id" UUID NOT NULL,
    "namespace_id" UUID NOT NULL,
    "name" VARCHAR(64) NOT NULL,
    "type" VARCHAR(64) NOT NULL, -- WORKFLOW | ACTIVITY | SYSTEM | BACKGROUND
    "current_task_count" INT NOT NULL DEFAULT 0,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("task_queue_id"),
    FOREIGN KEY ("namespace_id") REFERENCES "namespace" ("namespace_id")
  );

CREATE TABLE
  "task" (
    "task_id" UUID NOT NULL,
    "task_queue_id" UUID NOT NULL,
    "workflow_execution_id" UUID NOT NULL,
    "activity_id" UUID NOT NULL,
    "state" VARCHAR(64) NOT NULL, -- PENDING | IN_PROGRESS | COMPLETED | FAILED | CANCELLED
    "input" JSONB NOT NULL DEFAULT '{}',
    "output" JSONB NOT NULL DEFAULT '{}',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "started_at" TIMESTAMPTZ,
    "ended_at" TIMESTAMPTZ,
    PRIMARY KEY ("task_id"),
    FOREIGN KEY ("task_queue_id") REFERENCES "task_queue" ("task_queue_id"),
    FOREIGN KEY ("workflow_execution_id") REFERENCES "workflow_execution" ("workflow_execution_id"),
    FOREIGN KEY ("activity_id") REFERENCES "activity" ("activity_id")
  );
