# AutoReplaceBroken
AutoReplaceBroken is proof-of-concept spigot plugin written in Rust.

The plugin replaces any broken block with bedrock.

## Building
### Prerequisites
To build the project you need:
- Java 21 or higher
- Rust with Cargo

### Actually building
Just run `./gradlew build` like you would on any project, and it will automatically build the natives for the current architecture if it is supported.
If your architecture is not supported, gradle will throw an error.

Supported architectures:

| Architecture | Windows | MacOS | Linux |
|--------------|---------|-------|-------|
| x86 (i686)   | ✅       | ❌     | ✅     |
| x86_64       | ✅       | ✅     | ✅     |
| aarch64      | ❓       | ✅     | ✅     |
✅ = Fully supported<br>
❌ = Not supported<br>
❓ = May be possible, but not officially supported<br>
Any architecture or OS not listed here is not supported

For building a production-ready jar, it is recommended to use the GitHub workflow, as it is capable of building for all supported architectures.

## Working with the code
In the root directory, you will find the java project.

It consists of only 4 classes, most notably the `ARBRustGlue` and `NativeLibraryLoader` classes.<br>
`NativeLibraryLoader` contains methods and enums that are used for OS and architecture recognition, so the correct native library can be easily extracted and loaded.<br>
`ARBRustGlue` contains all native methods, that work as entrypoints into the Rust world.<br>
The leftover two classes are standard Bukkit classes (the main class and a Listener), you should intuitively be able to understand what they do.
If you do not, you most likely do not have a sufficient level to work with this project anyway.

In the `rust/` directory, you will find the Rust project. 

In `rust/src/` you will find the actual sources of the native library.<br>
`lib.rs` contains all native methods also defined in `me.andreasmelone.autoreplacebroken.jni.ARBRustGlue`. 
If you wish to add more events the code may respond to, you most likely want to do it there. Take the other functions as examples-<br>
`material.rs` is the Bukkit Material enum, converted to rust using `j2r_enum_gen.rs` which can be found in `rust/`<br>
`wrappers.rs` contains all wrapper structs, which are prefixed by W. All these structs do is wrap the equivalent classes/interfaces from Bukkit API.
If you wish to use more functionality from the Bukkit API, you most likely want to add wrappers here.

## License
As this is a Proof-of-Concept project, I have decided to license this project under CC0, meaning it is published to public domain and may be used by anybody without providing credits to the original project.