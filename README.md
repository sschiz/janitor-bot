# Janitor

[![Newest release on crates.io][crate-version-badge]][crate-link]
[![Project license][crate-license-badge]](LICENSE)

[crate-license-badge]: https://img.shields.io/crates/l/janitor-bot.svg
[crate-link]: https://crates.io/crates/janitor-bot
[crate-version-badge]: https://img.shields.io/crates/v/janitor-bot.svg

This is a bot that removes users trying to join to a chat that is designed for comments. 

## Run 
To run you should export environment variable `TELOXIDE_TOKEN`.  
Example:
```shell script
$ TELOXIDE_TOKEN=<your token> cargo run
```
  
To run using Docker execute:
```shell script
$ docker run -d \
 --env TELOXIDE_TOKEN=<your token> \
 --name janitor-bot sschiz/janitor-bot:latest
```
