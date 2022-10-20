# Tashikani

![Python version](https://img.shields.io/badge/python-3.10-blue)
![GitHub](https://img.shields.io/github/license/Ukiyo-server/tashikani)
![Discord](https://img.shields.io/discord/937243464079400990)
![GitHub issues](https://img.shields.io/github/issues/ukiyo-server/Tashikani)
![GitHub closed issues](https://img.shields.io/github/issues-closed-raw/ukiyo-server/Tashikani)
![GitHub pull requests](https://img.shields.io/github/issues-pr/ukiyo-server/Tashikani)
![GitHub closed pull requests](https://img.shields.io/github/issues-pr-closed/ukiyo-server/Tashikani)

Tashikani is a Discord bot that helps you learn Japanese. learn new words, level your vocabulary, compete with your friends, and more.

The source code here is developing and educational purposes. I would recommend against running your own instance of this bot. Instead, use t!invite and click the [link](https://discord.gg/gFqhKcs2p8) to add the bot to your server.

## Contributing

We open to contributions. Please, however, make an issue first if you want to contribute to the bot.

Right now, the code's still quite convoluted. We working on refactoring and reorganizing the project.

## Roadmap

We looking to add the following major features to the bot, in addition to fixes and small updates:

Coming soon.

## Development Notes

### Editor Setup

This project has been configured to work on these editors listed below out-of-the-box. To use other code editor and/or tooling, please respect the code formatting and style, feel free to add your editor config in this repo without changing the existing code formats. You will still need to use these following plugins on your editor to help you developing:

#### Neovim

Use the following plugins:

-   **Conquer of Completion**: Language server support for Neovim using Nodejs extensions\
    ([Installation guide](https://github.com/neoclide/coc.nvim))
    -   **coc-pyright**: Pyright for coc.nvim\
        ([Installation guide](https://github.com/fannheyward/coc-pyright))
-   **vim-toml**: TOML support\
     ([Installation guide](https://github.com/cespare/vim-toml))

> VS Code support is still in-progress at the moment as the Pyright plugin doesn't work

#### Visual Studio Code / Code OSS

Use the following plugins

-   **Pyright**: Static type checking for Python\
    ([OpenVSX](https://open-vsx.org/extension/ms-pyright/pyright) / [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=ms-pyright.pyright))
-   **Even Better TOML**: TOML support\
    ([Open VSX](https://open-vsx.org/extension/tamasfe/even-better-toml) / [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml))

### Required programs

#### Poetry

Poetry is a packaging and dependency manager for Python. It is used for

**Installation**:

Linux, macOS, Windows (WSL)

```sh
curl -sSL https://install.python-poetry.org | python3 -
```

Windows (Powershell)

```
(Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicParsing).Content | py -
```

Further installation guide: https://python-poetry.org/docs/

#### Just

> This hasn't been tested to work in Windows

A command runner

**Installation**:

via Cargo

```sh
cargo install just
```

via Homebrew

```sh
brew install just
```

via pacman (Arch Linux)

```sh
pacman -S just
```

Other system/package manager: https://github.com/casey/just

### Running the bot

You may need to setup a bot yourself if you haven't done it yet. Running bot requires you to input the bot API token. Refer to [this guide](https://discordpy.readthedocs.io/en/stable/discord.html) if you need a help in setting up your own bot.

Initialize project:

```sh
just init
```

Set the Discord bot token:

-   Create a `.env` file
-   `.env` file should be formatted like this:

```
DISCORD_TOKEN=<your-bot-token-here>
```

Run the bot:

```sh
just run
```

