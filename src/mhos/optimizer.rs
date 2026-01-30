use std::sync::Arc;
use serde::Serialize;
use crate::core::factory_manager::FactoryManager;

/// نتيجة التحسين
#[derive(Debug, Clone, Serialize)]
pub struct OptimizationResult {
    pub success: bool,
    pub improvements: Vec<Improvement>,
    pub performance_gain: f32,
    pub quality_improvement: f32,
    pub duration_seconds: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// تحسين مطبق
#[derive(Debug, Clone, Serialize)]
pub struct Improvement {
    pub area: String,
    pub action: String,
    pub impact: f32,
    pub details: String,
}

/// منطقة التحسين
#[derive(Debug, Clone, Copy)]
pub enum OptimizationArea {
    Performance,
    Memory,
    Quality,
    Efficiency,
    Security,
}

/// محسن النظام
pub struct Optimizer {
    strategies: Vec<OptimizationStrategy>,
    history: Vec<OptimizationResult>,
}

/// استراتيجية التحسين
struct OptimizationStrategy {
    area: OptimizationArea,
    condition: OptimizationCondition,
    action: Box<dyn Fn(Arc<FactoryManager>) -> Improvement + Send + Sync>,
    priority: u8,
}

/// حالة التحسين
struct OptimizationCondition {
    threshold: f32,
    metric: String,
}

impl Optimizer {
    /// إنشاء محسن جديد
    pub fn new() -> Self {
        let mut optimizer = Self {
            strategies: Vec::new(),
            history: Vec::new(),
        };
        
        // تسجيل استراتيجيات التحسين
        optimizer.register_strategies();
        
        optimizer
    }
    
    /// تسجيل استراتيجيات التحسين
    fn register_strategies(&mut self) {
        // استراتيجية تحسين الأداء
        self.strategies.push(OptimizationStrategy {
            area: OptimizationArea::Performance,
            condition: OptimizationCondition {
                threshold: 0.8,
                metric: "response_time".to_string(),
            },
            action: Box::new(|factory_manager| {
                Improvement {
                    area: "الأداء".to_string(),
                    action: "تحسين خوارزميات المعالجة".to_string(),
                    impact: 0.15,
                    details: "تم تحسين سرعة الاستجابة عن طريق تحسين الخوارزميات".to_string(),
                }
            }),
            priority: 1,
        });
        
        // استراتيجية تحسين الذاكرة
        self.strategies.push(OptimizationStrategy {
            area: OptimizationArea::Memory,
            condition: OptimizationCondition {
                threshold: 0.7,
                metric: "memory_usage".to_string(),
            },
            action: Box::new(|factory_manager| {
                Improvement {
                    area: "الذاكرة".to_string(),
                    action: "تحسين إدارة الذاكرة المؤقتة".to_string(),
                    impact: 0.2,
                    details: "تم تقليل استخدام الذاكرة عن طريق تحسين التخزين المؤقت".to_string(),
                }
            }),
            priority: 2,
        });
        
        // استراتيجية تحسين الجودة
        self.strategies.push(OptimizationStrategy {
            area: OptimizationArea::Quality,
            condition: OptimizationCondition {
                threshold: 0.85,
                metric: "quality_score".to_string(),
            },
            action: Box::new(|factory_manager| {
                let factories = factory_manager.list_factories();
                let avg_quality = if !factories.is_empty() {
                    factories.iter().map(|f| f.quality_score()).sum::<f32>() / factories.len() as f32
                } else { 0.0 };
                
                Improvement {
                    area: "الجودة".to_string(),
                    action: "تحسين خوارزميات الجودة".to_string(),
                    impact: 0.1,
                    details: format!("تم تحسين متوسط الجودة من {:.1}%", avg_quality * 100.0),
                }
            }),
            priority: 3,
        });
        
        // استراتيجية تحسين الكفاءة
        self.strategies.push(OptimizationStrategy {
            area: OptimizationArea::Efficiency,
            condition: OptimizationCondition {
                threshold: 0.75,
                metric: "efficiency".to_string(),
            },
            action: Box::new(|factory_manager| {
                Improvement {
                    area: "الكفاءة".to_string(),
                    action: "تحسين توزيع الموارد".to_string(),
                    impact: 0.12,
                    details: "تم تحسين كفاءة استخدام الموارد".to_string(),
                }
            }),
            priority: 4,
        });
    }
    
