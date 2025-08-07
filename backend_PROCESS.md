### **Core Concept**

A stateless WebSocket relay that:

* Registers clients via a `connectionId`
* Forwards text/binary messages to `target_id`
* Uses `broadcast::Sender<Message>` per connection

---

### 📦 **Important Data Structures**

```rust
struct AppState {
    connections: Arc<Mutex<HashMap<String, broadcast::Sender<Message>>>>,
}
```

---

### 🧷 **Connection Registration**

```rust
if data["type"] == "register" {
    let id = data["connectionId"].as_str();
    state.connections.lock().await.insert(id, tx.clone());
}
```

Each client identifies itself by a custom ID.

---

### 📤 **Message Forwarding**

```rust
if let Some(target_id) = data["target_id"].as_str() {
    if let Some(target_tx) = state.connections.lock().await.get(target_id) {
        let _ = target_tx.send(Message::Text(text));
    }
}
```

---

### 📦 **Binary Message Relay**

```rust
if let Some(target_id) = target_map.get(&conn_id) {
    if let Some(target_tx) = state.connections.lock().await.get(target_id) {
        let _ = target_tx.send(Message::Binary(bin_data));
    }
}
```

---

### 🏓 **Keepalive Ping**

```rust
let mut interval = interval(Duration::from_secs(30));
loop {
    interval.tick().await;
    ping_tx.send(Message::Ping(vec![])).await;
}
```

---

### 🧹 **Disconnect Cleanup**

```rust
state.connections.lock().await.remove(&conn_id);
```

---

### 🔁 **Tasks to Spawn**

* `sender_task`: forward internal messages to client
* `receive_task`: handle incoming messages from client
* `ping_task`: periodic ping
* `forward_task`: listen on broadcast and forward to WebSocket

---
