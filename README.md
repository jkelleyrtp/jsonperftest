# Json Perf test

This little bit of code does a numerical simulation with two different approaches:
    - One where the state is encoded in json throughout
    - One where the state is encoded as a rust struct throughout

I was curious what the performance hit of using json as the interchange format was in comparison with just sticking with regular rust structs. The code performs 1 million steps of an electron in a magnetic field as it moves along its gyroradius and I use the timeit module to time the entire runtime of the program including instantiation of the electron and the simulation.

I tried my best to keep the code similar to what's already in the codebase, including reindexing the json object every timestep. 

I was surprised of actually how performant the json approach was - I was personally expecting even more of a performance hit by encoding data as an enum in a hashmap. Still though, the rust version of the code is a 38x speedup over the json version. Even if time-to-simulate is not a concern, I think the neighbor calculation is.


```
Compiling jsonperftest v0.1.0 (/Users/jonkelley/Development/HashAI/Tinkering/jsonperftest)
Finished release [optimized + debuginfo] target(s) in 0.89s
Running `target/release/examples/serdey`
    1 loops: 617.494 ms
   
Compiling jsonperftest v0.1.0 (/Users/jonkelley/Development/HashAI/Tinkering/jsonperftest)
Finished release [optimized + debuginfo] target(s) in 0.56s
Running `target/release/examples/rusty`
    10 loops: 16.388100000000005 ms
```