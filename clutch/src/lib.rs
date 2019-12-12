//! # Clutch
//! 
//! Clutch is a MongoDB ODM inspired by MongooseJS and Rust's very own 
//! Diesel ORM. Its goal is to provide a means of interfacing with MongoDB 
//! in an easy to use, intuitive way, without sacrificing the ability to 
//! perform complex queries.
//! 
//! This project is ostensibly a Work In Progress, and large portions of the 
//! API are subject to change.

/* MongoDB */
extern crate mongodb;
#[macro_use] extern crate bson;

/* Serde */
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

/* Sys */
extern crate ron;
#[macro_use] extern crate bitflags;
