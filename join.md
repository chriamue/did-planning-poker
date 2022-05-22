# Join Protocol

## roles

This protocol has two roles: host, client

Clients join a session via invitation information.

Hosts accept clients join message.

## join

```json
{
  "id": "1234567890",
  "type": "https://github.com/chriamue/did-planning-poker/blob/main/join.md#join",
  "from": "did:example:client",
  "to": ["did:example:host"],
  "body": {
    "id": "<SessionId>",
    "alias": "<ClientName>",
  }
}
```

### body

id: Session Id from an invitation.

alias: Name, the Client wants visible for others

## accept

```json
{
  "id": "2345678901",
  "thid": "1234567890"
  "type": "https://github.com/chriamue/did-planning-poker/blob/main/join.md#accept",
  "from": "did:example:host",
  "to": ["did:example:client"],
}
```

Answer with join request id as thid.

Join request was accepted and host will inform client about updates.

## reject

```json
{
  "id": "2345678901",
  "thid": "1234567890"
  "type": "https://github.com/chriamue/did-planning-poker/blob/main/join.md#reject",
  "from": "did:example:host",
  "to": ["did:example:client"],
}
```

Join request was rejected. This is the case if collision of alias happened.
