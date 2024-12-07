# Blitzer Notifier

This repository contains code for the blitzer notifier service. 

## Service
It contains the following components
- Blitzer.de API Client
- Mysql Database
- Telegram Bot API

### Blitzer.de API Client
A request is emitted against this endpoint:  
`https://cdn2.atudo.net/api/4.0/pois.php`

Also, this request params are passed:
- z (zoom, set to 5)
- type (csv of types (e.g. 0,1,2,103,ts), see location type enum)
- box (csv of 4 floats, lat_min,lng_min,lat_max,lng_max)

Example: 
```bash
curl https://cdn2.atudo.net/api/4.0/pois.php?z=5&type=0,1,2,3,4,5,6,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,117,ts,vwd&box=xx.x,xx.x,xx.x,xx.x
```

### Mysql Database
A Mysql database is needed to ensure that only new points of interest are send.

The services creates a table called `known_blitzer`. It contains all necessary data of a poi (point of interest)

### Telegram Bot API
A Telegram bot is used to send a message to a chat. A message is sent if a new poi is found. 

#### Create a bot
Send a message to @BotFather on telegram. He will send you your own token. 

Also create a new group. You find the chat id in the url. (It could start with a dash)

## Configuration
Create a file with the name Settings.toml and place it in the same directory as the executable.

```toml
[locations]
[locations.first]
lat= 0.0
lng= 0.0

[locations.second]
lat= 0.0
lng= 0.0

[database]
host="localhost"
port=3306
database="blitzer"
username="username"
password="password"

[telegram]
token=""
chat_id=""
```

## Deployment
Build the service with
```bash
cargo build --release
```

Start with
```bash
./target/release/blitzer
```

Create a cron (e.g. every hour) to execute this service.
