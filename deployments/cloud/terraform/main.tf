# Terraform Configuration for Marwan Hub Factories v3.0.0
terraform {
  required_version = ">= 1.0.0"
  
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
    
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.0"
    }
    
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.0"
    }
  }
  
  backend "s3" {
    bucket = "marwan-hub-tfstate"
    key    = "marwan-hub-factories/terraform.tfstate"
    region = "us-east-1"
  }
}

# Provider Configuration
provider "aws" {
  region = var.aws_region
  profile = var.aws_profile
}

provider "kubernetes" {
  host                   = data.aws_eks_cluster.cluster.endpoint
  cluster_ca_certificate = base64decode(data.aws_eks_cluster.cluster.certificate_authority.0.data)
  token                  = data.aws_eks_cluster_auth.cluster.token
}

provider "helm" {
  kubernetes {
    host                   = data.aws_eks_cluster.cluster.endpoint
    cluster_ca_certificate = base64decode(data.aws_eks_cluster.cluster.certificate_authority.0.data)
    token                  = data.aws_eks_cluster_auth.cluster.token
  }
}

# Variables
variable "aws_region" {
  description = "AWS Region"
  type        = string
  default     = "us-east-1"
}

variable "aws_profile" {
  description = "AWS Profile"
  type        = string
  default     = "default"
}

variable "cluster_name" {
  description = "EKS Cluster Name"
  type        = string
  default     = "marwan-hub-cluster"
}

variable "environment" {
  description = "Environment Name"
  type        = string
  default     = "production"
}

variable "vpc_cidr" {
  description = "VPC CIDR Block"
  type        = string
  default     = "10.0.0.0/16"
}

# Data Sources
data "aws_availability_zones" "available" {
  state = "available"
}

data "aws_caller_identity" "current" {}

data "aws_eks_cluster" "cluster" {
  name = module.eks.cluster_id
}

data "aws_eks_cluster_auth" "cluster" {
  name = module.eks.cluster_id
}

# VPC Module
module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "~> 3.0"

  name = "${var.cluster_name}-vpc"
  cidr = var.vpc_cidr

  azs             = slice(data.aws_availability_zones.available.names, 0, 3)
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]

  enable_nat_gateway   = true
  single_nat_gateway   = true
  enable_dns_hostnames = true

  tags = {
    "kubernetes.io/cluster/${var.cluster_name}" = "shared"
    Environment = var.environment
    Project     = "marwan-hub"
  }
}

# EKS Module
module "eks" {
  source  = "terraform-aws-modules/eks/aws"
  version = "~> 18.0"

  cluster_name    = var.cluster_name
  cluster_version = "1.24"

  vpc_id     = module.vpc.vpc_id
  subnet_ids = module.vpc.private_subnets

  eks_managed_node_groups = {
    main = {
      desired_size = 3
      max_size     = 5
      min_size     = 2

      instance_types = ["t3.medium"]
      capacity_type  = "ON_DEMAND"

      labels = {
        Environment = var.environment
        Project     = "marwan-hub"
      }
    }
  }

  node_security_group_additional_rules = {
    ingress_self_all = {
      description = "Node to node all ports/protocols"
      protocol    = "-1"
      from_port   = 0
      to_port     = 0
      type        = "ingress"
      self        = true
    }
    
    egress_all = {
      description      = "Node all egress"
      protocol         = "-1"
      from_port        = 0
      to_port          = 0
      type             = "egress"
      cidr_blocks      = ["0.0.0.0/0"]
      ipv6_cidr_blocks = ["::/0"]
    }
  }

  tags = {
    Environment = var.environment
    Project     = "marwan-hub"
  }
}

# RDS for PostgreSQL
resource "aws_db_instance" "marwan_hub_db" {
  identifier           = "marwan-hub-db"
  engine              = "postgres"
  engine_version      = "15"
  instance_class      = "db.t3.small"
  allocated_storage   = 20
  storage_encrypted   = true
  
  db_name             = "marwan_hub"
  username            = "marwan"
  password            = random_password.db_password.result
  
  vpc_security_group_ids = [aws_security_group.rds_sg.id]
  db_subnet_group_name   = aws_db_subnet_group.marwan_hub.name
  
  backup_retention_period = 7
  backup_window          = "03:00-04:00"
  maintenance_window     = "sun:04:00-sun:05:00"
  
  skip_final_snapshot   = false
  final_snapshot_identifier = "marwan-hub-db-final-snapshot"
  
  tags = {
    Name        = "marwan-hub-database"
    Environment = var.environment
    Project     = "marwan-hub"
  }
}

