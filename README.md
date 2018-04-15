# Rustbox - A rust implementation of PushBox long term storage

## What is it?

This is an internal project. mozilla needs the ability to store large data 
chunks that may not fit perfectly within a standard WebPush message. PushBox
acts as an intermediary store for those chunks. 

Messages are created by Firefox Accounts (FxA), stored here, and then a 
WebPush message containing a URL that points back to this storage is sent
to the User Agent.

The User Agent can then fetch the data, decrypt it and do whatever it needs
to. 

This project, once completed, will eventually replace the AWS Severless 
PushBox project. It's being developed here because serverless can be a bit
greedy about what it grabs, and since PushBox is a rapid prototype, it's
good to treat it in a clean room environment.


## Requirements

The project requires Rust Nightly, a MySQL compliant data store, and 
access to a [Firefox Accounts token verifier](https://github.com/mozilla/fxa-auth-server) system. 


## Setting Up

1) Install Rust Nightly. 
 
The rocket.rs [Getting Started](https://rocket.rs/guide/getting-started/) 
document lists information on how to set that up.

2) create the Rustbox MySQL user and database. 

Because I'm horribly creative and because this is a WIP, I use "`test:test@localhost/pushbox`".
This is not recommended for production use. You can set your preferred 
MySQL access credential information as "database_url" in the `Rocket.toml`
settings file (See [Rocket Config](https://rocket.rs/guide/configuration/#rockettoml)
information.)

3) Run the MySQL migrations `up.sql` file located in `./migrations/*/up.sql`

4) Run `cargo run` to start the application, which (depending on the last commit) may actually start the program. YMMV.

