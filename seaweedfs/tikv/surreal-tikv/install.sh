kubectl create -f https://raw.githubusercontent.com/pingcap/tidb-operator/v1.3.8/manifests/crd.yaml

kubectl create namespace tidb-admin

helm install --namespace tidb-admin tidb-operator pingcap/tidb-operator --version v1.3.8

kubectl get pods --namespace tidb-admin -l app.kubernetes.io/instance=tidb-operator

kubectl create namespace tikv-cluster
kubectl apply -f tidb-operator/tikv-cluster.yaml -n tikv-cluster   

kubectl -n tikv-cluster apply -f https://raw.githubusercontent.com/pingcap/tidb-operator/master/examples/basic/tidb-monitor.yaml