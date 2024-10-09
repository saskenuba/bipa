What was the reason for your focus? What problems were you trying to solve?

The main goal of this project was to demonstrate my ability to build a robust server in Rust, using clean architecture
principles (DDD-like hexagonal architecture). I focused on ensuring data was fetched, transformed, and exposed
efficiently.

How long did you spend on this project?

Approximately 4-5 hours.

Did you make any trade-offs for this project? What would you have done differently with more time?

I opted to load all the data on server startup instead of appending nodes dynamically. This decision was made due to the
lack of specific node indexing requirements. With more time, I would implement node indexing by alias or public key,
allowing more dynamic interaction with the data.

What do you think is the weakest part of your project?

The weakest part might be the simplicity of the database interaction. There is no indexing of nodes by public key or
alias due to the lack of explicit requirements. Additionally, the startup fetch process could be improved for
large-scale data fetching by introducing more efficient pagination or batching techniques.
Is there any other information you’d like us to know?

The project follows a DDD-like hexagonal architecture, though the domain logic is minimal since the primary goal is to
fetch and expose external data. This structure allows for easy future expansion, such as adding domain-specific logic or
transforming data more comprehensively before exposure.