## Build tools & versions used

Cargo and rust stable - rustc 1.81.0 (eeb90cda1 2024-09-04)

## Steps to run the app

```shell
cargo build && cargo run --release
```

On another terminal:
```shell
curl localhost:3000/nodes
```

## What was the reason for your focus? What problems were you trying to solve? 

The main goal of this project was to demonstrate my ability to build a
robust server in Rust with a DDD-like hexagonal architecture, I
focused on ensuring data was fetched, transformed, and exposed
efficiently and each composing part were isolated on its own.

## How long did you spend on this project?

Approximately 6 hours.

## Did you make any trade-offs for this project? What would you have done differently with more time?


I opted to use bundled SQLite for simplicity's sake, instead of
spinning up Docker containers with PostgreSQL. However, this means
that data is not persistent across server restarts. For a real
project, I would have chosen PostgreSQL for data persistence and used
Nginx to load balance connections to the application.

Since we needed to prepopulate the database with initial data, I chose
to load everything at startup by calling the node's ranking endpoint
once. In a scenario with more data or if repeated calls were needed, I
would dynamically append nodes from the API. This decision was made
because the API is idempotent, and there was no requirement for node
indexing.

SQLite does not natively support async queries, so every call is
synchronous.  However, because SQLite is bundled and runs in memory,
it is very fast and does not pose a bottleneck for this
project. Ideally, queries should run in a separate Tokio task (similar
to green threads) to avoid blocking.

Also besides the name, there are no concept of jobs running on the
background except for the first and only time they run on the server's
startup. There are some libraries that allow the creation of CRON like
jobs(such as
[tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler)
that would query the server for new nodes and persist them on the
database.


## What do you think is the weakest part of your project?

The weakest part might be the simplicity of the database choice and
interaction. There are no indexing of nodes by any column due to the
lack of explicit requirements and SQLite will not be the best choice
of database because it won't scale well.

The project follows a DDD-like hexagonal architecture, though the
domain logic is minimal bacause the primary goal is to fetch and
expose external data, however this structure allows for easy future
expansion, such as adding domain-specific logic or transforming data
more comprehensively before exposure.

## Is there any other information you'd like us to know?

It was a fun challenge, for any questions just hit me up!
