//! [![license badge][]][license link] [![rust badge]][rust link]
//!
//! # dawn
//!
//! `dawn` is an asynchronous, simple, and extensible set of libraries which can
//! be used separately or in combination for the Discord API.
//!
//! This crate is a "skeleton crate": it includes all of the non-vendor-specific
//! crates in the `dawn` ecosystem. These include crates like `dawn-cache`,
//! `dawn-command-parser`, `dawn-gateway`, `dawn-http`, `dawn-model`,
//! `dawn-voice`, and more. These are explained in detail.
//!
//! Not included by default are crates like `dawn-cache-redis` for a
//! redis-backed cache implementation, `dawn-lavalink` for lavalink voice
//! support, and more. Read further down for a list of known first-party and
//! third-party integration crates.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`'s `[dependencies]` section:
//!
//! ```toml
//! dawn = "0.1"
//! ```
//!
//! ## Crates
//!
//! These are crates that can work together for a full application experience.
//! You may not need all of these - such as `dawn-cache` - but they can be
//! mixed together to accomplish just what you need.
//!
//! ### `dawn-model`
//!
//! `dawn-model` is a set of models defining structures, enums, and bitflags
//! for the entirety of the Discord API. It is split into a number of
//! sub-modules, such as `gateway` for containing the WebSocket gateway types,
//! `guild` for containing types owned by guilds (servers), `voice` containing
//! the types used by the Voice WebSocket API, and more.
//!
//! These are all in a single crate so that you can use `gateway` models without
//! depending on `dawn-gateway`. One use case is if you write your own WebSocket
//! gateway implementation.
//!
//! ### `dawn-cache`
//!
//! `dawn-cache` is based on a single trait which can be implemented to use
//! custom third-party backends with a single ubiquitous interface. The Cache is
//! responsible for holding information about things like guilds, channels, role
//! information, voice states, and any other data that comes from Discord.
//!
//! Included by default is an `InMemoryCache` backend, which caches within the
//! process's memory. Also available as a first-class library is
//! `dawn-cache-redis` which supports caching via Redis.
//!
//! ### `dawn-gateway`
//!
//! `dawn-gateway` is an implementation of Discord's sharding gateway sessions.
//! This is responsible for receiving stateful events in real-time from Discord
//! and sending *some* stateful information.
//!
//! It includes two primary types: the Shard and Cluster.
//!
//! The Shard handles a single WebSocket connection and can manage up to 2500
//! guilds. If you manage a small bot in under about 2000 guilds, then this is
//! what you use. See the [Discord docs][docs:discord:sharding] for more
//! information on sharding.
//!
//! The Cluster is an interface which manages the health of the shards it
//! manages and proxies all of their events under one unified stream. This is
//! useful to use if you have a large bot in over 1000 or 2000 guilds.
//!
//! ### `dawn-command-parser`
//!
//! `dawn-command-parser` is a crate for parsing commands out of messages
//! received over the gateway. It finds messages commanding your bot and parses
//! the arguments out.
//!
//! ### `dawn-http`
//!
//! `dawn-http` is an HTTP client supporting all of the Discord REST API. It is
//! based on `hyper`. It meets Discord's ratelimiting requirements and supports
//! proxying.
//!
//! ### `dawn-voice`
//!
//! `dawn-voice` is a crate supporting Discord's voice API. It exposes a
//! powerful API supporting efficient managed voice connections, queueing,
//! playback mutation, streaming, and audio controls.
//!
//! ## Examples
//!
//! ```rust,ignore
//! use futures::StreamExt;
//! use dawn::{
//!     gateway::{Config, Event, Shard},
//!     http::Client as HttpClient,
//! };
//! use std::{
//!     env,
//!     error::Error,
//! };
//!
//! let token = env::var("DISCORD_TOKEN")?;
//!
//! let http = HttpClient::new(&token);
//!
//! let config = Config::builder(&token).build();
//! let mut shard = Shard::new(config);
//! shard.connect().await?;
//! let mut events = shard.events();
//!
//! while let Some(event) = events.next().await {
//!     runtime::spawn(handle_event(event));
//! }
//!
//! async fn handle_event(event: Event) -> Result<(), Box<dyn Error>> {
//!     match event {
//!         Event::Connected(connected) => {
//!             println!("Connected on shard {}", connected.shard_id);
//!         },
//!         Event::Message(msg) => {
//!             if msg.content == "!ping" {
//!                 http.send_message(msg.channel_id).content("Pong!").await?;
//!             }
//!         },
//!         _ => {},
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! Maintaining a cache of guilds, users, channels, and more sent by the
//! gateway:
//!
//! ```rust,ignore
//! use futures::StreamExt;
//! use dawn::{
//!     cache::InMemoryCache,
//!     gateway::{Config, Event, Shard},
//! };
//! use std::{
//!     env,
//!     error::Error,
//! };
//!
//! let token = env::var("DISCORD_TOKEN")?;
//!
//! let config = Config::builder(&token).build();
//! let mut shard = Shard::new(config);
//! shard.connect().await?;
//! let mut events = shard.events();
//!
//! let cache = InMemoryCache::new();
//!
//! while let Some(event) = events.next().await {
//!     runtime::spawn(cache.update(&event));
//! }
//! ```
//!
//! ## Provided Crates
//!
//! Below is a list of crates which are either first-party or known unofficial
//! third-party crates. These are not included by default.
//!
//! ### First-party
//!
//! #### dawn-cache-redis
//!
//! `dawn-cache-redis` is an asynchronous caching implementation backed by
//! Redis. It uses `redis-async-rs`.
//!
//! #### dawn-lavalink
//!
//! `dawn-lavalink` is an implementation bridging `dawn-gateway` and [Lavalink],
//! offering a powerful interface for audio control.
//!
//! ### Third-party
//!
//! N/A
//!
//! ## License
//!
//! All first-party crates are licensed under [ISC][LICENSE.md]
//!
//! [LICENSE.md]: https://github.com/dawn-rs/dawn/blob/master/LICENSE.md
//! [Lavalink]: https://github.com/Frederikam/Lavalink
//! [docs:discord:sharding]: https://discordapp.com/developers/docs/topics/gateway#sharding
//! [license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
//! [license link]: https://opensource.org/licenses/ISC
//! [rust badge]: https://img.shields.io/badge/rust-nightly-93450a.svg?style=flat-square
//! [rust link]: https://github.com/rust-lang/rust/milestone/66

#[cfg(feature = "command-parser")]
pub extern crate dawn_command_parser as command_parser;

#[cfg(feature = "http")]
pub extern crate dawn_http as http;

#[cfg(feature = "model")]
pub extern crate dawn_model as model;
