# Pipelines as Code

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