# winipt-rs
## high level [winipt](https://github.com/ionescu007/winipt) rust bindings.

### Building
To build, youll only need a working c compiler discoverable by rust.
On windows the easiest way is to install `visual studio` and then
get the `Desktop Development with c++` Componement with the installer.

### Usage
Usage examples can be found in in the tests, which are located in the [lib.rs](https://github.com/sum-catnip/winipt-rs/blob/master/src/lib.rs) file.
The [original repo](https://github.com/ionescu007/winipt) has an example tool which can also be used as usage examples.

### TODO
- IPT_OPTIONS struct should probably implement copy trait since its only an int in size.
- when thats done, pass options by value instead of reference
- there should be a function which gets the trace and gets the trace size itself (maybe even put that in the main function)
- better ipt_options constructor
- use bitflags instead of bitmasks
