## Object Types
- __Pods__
A Pod can contain one or more containers that are tightly coupled and depend on one another. I.O.W, runs one or more closely related containers E.g a database, it's logger and back-up manager.
Without the db, the logger and back-up manager would be useless. So, these
containers with these apps can be good candidate for a pod.

- __Services__
Sets up networking in a kubernetes Cluster. You need to set up service if you want to access the pods. One reason is, everytime we spin up a pod, new ip address is assigned and it's cumbersome to have to manually get the new ip address to access the pod, so, rather than doing that directly, services is responsible for this.
They include:
- _ClusterIP_
  Exposes a set of pods to other objects in the cluster. It does not allow traffic from outside world. Just objects within a cluster
  
- _NodePort_
  Exposes a container/ set of pods to the outside world(only good for dev purposes. Not used for production) and allows us to access the container from inside the browser

- _LoadBalancer_
  
- _Ingress_

- __Deployment__
  It's similar to a pod. Maintains a set of identical pods, ensuring that they have the correct config and that the right number exists


## Pods Vs Deployment
- Pods a basic way of running kubernetes to run a single set of containers
   Deployment runs a set of identical ods (one or more)
- Pods is good for one-off dev purposes
   Deployment monitors the state of each pod, updating as necessary
- Pods rarely used directly in production
   Deployment is good for dev and for production

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
- Get the status of all running pods with more info
  `kubectl get pods -o wide`

- Get the status of all running services
  `kubectl get services`

- Check local ip address
  `minikube ip`



## Kube-api server in Master
It is 100% responsible for monitoring the current status of all the nodes in a cluster and making sure that
they are doing the correct thing.
E.g could check a multi-worker node to check the number of copies it's running, if it is less than what it needs to be running. This does this by polling all the nodes to make sure they're running the right amount of container working. If something fails, it can go to docker-hub to pull the image and run the container.

You don't work directly with node, rather you work with the master.


## IMPERATIVE VS DECLARATIVE APPROACH

In declarative approach, rather than writing commands to check the pods and whatnot and tell them how to update based on status, we can just update the config file and pass to `kubectl` which passes that to master and determine what to do.
E.g
if we change the image in an object, the master can check the name and the kind and based on that determine if it needs to update a pod on a node(virtual machine) or spin up a new one. `name` and `kind` are the identifying fields and changing this will spin up a new po.
So, if you want to update an object, you need to leave the name and kind as the same

## GETTING DETAILED INFO ABOUT AN OBJECT
`kubectl describe <object type> <object name(optional)>`
e.g `kubectl describe pod client-pod`

# REMOVING AND OBJECT (Imperative update)
`kubectl delete -f <config file>`

# GET STATUS OF DEPLOYMENT OBJECT
`kubectl get deployments`

## RESTART POD WHEN IMAGE UPDATED
`kubectl rollout restart -f client-deployment.yaml`

## RECONFIGURE DOCKER FOR DESKTOP IN THE CURRENT TERMINAL TEMPORARILY TO ENABLE DOCKER CONNECT TO DOCKER IN KUBERNETES VM(NODE)
`eval $(minikube docker-env)`
after this, you can do `docker ps` in the same terminal.

With this, you can inspect containers in Kubernetes VM(node) using existing knowledge of docker commands - docker cli.  With this, you can also `docker system prune` if there seems to be a caching issue in the containers in kubernetes
NB: A lot of these commands are also available to kubectl
e.g
`kubectl logs <pod tag>`
`kubectl exec -it <pod tag>` sh


## PVC (Persistent Volume Claim.)
`kubectl get pvc`

_More notes_
Same as in docker-compose/docker's volume to share the host machine's operating system with the filesystem in the container. 
e.g to make changes in local development reflect in our container.

We use this for postgres in case where a pod crashes and recreates a new pod, no data will be carried over from the deleted pod with postgres, which means we will be losing our data.

With PVC, we can have a persistent filesystem that can be accessed by postgres by having the data in the "Volume" on host machine. So, anytime we write data to the postgres container(in the pod), it is written into the volume.

So, if a pod with postgres crashes, we still link the volume on our machine with the newly created container in a new pod.

## Persistent Volume
`kubectl get pv`

## Volume
Not exactly the same thing as a docker volume.
We don't want this for data that needs to last.
- __"Volume" in generic container terminology__
  some type of mechanism that allows a container to access a filesystm outside itself

- __"Volume" in kubernetes__
  An object that allows a container to store data at the pod level

We are not using this because the data is tied to a pod. So, even though e.g other postgres containers can access the data within the pod(which means another postgres container e.g can access the same data if one crashes), it is also possible for the entire pod to die/crash/recreated.

So, kubernetes volume is not good for databases.

- __PVC vs KC__
Kubernetes Volume is tied to the lifecycle of a pod. PVC is tied to a lasting persistent data source which can still be accessed even  if a pod crashes.

- __Persistent Volume Claim vs Persistent Volume__
PVC is the advertised possible storage option while PV is the actual provisioned storage which could be static or dynamic

_Statistically provisioned persistent volume_ is volume that is available for PVC while _Dynamically provisioned persistent volume_  is created on the fly if the requested PVC could not be satisfied by the Dynamically provisioned persistent volume.


## Access Modes
- ReadWriteOnce: Can be used by a single node.
  
- ReadOnlyMany: Multiple nodes can read from this
  
- ReadWriteMany: Can be read and written to by many nodes


## Storage Info
`kubectl get storageclass`
`kubectl describe storageclass`


## Secrets
Securely stores a piece of information in the cluster such as database password.
To create secret imperatively
`kubectl create secret generic <secret name> --from-literal key=value`
e.g
`kubectl create secret generic pgpassword --from-literal PGPASSWORD=pass12345`



`kubectl create secret docker-registry` - for docker stuff
`kubectl create secret tls`

_To check secrets_:
`kubectl get secrets`