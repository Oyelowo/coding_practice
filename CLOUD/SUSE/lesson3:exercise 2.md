Exercise: Declarative Kubernetes Manifests
Kubernetes is widely known for its imperative and declarative management techniques. In the previous exercise, you have deployed the following resources using the imperative approach. Now deploy them using the declarative approach.

a namespace

name: demo
label: tier: test
a deployment:

# Solution
`kubectl create ns demoo --dry-run=client -o yaml > demoo-namespace.yaml`


image: nginx:alpine
name:nginx-apline
namespace: demo
replicas: 3
labels: app: nginx, tag: alpine

# Solution:
`kubectl create deploy nginx-alpine --image=nginx:alpine  -n demoo  --dry-run=client -o yaml > demoo-deployment.yaml`

a service:
expose the above deployment on port 8111
namespace: demo

# Solution
kubectl expose deploy nginx-alpine --port=8111 --target-port=8111 -n demoo --dry-run=client -o yaml > demoo-service.yaml

a configmap:

name: nginx-version
containing key-value pair: version=alpine
namespace: demo

# Solution
 kubectl create configmap nginx-version --from-literal=version=alpine -n demoo --dry-run=client -o yaml > demoo-configmap.yaml 

Note: Nginx is one of the public Docker images, that you can access and use for your exercises or testing purposes.



## Exercise solutions

Solution: Declarative Kubernetes Manifests
Declarative Approach
The declarative approach consists of using a full YAML definition of resources. As well, with this approach, you can perform directory level operations.

Examine the manifests for all of the resources in the exercises/manifests.

nd064_course_1/exercises/manifests

To create the resources, use the following command:

kubectl apply -f exercises/manifests/
To inspect all the resources within the namespace, use the following command:

kubectl get all -n demo

NAME                                READY   STATUS    RESTARTS   AGE
pod/nginx-alpine-798fb5b8bb-8rzq9   1/1     Running   0          12s
pod/nginx-alpine-798fb5b8bb-ms28l   1/1     Running   0          12s
pod/nginx-alpine-798fb5b8bb-qgqb2   1/1     Running   0          12s

NAME                   TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)    AGE
service/nginx-alpine   ClusterIP   10.109.197.180   <none>        8111/TCP   18s

NAME                           READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/nginx-alpine   3/3     3            3           12s

NAME                                      DESIRED   CURRENT   READY   AGE
replicaset.apps/nginx-alpine-798fb5b8bb   3  
