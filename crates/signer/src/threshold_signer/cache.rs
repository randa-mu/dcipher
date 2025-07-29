// use std::num::NonZeroUsize;
// use std::sync::Arc;
// use lru::LruCache;
// use redis::aio::{ConnectionManager, ConnectionManagerConfig};
//
// struct RedisBackedCache<K, V> {
//     mem_cache: Arc<tokio::sync::Mutex<LruCache<K, V>>>,
//     redis: ConnectionManager,
//     // Arc<std::sync::Mutex<LruCache<Vec<u8>, HashMap<u16, PartialSignature<SignatureGroup<BLS>>>>>>;
// }
//
// impl<K, V> RedisBackedCache<K, V> {
//     pub fn new(mem_cache_size: NonZeroUsize, redis_connection_manager: ConnectionManager) -> Self {
//         let config = ConnectionManagerConfig::new()
//             .set_push_sender()
//         Self {
//             mem_cache: Arc::new(tokio::sync::Mutex::new(LruCache::new(mem_cache_size))),
//             redis
//         }
//     }
// }
