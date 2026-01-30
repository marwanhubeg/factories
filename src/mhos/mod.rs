pub mod dashboard;
pub mod quality_gates;
pub mod optimizer;
pub mod monitor;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use crate::core::factory_manager::FactoryManager;

/// نظام MH-OS v2.2 الأساسي
pub struct MhosSystem {
    version: String,
    factory_manager: Arc<FactoryManager>,
    dashboard: dashboard::Dashboard,
    quality_gates: quality_gates::QualityGates,
    optimizer: optimizer::Optimizer,
    monitor: monitor::SystemMonitor,
    config: MhosConfig,
}

/// تكوين MH-OS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MhosConfig {
    pub auto_optimize: bool,
    pub quality_threshold: f32,
    pub monitoring_interval: u64,
    pub alert_enabled: bool,
    pub backup_enabled: bool,
    pub ai_assistant: bool,
}

impl Default for MhosConfig {
    fn default() -> Self {
        Self {
            auto_optimize: true,
            quality_threshold: 0.85,
            monitoring_interval: 30,
            alert_enabled: true,
            backup_enabled: true,
            ai_assistant: true,
        }
    }
}

/// حالة النظام
#[derive(Debug, Clone, Serialize)]
pub struct SystemStatus {
    pub overall_health: String,
    pub performance_score: f32,
    pub quality_score: f32,
    pub efficiency: f32,
    pub active_factories: usize,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

impl MhosSystem {
    /// إنشاء مثيل جديد لـ MH-OS
    pub fn new(factory_manager: Arc<FactoryManager>) -> Self {
        Self {
            version: "2.2".to_string(),
            factory_manager: factory_manager.clone(),
            dashboard: dashboard::Dashboard::new(factory_manager.clone()),
            quality_gates: quality_gates::QualityGates::new(),
            optimizer: optimizer::Optimizer::new(),
            monitor: monitor::SystemMonitor::new(),
            config: MhosConfig::default(),
        }
    }
    
    /// بدء تشغيل النظام
    pub async fn start(&self) {
        log::info!("🚀 بدء تشغيل MH-OS v{}", self.version);
        
        // بدء المراقبة
        self.monitor.start_monitoring(self.factory_manager.clone()).await;
        
        // التحقق من الجودة
        self.quality_gates.check_all(self.factory_manager.clone()).await;
        
        // تحسين النظام
        if self.config.auto_optimize {
            self.optimizer.optimize(self.factory_manager.clone()).await;
        }
        
        log::info!("✅ MH-OS جاهز للعمل");
    }
    
    /// الحصول على حالة النظام
    pub async fn get_status(&self) -> SystemStatus {
        let factories = self.factory_manager.list_factories();
        let active_count = factories.iter()
            .filter(|f| f.status() == "active")
            .count();
        
        let quality_scores: Vec<f32> = factories.iter()
            .map(|f| f.quality_score())
            .collect();
        
        let avg_quality = if !quality_scores.is_empty() {
            quality_scores.iter().sum::<f32>() / quality_scores.len() as f32
        } else { 0.0 };
        
        let performance = self.monitor.get_performance_metrics().await;
        let efficiency = (performance.cpu_efficiency + performance.memory_efficiency) / 2.0;
        
        let overall_health = if avg_quality >= self.config.quality_threshold && efficiency > 0.8 {
            "excellent".to_string()
        } else if avg_quality >= 0.7 && efficiency > 0.6 {
            "good".to_string()
        } else {
            "needs_attention".to_string()
        };
        
        SystemStatus {
            overall_health,
            performance_score: performance.overall_score,
            quality_score: avg_quality,
            efficiency,
            active_factories: active_count,
            issues: self.monitor.get_active_issues().await,
            recommendations: self.get_recommendations().await,
            last_update: chrono::Utc::now(),
        }
    }
    
    /// الحصول على التوصيات
    async fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        let status = self.get_status().await;
        
        if status.quality_score < self.config.quality_threshold {
            recommendations.push("تحسين جودة المصانع".to_string());
        }
        
        if status.efficiency < 0.7 {
            recommendations.push("تحسين كفاءة النظام".to_string());
        }
        
        if status.active_factories == 0 {
            recommendations.push("إنشاء مصانع جديدة".to_string());
        }
        
        recommendations
    }
    
    /// الحصول على لوحة التحكم
    pub fn get_dashboard(&self) -> &dashboard::Dashboard {
        &self.dashboard
    }
    
    /// الحصول على بوابات الجودة
    pub fn get_quality_gates(&self) -> &quality_gates::QualityGates {
        &self.quality_gates
    }
    
    /// تنفيذ تحسين النظام
    pub async fn run_optimization(&self) -> optimizer::OptimizationResult {
        self.optimizer.optimize(self.factory_manager.clone()).await
    }
    
    /// تحديث التكوين
    pub fn update_config(&mut self, config: MhosConfig) {
        self.config = config;
        log::info!("🔄 تم تحديث تكوين MH-OS");
    }
    
    /// الحصول على نسخة MH-OS
    pub fn version(&self) -> &str {
        &self.version
    }
}

/// وحدة مساعدة لـ MH-OS
pub mod utils {
    use super::*;
    
    /// تحليل البيانات الإحصائية
    pub fn analyze_statistics(data: &[f32]) -> Statistics {
        if data.is_empty() {
            return Statistics::default();
        }
        
        let sum: f32 = data.iter().sum();
        let mean = sum / data.len() as f32;
        
        let variance: f32 = data.iter()
            .map(|value| {
                let diff = mean - value;
                diff * diff
            })
            .sum::<f32>() / data.len() as f32;
        
        let std_dev = variance.sqrt();
        
        Statistics {
            mean,
            median: calculate_median(data),
            std_dev,
            min: *data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0),
            max: *data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0),
        }
    }
    
    /// حساب الوسيط
    fn calculate_median(data: &[f32]) -> f32 {
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }
    
    /// الإحصائيات
    #[derive(Debug, Clone, Serialize)]
    pub struct Statistics {
        pub mean: f32,
        pub median: f32,
        pub std_dev: f32,
        pub min: f32,
        pub max: f32,
    }
    
    impl Default for Statistics {
        fn default() -> Self {
            Self {
                mean: 0.0,
                median: 0.0,
                std_dev: 0.0,
                min: 0.0,
                max: 0.0,
            }
        }
    }
    
    /// المساعد الذكي
    pub struct AIAssistant {
        knowledge_base: HashMap<String, String>,
    }
    
    impl AIAssistant {
        pub fn new() -> Self {
            let mut knowledge_base = HashMap::new();
            
            // قاعدة المعرفة الأولية
            knowledge_base.insert("optimization".to_string(), 
                "لتحسين الأداء: 1. تحسين الخوارزميات 2. تحسين الذاكرة 3. موازنة الأحمال".to_string());
            knowledge_base.insert("quality".to_string(),
                "لتحسين الجودة: 1. فحص المدخلات 2. تحسين المعالجة 3. اختبار المخرجات".to_string());
            knowledge_base.insert("troubleshooting".to_string(),
                "لحل المشاكل: 1. التحقق من السجلات 2. اختبار المكونات 3. استعادة النسخ الاحتياطية".to_string());
            
            Self { knowledge_base }
        }
        
        pub fn get_advice(&self, topic: &str) -> Option<String> {
            self.knowledge_base.get(topic).cloned()
        }
        
        pub fn learn(&mut self, topic: String, advice: String) {
            self.knowledge_base.insert(topic, advice);
        }
    }
}
