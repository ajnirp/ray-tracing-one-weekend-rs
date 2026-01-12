In PowerShell:

```shell
cargo build --release

# .\target\release\ray-tracing.exe <-- BAD, don't do this. It sets the encoding to UTF-16

# See https://github.com/RayTracing/raytracing.github.io/discussions/1114#discussioncomment-8314508 for details

# Instead, do this:

.\target\release\ray-tracing.exe | Set-Content img\a.ppm -encoding String
```

To compare two PPM files in PowerShell:

```shell
fc.exe img\a.ppm img\b.ppm
```

The output will be something like:

```
Comparing files .\IMG\a.ppm and .\IMG\b.ppm
FC: no differences encountered
```