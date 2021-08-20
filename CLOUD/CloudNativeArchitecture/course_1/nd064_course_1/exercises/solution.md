Location of kubeconfig file in k3s: 
` /etc/rancher/k3s/k3s.yaml`

To get info about the kubeconfig : ` /etc/rancher/k3s/k3s.yaml`

Normally, it is set in 

Answer:

endpoints of the control plane and add-ons:

`kubectl cluster-info` to get the control plane and add-ons endpoints
aPI Server - Kubernetes control plane is running at https://127.0.0.1:6443

CoreDNS is running at https://127.0.0.1:6443/api/v1/namespaces/kube-system/services/kube-dns:dns/proxy

Metrics-server is running at https://127.0.0.1:6443/api/v1/namespaces/kube-system/services/https:metrics-server:/proxy


- Authentication mechanism:
   username (admin) and password

- `kubectl get nodes` -  to get all the nodes in the cluster
amount of nodes: 1

- `kubectl get nodes -o wide` - to get extra details about the nodes, including internal IP
node internal IP: 10.0.2.15

- `kubectl describe node node-name`  - to get all the configuration details about the node, including the allocated pod CIDR

`kubectl describe node localhost | grep CIDR`
the pod CIDR allocate to the node:  10.42.0.0/24
