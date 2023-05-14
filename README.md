# Simple demo app with Actix Web microframework

The app exposes the an API endpoint under `/` which accepts a `content` string as query parameter. It returns a PNG in the body with a QRCode that encodes the string.

By default it listens to port 8080, which can be modified through the env variable `PORT`.

The provided `Dockerfile` can be used to build a container image for the microservice.

The endpoints `/health/readiness` and `/health/liveness` are also provided for compatibility with [Knative Serving](https://knative.dev/docs/serving/).
