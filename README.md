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

To convert a PPM to a PNG:

```shell
magick. img\a.ppm a.png
```

For some reason, using a path like `img\a.png` makes the command fail with an error about "improper image headers".

To diff two PPM files:

```shell
fc.exe img\a.ppm img\b.ppm
```

The output will be something like:

```
Comparing files .\IMG\a.ppm and .\IMG\b.ppm
FC: no differences encountered
```

TODO:

* Profile code w/ flame graph and determine bottlenecks. The flamegraph SVG has text that is cut off. Try the text output instead. Maybe the random number generation is the bottleneck after adding buffered writes?
* Add a CameraOptions struct to simplify the process of initializing a camera. Right now there's a fairly long list of params to `Camera::new()`.
* Pass a mutable reference to the RNG through the functions instead of initializing it inside the utility helper.
* Make `image_width` an arg as well. And `aspect_ratio` can be a string that the program parses into a `<width, height>` pair.
* Figure out the improper image header issue with ImageMagick.
* GPU rendering!