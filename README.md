# rent-scrapper
Scrap fresh rent ads from https://www.otodom.pl/ and https://www.olx.pl/ and send updates into Telegram private chat.

## Pre-requirement:
Installed Docker

## RunBook:
1. Register new bot with BotFather: https://core.telegram.org/bots#3-how-do-i-create-a-bot
2. Write msg to bot to identify chat_id
3. Setup OTODOM_SEARCH_URL https://github.com/nolik/rent-scrapper/blob/c2ed4178922845e56544b98e781a3bd6383bfc95/src/main.rs#L8 and OLX_SEARCH_URL https://github.com/nolik/rent-scrapper/blob/c2ed4178922845e56544b98e781a3bd6383bfc95/src/main.rs#L9
4. Setup TELEGRAM_BOT_ID and TELEGRAM_BOT_TOKEN for TELEGRAM_SEND_MSG_URL https://github.com/nolik/rent-scrapper/blob/ca56ff6d10c57c1f0651111fa9d39588bd940f78/src/main.rs#L10
5. Setup TELEGRAM_CHAT_ID https://github.com/nolik/rent-scrapper/blob/ca56ff6d10c57c1f0651111fa9d39588bd940f78/src/main.rs#L11
6. Build and run container with scrapper: `docker run --rm -it $(docker build -q .)`
