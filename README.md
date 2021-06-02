# Foxie Bot
This is just a simple bot for the server I am on with my friends

## Usage
### Hosted by me
If you want to use the version I host, on your server, use this [link](https://discord.com/api/oauth2/authorize?client_id=810753013219065917&permissions=240704&scope=bot)
### Hosted by you
If you want to host Foxie yourself, just run `cargo build --release`. You have to have a functional rust sdk installed to do this. \
To supply the bot token to the program you can do one of two things:
- Set the environment variable FOXIE_TOKEN to your bot token
- Create a file with the name `.env`. In it, put `FOXIE_TOKEN={your bot token}`

