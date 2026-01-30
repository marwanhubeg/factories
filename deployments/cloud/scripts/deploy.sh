#!/bin/bash
# سكريبت النشر على السحابة
echo "🚀 نشر Marwan Hub على السحابة..."

# متغيرات
REGION="us-east-1"
CLUSTER_NAME="marwan-hub-cluster"
NAMESPACE="marwan-hub"

# 1. تهيئة Terraform
cd deployments/cloud/terraform
terraform init

# 2. التحقق من الخطة
terraform plan

# 3. التطبيق
read -p "هل تريد المتابعة؟ (نعم/لا): " CONFIRM
if [ "$CONFIRM" = "نعم" ]; then
    terraform apply -auto-approve
fi

# 4. تكوين kubectl
aws eks update-kubeconfig --region $REGION --name $CLUSTER_NAME

# 5. نشر Kubernetes
kubectl create namespace $NAMESPACE
kubectl apply -f ../../kubernetes/marwan-hub.yaml

echo "✅ تم النشر بنجاح!"
