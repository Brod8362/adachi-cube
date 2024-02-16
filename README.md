# Adachi Cube Discord Bot

[Add to your server](https://discord.com/api/oauth2/authorize?client_id=1207911707104509972&permissions=2048&scope=bot)

Need to fact-check a source?
Need to make an important life decision?
Feeling overwhelmed with the burden of choice?

Let the **Adachi Cube** be your single source of truth!

You're only one `/ask` away from any answer you seek.

The Adachi Cube is not responsible for any consequences of your actions. It should be noted, however, that it is always correct.

# The Bot

The Adachi Cube is a very-high-effort shitpost discord bot, with custom-made blender animations.

When asked a yes or no question with `/ask`, it will respond with an answer (true, false, maybe)

# Running

First, configure `config.toml`. See `bot/config.toml.template` for example configuration options. 
Additionally see `bot/src/config.rs` for more detailed technical info.

Run with `cargo run`.

The default config path is `./config.toml`, but can be overridden with the `ADACHI_CONFIG_PATH` environment variable.

# Docker

Dockerfile TBD.
