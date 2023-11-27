# TODO Application Documentation

## Overview

This application is a web service built with the Rocket framework to manage tasks. It interfaces with a PostgreSQL database to persist task information, including names and completion statuses.

## Application Components

### Rocket Framework

The web service functionality is powered by the Rocket framework, which handles incoming HTTP requests and routes them to the appropriate handlers.

### SQLx for Database Operations

SQLx provides async support for database interactions, enabling efficient communication with the PostgreSQL database.

### Database Connection Pool

The application maintains a pool of connections to the PostgreSQL database, which optimizes connection reuse and improves performance.

## Modules

- `main.rs`: Sets up the Rocket application, defines routes, and initializes the server.
- `db_connection.rs`: Contains async functions for database operations, such as retrieving task counts and details.

## Endpoints

### GET `/`

Returns a summary of tasks, including counts of pending and completed tasks and details of each task.

**Response:**

```json
{
  "Number of Pending Tasks": "count",
  "Number of Completed Tasks": "count",
  "Tasks": {
    "task_name": "pending_status_boolean",
    ...
  }
}
