# Simple demo web app in Rust, compatible with Knative Serving

The app exposes the an API endpoint under `/` which accepts a `content` string as query parameter. It returns an image (by default a PNG) in the body with a QRCode that encodes the string.

Optionally one can also pass the following query parameters:

* `render`: how to render the image, either `png` (defaut) or `svg`.
* `size`: the size of the raster image in pixels (ignored when `render=svg`)
* `shape`: the shape of the QRCode modules: `square` (default), `roundedsquare`, `circle`, `diamond`, `vertical` or `horizontal` 
  (beware that not all client applications will be able to correctly decode a QRCode with an exotic module shape).
* `embed`: whether to embed an image in the QRcode (boolean, default is `false`). 
   The [default image](./assets/thehat.svg) can be replaced with a custom svg image 
   by means of the env variable `EMBEDDED_IMG_PATH`.

Example requests with curl:

```none
curl localhost:8080/?content=the-1st-string-to-be-encoded&size=1000&embed=true

curl localhost:8080/?content=the-2nd-string-to-be-encoded&render=svg&shape=roundedsquare
```

## Knative Serving

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

## Pipelines as Code

A file `.tekton/pipelinerun.yaml` is provided for the [Pipelines as Code feature of Tekton](https://pipelinesascode.com/). 
To enable it, follow the instructions provided by the previous link. It provides three tasks, executed sequentially:

1. fetch-repository (with [git-clone](https://hub.tekton.dev/tekton/task/git-clone))
2. build-image (with [buildah](https://hub.tekton.dev/tekton/task/buildah))
3. kn-service-apply (with [kn](https://hub.tekton.dev/tekton/task/kn))

This fetches the code, builds the container image and finally deploys the Knative service 
to the Kubernetes or OpenShift cluster.

> _Tested on OCP 4.14 with Pipelines 1.15_

NB1: An easier alternative would have been to use Knative Functions with Rust and remote deploy, but with the approach exemplified in this repo, we have full control over the image build, which is actually faster than with [buildpack](https://github.com/paketo-community/rust-dist) and produces a much leaner image!

NB2: On OpenShift, the pod that runs the `kn` Tekton Task might not have the necessary permissions to execute `kn service apply`. You may have to create a ClusterRole and bind it with a `ClusterRoleBinding` to the `pipeline` service account. 
See [crb.yaml](crb.yaml) for an example of how to achieve this (change the namespace according to your environment).