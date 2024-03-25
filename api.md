Authentication
==============

Endpoints that require authentication require the following headers:
- username
- token

Where the username is simply the email you provided to the gamemaster trucated
at the `@` character.

Requests
========

The only request method allowed is `POST` regardless if it is semantically
correct. All of the request bodies are expected to be `JSON` the same way the
replies will only be in `JSON`. Whenever an error is encountered the API replies
with
```json
{"error": string}
```

For a request to an endpoint to succeed all preconditions for that action must
be met.

Endpoints
=========

`/shoot`
--------

Request data:
```json
{"from": [int, int], "to": [int, int]}
```

Given the coordinates of a zord that you own and another player's zord, your
zord shoots at the targeted zord.

`/move`
-------

Request data:
```json
{"from": [int, int], "to": [int, int]}
```

Given the coordinates of the zord to move and the cell to end up in move the
zord to said cell.

`/shield`
---------

```json
{"coord": [int, int]}
```

Given the coordinate of a zord generate a shield on said zord.

`/increase`
-----------

Request data:
```json
{"coord": [int, int]}
```

Given the coordinate of a zord, increase its range.

`/donate`
---------

Request data:
```json
{"receiver": string, "amount": int}
```

Given the username of another player donate the specified amount of points.

`/build`
--------

Request data:
```json
{"coord": [int, int]}
```

Given a coordinate withing distance 1 of a zord you own, build a new zord at the
specified coordinate.

`/map`
------

Response data:
```json
{"map": [
    {
        "Zord": {
            "x": int,
            "y": int,
            "hp": int,
            "shields": int,
            "range": int,
            "owner": string,
        }
    },
    {
        "Totem": {
            "x": int,
            "y": int,
        }
    },
    ...
]}
```

Returns the entities present on the map. An entity can either be a `zord` or a
`totem`.

`/leaderboard`
--------------

Response data:
```json
{"leaderboard": [
    {
        "Player": {
            "name": string,
            "actions": int,
            "points": int,
        }
    },
    ...
]}
```

Returns all of the player's info.

`/day`
------

Response data:
```json
{"game-info": {
    "day": int,
    "start_of_day": int,
}}
```

Returns the current day and the unix timestamp of the start of the current day.
Subtract 10800 from the current timestamp and check if it is less than the start
of the day. If it is you're within the 3 hour window of truce.

All timestamps are calculated using the `Europe/Rome` timezone.

`/activity`
-----------

Query parameters:
- `chunk`

Response data:
```json
[
    {
        "shoot": {
            shooter: string,
            from: [int, int],
            to: [int, int],
            target: string,
            timestamp: int,
        }
    },
    {
        "move": {
            player: string,
            from: [int, int],
            to: [int, int],
            timestamp: int,
        }
    },
    {
        "generate-shield": {
            player: string,
            zord_coord: [int, int],
            timestamp: int,
        }
    },
    {
        "increase-range": {
            player: string,
            zord_coord: [int, int],
            timestamp: int,
        }
    },
    {
        "donate-points": {
            from: string,
            to: string,
            timestamp: int,
        }
    },
    {
        "build-zord": {
            player: string,
            zord_coord: [int, int],
            timestamp: int,
        }
    },
    {
        "totem-points": {
            player: string,
            coord: [int, int],
            points: int,
            timestamp: int,
        }
    },
    {
        "respawn": {
            player: string,
            coord: [int, int],
            timestamp: int,
        }
    },
    {
        "totem-spawned": {
            coord: [int, int],
            timestamp: int,
        }
    },
    ...
]
```

Returns the last 100 events from the activity log. To get the other events
provide the `chunk` parameter to get the other chunks of events.
