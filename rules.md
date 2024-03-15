woop-attack
=========

Rules
======

Basic rules:
- A game lasts 4 weeks
- A player can control up to 3 zords
- A zord has 2 lives
- Each player each day gets 5 actions per day
- Unused actions at the end of the day are lost
- Each player starts out with 0 points
- A kill is rewarded with 3 points
- Losing all zords makes the player lose a third of their points (`floor(points * 2 / 3)`)
- Players out of zords respawn at the beginning of the next day
- At the end of the game the player with the most points wins

Basic actions:
- Combat:
    - A player can use an action point to shoot another player
    - A player can shoot only after 3 hours from the beginning of the day
    - A player can only shoot players within their range
    - The base range is 5 squares
    - The distance between 2 squares is calculated as follows:
        `max(abs(x_f - x_t), abs(y_f - y_t))`
    - If hit directly a zord loses a health point
    - A zord dies if they lose all lives
    - A player can use an action to build a shield
    - A player can build multiple shields
    - A shield is consumed when shot by another player
    - A shield lasts until the end of the day
    - A player can use an action to increase their range by one
    - At the end of the day the range returns to its base value
- Movement:
    - A player can use an action to move a zord one square
- Special actions:
    - A player can use an action to donate points
    - A player can donate any amount of points they have
    - A player can build a zord by spending 10 points and using an action
    - A zord built using 10 points starts out with 1 health point
    - A player can spend 5 points to gift an action to another player (not sure if I want this rule)

Objectives:
- Across the map there are multiple totems (2 totems might be ideal)
- At the end of the day the totem awards 50 points distributed equally among the players based on
    the zords in the totem area

Tecnicalities
============

- Every player can see the amount of active shields everyone has
- Every player can see the amount of points everyone else has
- Every player can see the range of everyone else
- Every player can see amount of lives of every zord
