# minikube start --disk-size=40g --extra-disks=1 --driver hyperkit


# sudo -S qemu-img create -f raw /var/lib/libvirt/images/minikube-box-vm-disk-50G 50G
# virsh -c qemu:///system attach-disk minikube --source /var/lib/libvirt/images/minikube-box-vm-disk-50G --target vdb --cache none
# virsh -c qemu:///system reboot --domain minikube


minikube start -n 5


sudo qemu-img create -f qcow2 /var/lib/libvirt/images/minikube-m02.qcow2 25000M -o preallocation=full
sudo qemu-img create -f qcow2 /var/lib/libvirt/images/minikube-m03.qcow2 25000M -o preallocation=full
sudo qemu-img create -f qcow2 /var/lib/libvirt/images/minikube-m04.qcow2 25000M -o preallocation=full
sudo qemu-img create -f qcow2 /var/lib/libvirt/images/minikube-m05.qcow2 25000M -o preallocation=full


virsh attach-disk --domain minikube-m02 /var/lib/libvirt/images/minikube-m02.qcow2 --target vdb --persistent --config --live
virsh attach-disk --domain minikube-m03 /var/lib/libvirt/images/minikube-m03.qcow2 --target vdb --persistent --config --live
virsh attach-disk --domain minikube-m04 /var/lib/libvirt/images/minikube-m04.qcow2 --target vdb --persistent --config --live
virsh attach-disk --domain minikube-m05 /var/lib/libvirt/images/minikube-m05.qcow2 --target vdb --persistent --config --live