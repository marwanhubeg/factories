#!/bin/bash
# سكريبت حذف الموارد
echo "⚠️  تحذير: هذا سيحذف جميع موارد Marwan Hub!"

read -p "هل أنت متأكد؟ اكتب 'نعم' للمتابعة: " CONFIRM

if [ "$CONFIRM" = "نعم" ]; then
    # 1. حذف Kubernetes resources
    kubectl delete -f ../kubernetes/marwan-hub.yaml --ignore-not-found
    
    # 2. حذف Terraform resources
    cd deployments/cloud/terraform
    terraform destroy -auto-approve
    
    echo "🗑️  تم حذف جميع الموارد"
else
    echo "❌ تم الإلغاء"
fi
