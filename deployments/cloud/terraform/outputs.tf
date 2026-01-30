output "deployment_instructions" {
  description = "Instructions for deploying Marwan Hub"
  value = <<-EOT
    Marwan Hub Infrastructure deployed successfully!
    
    Next Steps:
    1. Configure kubectl:
       aws eks update-kubeconfig --name ${module.eks.cluster_id} --region ${var.aws_region}
    
    2. Deploy Marwan Hub:
       kubectl apply -f ../kubernetes/marwan-hub.yaml -n marwan-hub
     
    3. Get LoadBalancer URL:
       kubectl get svc -n marwan-hub
     
    4. Access the application:
       http://$(kubectl get svc marwan-hub-service -n marwan-hub -o jsonpath='{.status.loadBalancer.ingress[0].hostname}')
    
    5. Database Connection:
       Host: ${aws_db_instance.marwan_hub_db.endpoint}
       Database: marwan_hub
       Username: marwan
       Password: ${random_password.db_password.result}
    
    6. Redis Connection:
       Host: ${aws_elasticache_cluster.marwan_hub_redis.cache_nodes.0.address}
       Port: 6379
    
    7. S3 Bucket: ${aws_s3_bucket.marwan_hub_storage.bucket}
    
    8. Monitor Cluster:
       kubectl get pods -n marwan-hub
       kubectl get svc -n marwan-hub
       kubectl get ingress -n marwan-hub
    
    Environment: ${var.environment}
    Region: ${var.aws_region}
    EOT
}

output "security_notes" {
  description = "Important security notes"
  value = <<-EOT
    IMPORTANT SECURITY NOTES:
    
    1. Change all default passwords immediately
    2. Enable CloudTrail for auditing
    3. Configure WAF for the ALB
    4. Enable GuardDuty for threat detection
    5. Rotate database passwords regularly
    6. Enable SSL/TLS for all connections
    7. Configure backup and disaster recovery
    8. Monitor CloudWatch logs and metrics
    
    For production use:
    - Use AWS Secrets Manager for credentials
    - Implement Network Policies
    - Enable EKS control plane logging
    - Use private endpoints where possible
    EOT
  sensitive = true
}

output "cost_estimation" {
  description = "Monthly cost estimation"
  value = <<-EOT
    Estimated Monthly Costs:
    
    EKS Cluster (3 x t3.medium): ~$150
    RDS PostgreSQL (db.t3.small): ~$50
    ElastiCache Redis (cache.t3.small): ~$30
    S3 Storage (10GB): ~$3
    Data Transfer: ~$10
    ALB: ~$20
    Total Estimated: ~$263/month
    
    Note: Actual costs may vary based on usage.
    EOT
}

output "monitoring_endpoints" {
  description = "Monitoring endpoints"
  value = <<-EOT
    Monitoring Endpoints:
    
    1. EKS Dashboard:
       kubectl port-forward -n kube-system svc/kubernetes-dashboard 8080:80
       http://localhost:8080
    
    2. Prometheus Stack (if installed):
       kubectl port-forward -n monitoring svc/prometheus-operated 9090:9090
       http://localhost:9090
    
    3. Grafana (if installed):
       kubectl port-forward -n monitoring svc/grafana 3000:3000
       http://localhost:3000 (admin/admin)
    
    4. CloudWatch Logs:
       https://console.aws.amazon.com/cloudwatch/home?region=${var.aws_region}#logsV2:log-groups
    
    5. RDS Monitoring:
       https://console.aws.amazon.com/rds/home?region=${var.aws_region}#dbinstance:id=${aws_db_instance.marwan_hub_db.id}
    EOT
}
