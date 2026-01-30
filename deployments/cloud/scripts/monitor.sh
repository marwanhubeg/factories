#!/bin/bash
# سكريبت المراقبة
echo "📊 مراقبة نظام Marwan Hub..."

# الحالة العامة
kubectl get pods -n marwan-hub
kubectl get svc -n marwan-hub
kubectl get ingress -n marwan-hub

# السجلات
echo "📝 سجلات التطبيق:"
kubectl logs -n marwan-hub deployment/marwan-hub --tail=10

# الموارد
echo "💻 استخدام الموارد:"
kubectl top pods -n marwan-hub
kubectl top nodes

# الصحة
echo "🏥 فحص الصحة:"
kubectl describe deployment marwan-hub -n marwan-hub
