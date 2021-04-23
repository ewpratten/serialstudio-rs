# Serial Studio for Rust

[![Crates.io](https://img.shields.io/crates/v/serialstudio)](https://crates.io/crates/serialstudio) 
[![Documentation](https://docs.rs/serialstudio/badge.svg)](https://docs.rs/serialstudio) 
![Build](https://github.com/Ewpratten/serialstudio-rs/workflows/Build/badge.svg)

This is a very simple Rust library to act as a data source for [Serial Studio](https://github.com/Serial-Studio/Serial-Studio).

## What is provided?

This crate provides a simple threaded TCP server for use with Serial Studio. As of now, only one connection can be handled at a time, per port.
