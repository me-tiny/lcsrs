<p align="center">
    <h1 align="center">lcsrs</h1>
    <p align="center">spaced repetition for leetcode</p>
</p>

## purpose

specifically for my workflow of [this leetcode script](http://github.com/me-tiny/leetcode), although modification to allow use on many different workflows is definitely on the cards if there's interest

may also implement the whole script into lcsrs for ease of use

## usage

### cli commands

```bash
# TODO: haven't set in stone the exact commands, have a baseline but may change
```

## deck

deck is imported from the solved problems in the `$LEETCODE_DIR/problems/`
directory and is stored at `$LEETCODE_DIR/.lcsrs.json`

when reviewing a problem, lcsrs will backup your current `sol.cpp` file at
`$LEETCODE_DIR/.lcsrs/backups`, and the currently worked on `sol.cpp` file will
be generated in `$LEETCODE_DIR/.lcsrs/active` with a clean template

## algorithm

using an algorithm inspired by [SuperMemo 2](https://supermemo.com/english/ol/sm2.htm), which was used by Anki before the
switch to [FSRS](https://github.com/open-spaced-repetition)

i went this direction, as i only use the `Good` and `Again` buttons in Anki, and
using the full implementation of SM2 or FSRS seemed a bit overkill for this
project, but i may switch to it after testing how this algorithm goes.

### breakdown

#### constants

> [!IMPORTANT]
> **subject to change with tuning, or implementation of a config file**

> _all of these live in `srs.rs`_

| name                    | value |
| ----------------------- | ----- |
| `INITIAL_INTERVAL_DAYS` | 1.0   |
| `INITIAL_EASE`          | 2.5   |
| `MIN_EASE`              | 1.5   |
| `EASE_BONUS_GOOD`       | 0.1   |
| `EASE_PENALTY_AGAIN`    | 0.3   |

##### tuning

_too frequent?_

raise `INITIAL_EASE` or lower `EASE_PENALTY_AGAIN`

_not frequent enough?_

lower `INITIAL_EASE` or raise `EASE_PENALTY_AGAIN`

#### card state

| field    | desc                                        | intial value |
| -------- | ------------------------------------------- | ------------ |
| interval | days until next review                      | 1.0          |
| ease     | multi applied on `Good`                     | 2.5          |
| streak   | streak of `Good` ratings, resets on `Again` | 0            |
| due      | next review date                            | today        |

#### on review

##### `Good`

> [!NOTE]
> solved without having to look anything up

```bash
interval = interval * ease
ease = min(ease + 0.1, 3.5)
streak += 1
```

##### `Again`

> [!NOTE]
> got stuck or had to look at backup or look it up

```bash
interval = 1
ease = max(ease - 0.3, 1.5)
streak = 0
```
