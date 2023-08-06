# Websocket API

## Types

### Bytes32HexString

"0x0000000000000000000000000000000000000000000000000000000000000000"

### Event

```json
{
  blockNumber: Number,
  EventIndex: Number
};
```

### EventMeta

```json
{
  index: Number,
  name: String
}
```

### Variant

```json
{
  index: Number,
  name: String,
  events: EventMeta
}
```
]

### Key

```json
{
  type: "AccountId",
  value: Bytes32HexString
};
```

```json
{
  type: "AccountIndex",
  value: Number
};
```

```json
{
  type: "AuctionIndex",
  value: Number
};
```

```json
{
  type: "BountyIndex",
  value: Number
};
```

```json
{
  type: "CandidateHash",
  value: Bytes32HexString
};
```

```json
{
  type: "EraIndex",
  value: Number
};
```

```json
{
  type: "MessageId",
  value: Bytes32HexString
};
```

```json
{
  type: "ParaId",
  value: Number
};
```

```json
{
  type: "PoolId",
  value: Number
};
```

```json
{
  type: "PreimageHash",
  value: Bytes32HexString
};
```

```json
{
  type: "ProposalHash",
  value: Bytes32HexString
};
```

```json
{
  type: "ProposalIndex",
  value: Number
};
```

```json
{
  type: "RefIndex",
  value: Number
};
```

```json
{
  type: "RegistrarIndex",
  value: Number
};
```

```json
{
  type: "SessionIndex",
  value: Number
};
```

```json
{
  type: "TipHash",
  value: Bytes32HexString
};
```

```json
{
  type: "Variant",
  value: [Number, Number]
};
```

## Request

### Status

```json
{
  type: "Status"
};
```

### Variants

```json
{
  type: "Variants"
};
```

### Get Events

```json
{
  type: "GetEvents",
  key: Key
};
```

### Subscribe Events

```json
{
  type: "SubscribeEvents",
  key: Key
};
```

## Response

### Status
    
```json
{
  type: "Status",
  lastHeadBlock: Number,
  lastBatchBlock: Number,
  batchIndexingComplete: Boolean
};
```

### Variants

```json
{
  type: "Variants",
  data: [Variant, ...]
}
````

### Events

```json
{
  type: "Events",
  key: Key,
  data: [Event, ...]
}
````

### Subscribed

```json
{
  type: "Subscribed"
}
````
