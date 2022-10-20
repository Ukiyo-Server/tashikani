#!/usr/bin/env python3

import os
import discord
from discord.message import Message

intents = discord.Intents.default()
intents.message_content = True

client = discord.Client(intents=intents)


@client.event
async def on_ready():
    print(f'Logged in as {client.user}')


@client.event
async def on_message(message: Message):
    if message.author == client.user:
        return

    if message.content.startswith('!help'):
        await message.channel.send("Help")


def main() -> None:
    client.run(os.environ['DISCORD_TOKEN'])


if __name__ == "__main__":
    main()
