# RustTune Performance Benchmark Report

This document outlines the performance optimizations applied to RustTune's music search, playlist loading, metadata fetching, and audio streaming endpoints. 

The test was conducted using **k6** to measure system behavior before ("Naive") and after ("Optimized") applying the improvements.

## 🛠 Load Test Methodology
- **Tool**: k6
- **Test Duration**: 25 seconds (Ramp up, steady load, ramp down)
- **Virtual Users (VUs)**: Up to 10 concurrent users
- **Dataset Size**: 100,000 mock songs
- **Audio File Size**: ~5MB dummy MP3 file

---

## 📊 Results Summary

| Endpoint | Naive Avg Time | Optimized Avg Time | Improvement Factor |
|----------|---------------:|-------------------:|-------------------:|
| Search (`?q=Rock`) | **40.71 ms** | **1.06 ms** | **~38x faster** |
| Playlist Loading | **36.01 ms** | **0.74 ms** | **~48x faster** |
| Metadata Fetching | **1.63 ms** | **0.67 ms** | **~2.4x faster** |
| Audio Streaming | **2.83 ms** | **1.86 ms** | **~1.5x faster** |

*Note: In the naive streaming version, the entire 5MB file was buffered into memory and sent at once, which can lead to Out-Of-Memory (OOM) errors at scale. The optimized version streams bytes on-demand using HTTP Range requests.*

---

## 🚀 Key Optimizations Applied

### 1. Music Search (`/search`)
- **Before**: Iterated over all 100,000 songs, comparing strings sequentially (`O(N)`).
- **After**: Implemented an in-memory **Inverted Index** (`HashMap<String, HashSet<usize>>`). Lookups are now `O(1)` per token, with sub-millisecond response times.

### 2. Playlist Loading (`/playlist`)
- **Before**: Returned the entire playlist containing 100,000 tracks. This resulted in huge memory usage and massive JSON payloads over the network (1.7 GB total data received during the test).
- **After**: Introduced **Pagination** (`?page=1&limit=50`). The response size is strictly bounded, drastically reducing both serialization overhead and network transfer times.

### 3. Metadata Fetching (`/metadata/{id}`)
- **Before**: Performed an `O(N)` array scan to find a song by its ID.
- **After**: Introduced an `O(1)` `HashMap` lookup and a **Moka Cache** (`moka::sync::Cache`) layer to cache frequently accessed metadata, guaranteeing fast lookups regardless of library size.

### 4. Audio Streaming (`/stream`)
- **Before**: The server read the entire audio file into memory and returned a massive blob, lacking support for partial downloads.
- **After**: Utilized `actix_files::NamedFile` to support **HTTP Range Requests**. Audio files are now streamed in manageable chunks (e.g., 1MB at a time) and offloaded efficiently, enabling the client to buffer or seek seamlessly.

---

## 📈 K6 Output Snapshot (Optimized Endpoints)

```text
metadata_time_optimized..........: avg=0.671308  min=0           med=0.4851  max=2.0628  p(90)=0.672   p(95)=0.8879
playlist_time_optimized..........: avg=0.743781  min=0           med=0.5188  max=2.2335  p(90)=1.0116  p(95)=1.4398
search_time_optimized............: avg=1.06014   min=0.3344      med=0.8887  max=4.7578  p(90)=1.4586  p(95)=1.4883
stream_time_optimized............: avg=1.862606  min=1.0324      med=1.6249  max=3.2081  p(90)=2.3618  p(95)=2.6833
```
