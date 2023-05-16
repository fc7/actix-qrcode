# Simple demo app with Actix Web microframework

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

The endpoints `/health/readiness` and `/health/liveness` are also provided for compatibility with [Knative Serving](https://knative.dev/docs/serving/).

By default the service listens to port 8080. It can be modified via the environment variable `PORT`.
The IP address to which the service binds is by default `0.0.0.0`, but it can be likewise modified via `BIND_ADDRESS`.

The provided `Dockerfile` can be used to build a container image based on Fedora 38. 
Alternatively, using the file `Dockerfile.ubi9` will base the image on UBI 9, 
which is a better choice for a production deployment, e.g. on OpenShift.

To deploy your image as a Knative service, e.g. on OpenShift Serverless, use e.g.

```none
kn service create qrcode \
    --image your-repo/qrcode \
    --port 8080
```
