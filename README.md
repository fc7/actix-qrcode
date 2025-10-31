# Simple QR code service in Rust

The app exposes the an API endpoint under `/` which accepts a `content` string as query parameter. It returns an image (by default a PNG) in the body with a QR code that encodes the string.

Optionally one can also pass the following query parameters:

* `render`: how to render the image, either `png` (defaut) or `svg`.
* `size`: the size of the raster image in pixels (ignored when `render=svg`)
* `shape`: the shape of the QR code modules: `square` (default), `roundedsquare`, `circle`, `diamond`, `vertical` or `horizontal`
  (beware that not all client applications will be able to correctly decode a QR code with an exotic module shape).
* `embed`: whether to embed an image in the QR code (boolean, default is `false`).
   The [default image](./assets/thehat.svg) can be replaced with a custom svg image
   by means of the env variable `EMBEDDED_IMG_PATH`.

Example requests with curl:

```none
curl localhost:8080/?content=the-1st-string-to-be-encoded&size=1000&embed=true

curl localhost:8080/?content=the-2nd-string-to-be-encoded&render=svg&shape=roundedsquare
```

## Health probes

The endpoints `/health/readiness` and `/health/liveness` are also provided.

By default the service listens to port 8080. It can be modified via the environment variable `PORT`.
The IP address to which the service binds is by default `0.0.0.0`, but it can be likewise modified via `BIND_ADDRESS`.

CORS (Cross-Origin Resource Sharing) is enabled by default and allows requests from any origin. To restrict CORS to a specific origin, set the `CORS_ORIGIN` environment variable (e.g., `CORS_ORIGIN=http://localhost:5173`).

The provided `Containerfile` can be used to build a container image based on 
[UBI 9](https://catalog.redhat.com/en/software/containers/ubi9/ubi/615bcf606feffc5384e8452e) ([Docker Hub](https://hub.docker.com/r/redhat/ubi9)).

## Deploying on OpenShift

To deploy on OpenShift, use the provided [deployment.yaml](./openshift/deployment.yaml) and adapt it as needed.

The ClusterRoleBinding [crb.yaml](./openshift/crb.yaml) is useful when deploying with Knative Serving.
See the [Pipelines as Code README](./openshift/README.md) for more details.
