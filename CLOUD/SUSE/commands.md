Exercise: Kubernetes Resources
Now you have learned many Kubernetes recourses, in this exercise, you will deploy the following resources using the kubectl command.

a namespace
name: demo
label: tier: test

SOLUTION:

Create namespace
`kubectl create ns demo`

# Label namespace
kubectl label ns demo tier=test

# List namespaces
`kubectl get ns`
or
`kubectl create namespaces`

 kubectl get ns --show-labels



Create deployment

a deployment:
image: nginx:alpine
name:nginx-apline
namespace: demo
replicas: 3
labels: app: nginx, tag: alpine

kubectl create deploy nginx-alpine --image=nginx:alpine -n demo -- r 3

kubectl create deploy nginx-alpine --image=nginx:alpine --replicas=3 --namespace demo


`kubectl create deploy go-helloworld --image=oyelowo/go-helloworld:v1.1.0 -n test-oyelowo`

# Add label
`kubectl label deploy nginx-alpine app=nginx tag=alpine --namespace demo`

# Show labels for deployment
´kubectl get deploy --show-labels `



a service:
expose the above deployment on port 8111
namespace: demo

`kubectl create service clusterip my-serv --tcp=8111:8111 -n demo`
`kubectl create service clusterip my-cs --tcp=8111:8111`


# Get pods in namespace -demo
`kubectl get po -n demo`




4
a configmap:
name: nginx-version
containing key-value pair: version=alpine
namespace: demo

`kubectl create configmap nginx-version --from-literal=version=alpine -n demo`

`kubectl get configmap -n demo -o yaml`





Soultion from exercise
Solution: Kubernetes Resources
Below is a snippet creating a namespace and labeling it, a deployment, a service, and a configmap using the kubectl operations.

# create the namespace 
# note: label option is not available with `kubectl create`
kubectl create ns demo

# label the namespace
kubectl label ns demo tier=test

# create the nginx-alpine deployment 
kubectl create deploy nginx-alpine --image=nginx:alpine  --replicas=3 --namespace demo

# label the deployment
kubectl label deploy nginx-alpine app=nginx tag=alpine --namespace demo

# expose the nginx-alpine deployment, which will create a service
kubectl expose deployment nginx-alpine --port=8111 --namespace demo

# create a config map
kubectl create configmap nginx-version --from-literal=version=alpine --namespace demo
Spoiler alert: in the next section, you will learn and practice how to deploy Kubernetes resources using a different approach.