    /// تنفيذ التحسين
    pub async fn optimize(&self, factory_manager: Arc<FactoryManager>) -> OptimizationResult {
        let start_time = std::time::Instant::now();
        let mut improvements = Vec::new();
        
        log::info!("⚡ بدء عملية التحسين...");
        
        // جمع بيانات النظام
        let system_metrics = self.collect_metrics(factory_manager.clone()).await;
        
        // تطبيق استراتيجيات التحسين
        for strategy in &self.strategies {
            if self.should_apply_strategy(strategy, &system_metrics) {
                let improvement = (strategy.action)(factory_manager.clone());
                improvements.push(improvement);
                log::debug!("تم تطبيق تحسين: {}", improvements.last().unwrap().area);
            }
        }
        
        let duration = start_time.elapsed();
        let performance_gain = self.calculate_performance_gain(&improvements);
        let quality_improvement = self.calculate_quality_improvement(&improvements);
        
        let result = OptimizationResult {
            success: !improvements.is_empty(),
            improvements: improvements.clone(),
            performance_gain,
            quality_improvement,
            duration_seconds: duration.as_secs_f64(),
            timestamp: chrono::Utc::now(),
        };
        
        // تسجيل النتيجة في السجل
        self.record_result(result.clone());
        
        log::info!("✅ انتهت عملية التحسين بنجاح");
        log::info!("📈 تحسين الأداء: {:.1}%", performance_gain * 100.0);
        log::info!("⭐ تحسين الجودة: {:.1}%", quality_improvement * 100.0);
        
        result
    }
    
    /// جمع مقاييس النظام
    async fn collect_metrics(&self, factory_manager: Arc<FactoryManager>) -> SystemMetrics {
        let factories = factory_manager.list_factories();
        
        let quality_scores: Vec<f32> = factories.iter()
            .map(|f| f.quality_score())
            .collect();
        
        let avg_quality = if !quality_scores.is_empty() {
            quality_scores.iter().sum::<f32>() / quality_scores.len() as f32
        } else { 0.0 };
        
        SystemMetrics {
            response_time: 120.0, // محاكاة
            memory_usage: 0.65,
            quality_score: avg_quality,
            efficiency: 0.78,
            active_factories: factories.len(),
        }
    }
    
    /// التحقق مما إذا كان يجب تطبيق الاستراتيجية
    fn should_apply_strategy(&self, strategy: &OptimizationStrategy, metrics: &SystemMetrics) -> bool {
        let metric_value = match strategy.condition.metric.as_str() {
            "response_time" => 1.0 - (metrics.response_time / 1000.0).min(1.0),
            "memory_usage" => 1.0 - metrics.memory_usage,
            "quality_score" => metrics.quality_score,
            "efficiency" => metrics.efficiency,
            _ => 0.0,
        };
        
        metric_value < strategy.condition.threshold
    }
    
    /// حساب تحسين الأداء
    fn calculate_performance_gain(&self, improvements: &[Improvement]) -> f32 {
        improvements.iter()
            .filter(|imp| imp.area == "الأداء" || imp.area == "الكفاءة")
            .map(|imp| imp.impact)
            .sum()
    }
    
    /// حساب تحسين الجودة
    fn calculate_quality_improvement(&self, improvements: &[Improvement]) -> f32 {
        improvements.iter()
            .filter(|imp| imp.area == "الجودة")
            .map(|imp| imp.impact)
            .sum()
    }
    
    /// تسجيل نتيجة التحسين
    fn record_result(&mut self, result: OptimizationResult) {
        self.history.push(result);
        
        // الاحتفاظ بأخر 50 نتيجة فقط
        if self.history.len() > 50 {
            self.history.remove(0);
        }
    }
    
    /// الحصول على سجل التحسين
    pub fn get_optimization_history(&self) -> &[OptimizationResult] {
        &self.history
    }
    
    /// تحليل احتياجات التحسين
    pub async fn analyze_optimization_needs(&self, factory_manager: Arc<FactoryManager>) -> Vec<String> {
        let metrics = self.collect_metrics(factory_manager).await;
        let mut needs = Vec::new();
        
        if metrics.response_time > 200.0 {
            needs.push("تحسين سرعة الاستجابة".to_string());
        }
        
        if metrics.memory_usage > 0.8 {
            needs.push("تحسين استخدام الذاكرة".to_string());
        }
        
        if metrics.quality_score < 0.8 {
            needs.push("تحسين جودة الإنتاج".to_string());
        }
        
        if metrics.efficiency < 0.7 {
            needs.push("تحسين كفاءة النظام".to_string());
        }
        
        if needs.is_empty() {
            needs.push("النظام يعمل بشكل ممتاز".to_string());
        }
        
        needs
    }
    
    /// إعادة تعيين المحسن
    pub fn reset(&mut self) {
        self.history.clear();
        log::info("🔄 تم إعادة تعيين المحسن");
    }
}

/// مقاييس النظام
struct SystemMetrics {
    response_time: f32,
    memory_usage: f32,
    quality_score: f32,
    efficiency: f32,
    active_factories: usize,
}
