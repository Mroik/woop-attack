TO DECIDE
=========

Rules
======

Basic rules:
- A game lasts 4 weeks
- A player can control up to 3 zords
- A zord has 3 lives
- Lives of each zord are restored to 3 at the beginning of the week
- Each player each day gets 5 actions per day
- Unused actions at the end of the day are lost
- Each player starts out with 0 points
- A kill is rewarded with 3 points
- Dying halves your points (`floor(points / 2)`)
- Players out of zords respawn after 2 days
- At the end of the game the player with the most points wins

Basic actions:
- Combat:
    - A player can use an action point to shoot another player
    - A player can shoot only after 3 hours from the beginning of the day
    - A player can only shoot players within their range
    - The base range is 5 squares
    - The distance between 2 squares is calculated as follows:
        `max(abs(x_f - x_t), abs(y_f - y_t))`
    - If hit directly a zord loses a life
    - A zord dies if they lose all lives
    - A player can use an action to build a shield
    - A player can build multiple shields
    - A shield is consumed when shot by another player
    - A shield lasts until the end of the day
- Movement:
    - A player can use an action to move a zord one square
    - A zord can only move in one of the eight adjacent square
- Special actions:
    - A player can use an action to donate points
    - A player can donate any amount of points they have
    - A player can build a zord by spending 10 points
    - A player can use an action to increase their range by one
    - At the end of the day the range returns to its base value
    - A player can spend 5 points to gift an action to another player

Tecnicalities
============

- Every player can see the amount of active shields everyone has
- Every player can see the amount of points everyone else
- Every player can see the range of everyone else
- Every player can see amount of lives of every zord
