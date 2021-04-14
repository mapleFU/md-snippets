# MD-Snippets

MD-Snippets 是 mwish 的代码碎片，专门放一些垃圾

目前有：

* [vector_clock](./src/vector_clock.rs)
* [leaky bucket](./components/ratelimiter): 用 mutex 实现的 Leaky Bucket 限流算法，因为 AtomicPtr 使用有点费劲，就算了
* [consistent hashing](./components/consistent_hash): port groupcache 的 consistent ring hash
* [single flight](./components/single_flight): port groupcache 的 single flight, 因为 Rust 的 WG 的 wait 和 Go 语义不太一样，所以写了个 spin (实际上可以用 cv)
* [bloom filter](./components/bloom_filter): 实现的朴素的 Bloom Filter.
