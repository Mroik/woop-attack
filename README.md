Basic actions:
- Combat:
    - [x] A player can use an action point to shoot another player
    - [x] A player can shoot only after 3 hours from the beginning of the day
    - [x] A player can only shoot players within their range
    - [x] The base range is 5 squares
    - [x] The distance between 2 squares is calculated as follows:
        `max(abs(x_f - x_t), abs(y_f - y_t))`
    - [x] If hit directly a zord loses a life
    - [x] A zord dies if they lose all lives
    - [x] A player can use an action to build a shield
    - [x] A player can build multiple shields
    - [x] A shield is consumed when shot by another player
    - [x] A shield lasts until the end of the day
- Movement:
    - [x] A player can use an action to move a zord one square
    - [x] A zord can only move in one of the eight adjacent square
- Special actions:
    - [x] A player can use an action to donate points
    - [x] A player can donate any amount of points they have
    - [x] A player can build a zord by spending 10 points and using an action
    - [x] A player can use an action to increase their range by one
    - [x] At the end of the day the range returns to its base value
    - [ ] At the end of the day players with zords on the totem areas get points
    - [ ] A player can spend 5 points to gift an action to another player

API: TODO
