# Pods
A Pod can contain one or more containers that are tightly coupled and depend on one another. I.O.W, runs one or more closely related containers E.g a database, it's logger and back-up manager.
Without the db, the logger and back-up manager would be useless. So, these
containers with these apps can be good candidate for a pod.

# Services
Sets up networking in a kubernetes Cluster. You need to set up service if you want to access the pods
They include:
- ClusterIP
  
- NodePort
  Exposes a container to the outside world(only good for dev purposes. Not used for production) and allows us to access the container from inside the browser

- LoadBalancer
  
- Ingress



# Linking different components
You use selector in one to refer to the label in another. The selector does not have to be called component.
i.e
  selector:
    component: web

for 
  labels:
    component: web
Can as well be
  selector:
    tier: frontend

for 
  labels:
    tier: frontend
```
apiVersion: v1
kind: Service
metadata:
  name:  client-node-port
spec:
  type: NodePort
  ports:
    - ports: 3050
      targetPort:  8080
      nodePort:  31515
  selector:
    component: web
```

```
apiVersion: v1
kind: Pod #
metadata:
  name: client-pod
  labels:
    component: web
spec:
  containers:
    - name: client
      image: stephengrider/multi-client
      ports:
        - containerPort: 3000

```

Ports that should be mapped
    - port: 3050
      targetPort:  3000
      nodePort:  31515

- targetPort is the containerPort in the Pod. Used to access a container within a pod.

- nodePort: used from the browser to access a Pod

# COMMANDS
- Feed a config file to kubectl
kubectl apply -f <filename>

- Get the status of all running pods
  `kubectl get pods`
- Get the status of all running pods
  `kubectl get pods`

- Get the status of all running services
  `kubectl get services`

- Check local ip address
  `minikube ip`



## Kube-api server in Master
It is 100% responsible for monitoring the current status of all the nodes in a cluster and making sure that
they are doing the correct thing.
E.g could check a multi-worker node to check the number of copies it's running, if it is less than what it needs to be running. This does this by polling all the nodes to make sure they're running the right amount of container working. If something fails, it can go to docker-hub to pull the image and run the container.

You don't work directly with node, rather you work with the master.
