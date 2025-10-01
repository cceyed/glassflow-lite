# Planner Agent

## Status

🚧 **Coming Soon**

## Overview

The Planner Agent will break down architecture plans into actionable tasks with dependencies.

## Planned Features

- Task breakdown from architecture
- Dependency analysis
- Implementation order
- Complexity estimation
- Time estimation

## API (Planned)

### IPC Commands
- `planner_create_plan(architecture_plan)`
- `planner_get_tasks()`
- `planner_reorder_tasks(task_ids)`
- `planner_export_plan(path)`

### Events
- `planner:task-created`
- `planner:plan-complete`
- `planner:error`

## Integration

Will receive input from Architect Agent and output to Executor Agent.
