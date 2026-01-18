In PowerShell, to build:

```shell
cargo build --release
```

To run, don't run this

```shell
# .\target\release\ray-tracing.exe <-- BAD, don't do this. It sets the encoding to UTF-16
```

See [here](https://github.com/RayTracing/raytracing.github.io/discussions/1114#discussioncomment-8314508) for details. Instead, do this:

```shell
.\target\release\ray-tracing.exe | Set-Content img\a.ppm -encoding String
```

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