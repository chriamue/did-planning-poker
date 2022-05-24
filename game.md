# Game Protocol

## roles

This protocol has two roles: host, client

Clients can send their vote.

Hosts can update games players.

## players

```json
{
  "id": "1234567890",
  "type": "https://github.com/chriamue/did-planning-poker/blob/main/game.md#players",
  "from": "did:example:host",
  "to": ["did:example:client"],
  "body": {
    "id": "<SessionId>",
    "players" [
      {
        "did": "<did>",
        "alias": "<alias>",
        "ping": 42,
        "voted": "<vote>"
      }
    ]
  }
}
```

### players body

id: Session Id from an invitation.

did: did of the player

alias: alias of the player

ping: ping of the player the host measured

voted: last vote

## cards

```json
{
  "id": "2345678901",
  "type": "https://github.com/chriamue/did-planning-poker/blob/main/game.md#cards",
  "from": "did:example:host",
  "to": ["did:example:client"],
  "body": {
    "id": "<SessionId>",
    "cards": ["<card>"]
  }
}
```

### cards body

id: Session Id from an invitation.

cards: array of available cards for voting.

## vote

```json
{
  "id": "2345678901",
  "thid": "1234567890",
  "type": "https://github.com/chriamue/did-planning-poker/blob/main/game.md#vote",
  "from": "did:example:host",
  "to": ["did:example:client"],
  "body": {
    "id": "<SessionId>",
    "vote": "<vote>"
  }
}
```

### vote body

id: Session Id from an invitation.
