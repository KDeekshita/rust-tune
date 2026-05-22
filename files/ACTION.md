# ACTION.md — Fix Plan for Issue #64

Resolves issue #64: three verified backend gaps found during codebase review.

---

## What was changed

### 1. `Cargo.toml` — Added missing dependencies

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dotenvy = "0.15"
```

- `serde` + `serde_json` — typed JSON serialization (Fix 1 + Fix 2)
- `dotenvy` — `.env` file loading at runtime (Fix 3)

---

### 2. `src/main.rs` — Full refactor across 3 issues

#### Fix 1 · Typed JSON responses via `serde`

**Before:**
```rust
.body(r#"{"message":"Hello from RustTune!","status":"ok"}"#)
```

**After:**
```rust
#[derive(Serialize)]
struct HelloResponse {
    message: String,
    status: String,
}

// handler returns:
web::Json(HelloResponse {
    message: "Hello from RustTune!".into(),
    status: "ok".into(),
})
```

Compile-time schema enforcement. No silent JSON typos possible.

---

#### Fix 2 · `GET /api/songs` endpoint with shared app state

**Before:** No `/api/songs` route. Song titles hardcoded in HTML.

**After:**
```rust
#[derive(Serialize, Clone)]
struct Song { id: u32, title: String, artist: String, duration: String }

// registered as:
web::Data::new(song_data)

// route:
#[get("/api/songs")]
async fn songs(data: web::Data<Vec<Song>>) -> impl Responder {
    web::Json(data.get_ref().clone())
}
```

Frontend can now call `GET /api/songs` and render dynamically.
Song data lives in the backend — ready for DB integration later.

---

#### Fix 3 · Environment variable config (HOST / PORT)

**Before:**
```rust
println!("Server running at http://127.0.0.1:8000");
.bind(("127.0.0.1", 8000))?
```

**After:**
```rust
dotenvy::dotenv().ok();
let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
let port: u16 = env::var("PORT")
    .unwrap_or_else(|_| "8000".to_string())
    .parse()
    .expect("PORT must be a valid u16");
```

Defaults preserved — no breaking change for local dev.
Set `PORT=3000` or any value at runtime without recompiling.

---

### 3. `.env.example` — New file

```env
HOST=127.0.0.1
PORT=8000
```

Copy to `.env` for local overrides. `.env` stays gitignored.

---

## How to test locally

```bash
# Default — same as before
cargo run

# Custom port
PORT=3000 cargo run

# Via .env
cp .env.example .env
# edit .env as needed
cargo run

# Verify songs API
curl http://127.0.0.1:8000/api/songs

# Verify hello API (now typed JSON)
curl http://127.0.0.1:8000/api/hello
```

---

## Expected responses

**`GET /api/hello`**
```json
{ "message": "Hello from RustTune!", "status": "ok" }
```

**`GET /api/songs`**
```json
[
  { "id": 1, "title": "Midnight Echo", "artist": "Electronic Vibes", "duration": "3:24" },
  { "id": 2, "title": "Neon Waves",    "artist": "Synth Pop",        "duration": "4:05" },
  { "id": 3, "title": "Dream Runner",  "artist": "Lo-Fi Beats",      "duration": "2:58" }
]
```

---

## Files changed

| File | Change |
|------|--------|
| `Cargo.toml` | Added `serde`, `serde_json`, `dotenvy` |
| `src/main.rs` | Full refactor — typed responses, songs endpoint, env config |
| `.env.example` | New — runtime config template |

---

## Next steps (out of scope for this PR)

- Update `script.js` to fetch `/api/songs` and render dynamically (replaces hardcoded HTML cards)
- Add DB persistence layer when playlist/favorites features land
