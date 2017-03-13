
Read events from the event store:

```rust
event_store.stream();
event_store.id("123").stream();
event_store.topic("task").stream();
event_store.topic("task").id("123").stream();

event_store.stream().until_empty();
event_store.stream().from_offset(20);
```

Persist events to the event store:

```rust
event_store.append(events);
event_store.id("123").append(events);
event_store.topic("task").append(events);
event_store.topic("task").id("123").append(events);
```
