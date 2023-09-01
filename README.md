# Simple WASM demo web app in Rust, compatible with Knative Serving

_**NB**: This branch provides the same app as in branch `warp`, but adapted for compilation to [WebAssembly](https://webassembly.org/) + [WASI](https://wasi.dev/). Besides replacing the warp and tokio dependencies in `Cargo.toml` to [warp_wasi](https://crates.io/crates/warp_wasi) and [tokio_wasi](https://crates.io/crates/tokio_wasi), respectively, we only had to change change the `#[tokio::main]` annotation of the main function to `#[tokio::main(flavor = "current_thread")]`._

The app exposes the an API endpoint under `/` which accepts a `content` string as query parameter. It returns an image (by default a PNG) in the body with a QRCode that encodes the string.

Optionally one can also pass the following query parameters:

* `render`: how to render the image, either `png` (defaut) or `svg`.
* `size`: the size of the raster image in pixels (ignored when `render=svg`)
* `shape`: the shape of the QRCode modules: `square` (default), `roundedsquare`, `circle`, `diamond`, `vertical` or `horizontal` 
  (beware that not all client applications will be able to correctly decode a QRCode with an exotic module shape).

Example requests with curl:

```none
curl localhost:8080/?content=the-1st-string-to-be-encoded&size=1000

curl localhost:8080/?content=the-2nd-string-to-be-encoded&render=svg&shape=roundedsquare
```

## Compiling to WASM

First, we need make sure to add the appropriate compilation target to Rust:

```
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release
```

The binary can be executed with a WebAssembly runtime like WasmTime or WasmEdge:

```
wasmedge target/wasm32-wasi/release/qrcode-warp.wasm
```

You can even build a lightweight OCI container and run it with podman! For that, your container runtime engine needs to use crun instead of runc, and you need to install the packages wasmedge and crun-wasm on your host environment (I've tested it with Fedora CoreOS).

The container image can be generated by running:

```
$ cat > Containerfile <<EOF
FROM scratch
COPY ./target/wasm32-wasi/release/qrcode-warp.wasm /
CMD [ "/qrcode-warp.wasm" ]
EOF
$ podman build --platform wasi/wasm -f Containerfile -t qrcode-warp-wasm .
```

The container can be run with 

```
podman run --platform wasi/wasm -p 8080:8080 qrcode-warp-wasm
```

Test that it works as expected:

```
curl "http://localhost:8080/?content=rust-and-wasmedge-rock-1907348731469128738172638713&render=svg" -o test.svg
```


