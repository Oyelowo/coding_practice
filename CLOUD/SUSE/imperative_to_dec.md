# Create namespace and extract to a yaml file.
`kubectl create ns demo2 --dry-run=client -o yaml > namespace.yaml`


# Create deployment and extract to a yaml file.
`kubectl create deploy busybox --image=busybox -r=5 -n demo2 --dry-run=client -o yaml `