resource "random_password" "db_password" {
  length  = 16
  special = false
}

resource "aws_db_subnet_group" "marwan_hub" {
  name       = "marwan-hub-db-subnet-group"
  subnet_ids = module.vpc.private_subnets
  
  tags = {
    Name = "marwan-hub-db-subnet-group"
  }
}

resource "aws_security_group" "rds_sg" {
  name        = "marwan-hub-rds-sg"
  description = "Security group for Marwan Hub RDS"
  vpc_id      = module.vpc.vpc_id

  ingress {
    description     = "PostgreSQL from EKS"
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [module.eks.node_security_group_id]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "marwan-hub-rds-sg"
    Environment = var.environment
  }
}

# ElastiCache for Redis
resource "aws_elasticache_cluster" "marwan_hub_redis" {
  cluster_id           = "marwan-hub-redis"
  engine              = "redis"
  node_type           = "cache.t3.small"
  num_cache_nodes     = 1
  parameter_group_name = "default.redis7"
  port                = 6379
  subnet_group_name   = aws_elasticache_subnet_group.marwan_hub.name
  security_group_ids  = [aws_security_group.redis_sg.id]
  
  tags = {
    Name        = "marwan-hub-redis"
    Environment = var.environment
    Project     = "marwan-hub"
  }
}

resource "aws_elasticache_subnet_group" "marwan_hub" {
  name       = "marwan-hub-redis-subnet-group"
  subnet_ids = module.vpc.private_subnets
}

resource "aws_security_group" "redis_sg" {
  name        = "marwan-hub-redis-sg"
  description = "Security group for Marwan Hub Redis"
  vpc_id      = module.vpc.vpc_id

  ingress {
    description     = "Redis from EKS"
    from_port       = 6379
    to_port         = 6379
    protocol        = "tcp"
    security_groups = [module.eks.node_security_group_id]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name        = "marwan-hub-redis-sg"
    Environment = var.environment
  }
}

# S3 for Storage
resource "aws_s3_bucket" "marwan_hub_storage" {
  bucket = "marwan-hub-storage-${data.aws_caller_identity.current.account_id}"
  
  tags = {
    Name        = "marwan-hub-storage"
    Environment = var.environment
    Project     = "marwan-hub"
  }
}

resource "aws_s3_bucket_versioning" "marwan_hub_storage" {
  bucket = aws_s3_bucket.marwan_hub_storage.id
  
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "marwan_hub_storage" {
  bucket = aws_s3_bucket.marwan_hub_storage.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

# IAM Role for EKS
resource "aws_iam_role" "marwan_hub_eks_role" {
  name = "marwan-hub-eks-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "eks.amazonaws.com"
        }
      }
    ]
  })

  tags = {
    Name        = "marwan-hub-eks-role"
    Environment = var.environment
  }
}

resource "aws_iam_role_policy_attachment" "eks_cluster_policy" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKSClusterPolicy"
  role       = aws_iam_role.marwan_hub_eks_role.name
}

resource "aws_iam_role_policy_attachment" "eks_service_policy" {
  policy_arn = "arn:aws:iam::aws:policy/AmazonEKSServicePolicy"
  role       = aws_iam_role.marwan_hub_eks_role.name
}

# Outputs
output "cluster_id" {
  description = "EKS Cluster ID"
  value       = module.eks.cluster_id
}

output "cluster_endpoint" {
  description = "EKS Cluster Endpoint"
  value       = module.eks.cluster_endpoint
}

output "cluster_security_group_id" {
  description = "Security group ID attached to the cluster"
  value       = module.eks.cluster_security_group_id
}

output "vpc_id" {
  description = "VPC ID"
  value       = module.vpc.vpc_id
}

output "db_endpoint" {
  description = "RDS Endpoint"
  value       = aws_db_instance.marwan_hub_db.endpoint
  sensitive   = true
}

output "redis_endpoint" {
  description = "Redis Endpoint"
  value       = aws_elasticache_cluster.marwan_hub_redis.cache_nodes.0.address
}

output "s3_bucket" {
  description = "S3 Bucket Name"
  value       = aws_s3_bucket.marwan_hub_storage.bucket
}

output "kubeconfig" {
  description = "kubectl config as generated by the module"
  value       = module.eks.kubeconfig
  sensitive   = true
}
