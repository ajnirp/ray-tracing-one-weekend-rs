# Build

In PowerShell, to build:

```shell
cargo build --release
```

# Run

To run, don't run this

```shell
# .\target\release\ray-tracing.exe <-- BAD, don't do this. It sets the encoding to UTF-16
```

See [here](https://github.com/RayTracing/raytracing.github.io/discussions/1114#discussioncomment-8314508) for details. Instead, do this:

```shell
.\target\release\ray-tracing.exe | Set-Content img\a.ppm -encoding String
```

To profile the runtime:

```shell
Measure-Command { .\target\release\ray-tracing.exe | Set-Content img\a.ppm -encoding String }
```

# Example run

On my machine, to generate the image at the end of the book,

```
Days              : 0
Hours             : 0
Minutes           : 33
Seconds           : 14
Milliseconds      : 344
Ticks             : 19943446029
TotalDays         : 0.0230826921631944
TotalHours        : 0.553984611916667
TotalMinutes      : 33.239076715
TotalSeconds      : 1994.3446029
TotalMilliseconds : 1994344.6029
```

# Misc

To compare two PPM files:

```shell
fc.exe img\a.ppm img\b.ppm
```

The output will be something like:

```
Comparing files .\IMG\a.ppm and .\IMG\b.ppm
FC: no differences encountered
```

TODO:

* pass a mutable reference to the RNG through the functions instead of initializing it inside the utility helper