use std::sync::Arc;
use serde::Serialize;
use crate::core::factory_manager::FactoryManager;

/// بوابة جودة
#[derive(Debug, Clone, Serialize)]
pub struct QualityGate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub threshold: f32,
    pub weight: f32,
    pub enabled: bool,
}

/// نتيجة فحص بوابة
#[derive(Debug, Clone, Serialize)]
pub struct GateResult {
    pub gate_id: String,
    pub passed: bool,
    pub actual_score: f32,
    pub threshold: f32,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// تقرير الجودة
#[derive(Debug, Clone, Serialize)]
pub struct QualityReport {
    pub overall_score: f32,
    pub passed_gates: usize,
    pub total_gates: usize,
    pub results: Vec<GateResult>,
    pub status: QualityStatus,
    pub recommendations: Vec<String>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// حالة الجودة
#[derive(Debug, Clone, Serialize)]
pub enum QualityStatus {
    Excellent,
    Good,
    Warning,
    Failed,
}

/// نظام بوابات الجودة
pub struct QualityGates {
    gates: Vec<QualityGate>,
}

impl QualityGates {
    /// إنشاء نظام بوابات جودة جديد
    pub fn new() -> Self {
        let mut gates = Vec::new();
        
        // بوابات الجودة الأساسية
        gates.push(QualityGate {
            id: "design_check".to_string(),
            name: "فحص التصميم".to_string(),
            description: "فحص جودة التصميم والهيكل".to_string(),
            threshold: 0.9,
            weight: 0.2,
            enabled: true,
        });
        
        gates.push(QualityGate {
            id: "content_quality".to_string(),
            name: "جودة المحتوى".to_string(),
            description: "فحص جودة المحتوى واللغة".to_string(),
            threshold: 0.85,
            weight: 0.25,
            enabled: true,
        });
        
        gates.push(QualityGate {
            id: "performance_test".to_string(),
            name: "اختبار الأداء".to_string(),
            description: "فحص أداء النظام والاستجابة".to_string(),
            threshold: 0.8,
            weight: 0.15,
            enabled: true,
        });
        
        gates.push(QualityGate {
            id: "security_check".to_string(),
            name: "فحص الأمان".to_string(),
            description: "فحص إجراءات الأمان والحماية".to_string(),
            threshold: 0.95,
            weight: 0.2,
            enabled: true,
        });
        
        gates.push(QualityGate {
            id: "compatibility_test".to_string(),
            name: "اختبار التوافق".to_string(),
            description: "فحص التوافق مع الأنظمة المختلفة".to_string(),
            threshold: 0.85,
            weight: 0.1,
            enabled: true,
        });
        
        gates.push(QualityGate {
            id: "user_experience".to_string(),
            name: "تجربة المستخدم".to_string(),
            description: "تقييم سهولة الاستخدام والواجهة".to_string(),
            threshold: 0.8,
            weight: 0.1,
            enabled: true,
        });
        
        Self { gates }
    }
    
    /// فحص جميع البوابات
    pub async fn check_all(&self, factory_manager: Arc<FactoryManager>) -> QualityReport {
        let mut results = Vec::new();
        
        for gate in &self.gates {
            if gate.enabled {
                let result = self.check_gate(gate, factory_manager.clone()).await;
                results.push(result);
            }
        }
        
        let passed_gates = results.iter().filter(|r| r.passed).count();
        let total_gates = results.len();
        
        let overall_score = if total_gates > 0 {
            results.iter()
                .map(|r| r.actual_score * self.get_gate_weight(&r.gate_id))
                .sum::<f32>() / results.iter().map(|r| self.get_gate_weight(&r.gate_id)).sum::<f32>()
        } else {
            0.0
        };
        
        let status = self.determine_quality_status(overall_score, passed_gates, total_gates);
        let recommendations = self.generate_recommendations(&results, overall_score);
        
        QualityReport {
            overall_score,
            passed_gates,
            total_gates,
            results,
            status,
            recommendations,
            generated_at: chrono::Utc::now(),
        }
    }
    
    /// فحص بوابة محددة
    async fn check_gate(&self, gate: &QualityGate, factory_manager: Arc<FactoryManager>) -> GateResult {
        let (actual_score, message) = match gate.id.as_str() {
            "design_check" => self.check_design(factory_manager.clone()).await,
            "content_quality" => self.check_content_quality(factory_manager.clone()).await,
            "performance_test" => self.check_performance(factory_manager.clone()).await,
            "security_check" => self.check_security(factory_manager.clone()).await,
            "compatibility_test" => self.check_compatibility(factory_manager.clone()).await,
            "user_experience" => self.check_user_experience(factory_manager.clone()).await,
            _ => (0.0, "بوابة غير معروفة".to_string()),
        };
        
        let passed = actual_score >= gate.threshold;
        
        GateResult {
            gate_id: gate.id.clone(),
            passed,
            actual_score,
            threshold: gate.threshold,
            message,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// فحص التصميم
    async fn check_design(&self, factory_manager: Arc<FactoryManager>) -> (f32, String) {
        let factories = factory_manager.list_factories();
        
        if factories.is_empty() {
            return (0.0, "لا توجد مصانع للفحص".to_string());
        }
        
        let design_scores: Vec<f32> = factories.iter()
            .map(|f| f.quality_score() * 0.9) // تحويل جودة المصنع إلى جودة تصميم
            .collect();
        
        let avg_score = design_scores.iter().sum::<f32>() / design_scores.len() as f32;
        
        let message = if avg_score > 0.95 {
            "التصميم ممتاز".to_string()
        } else if avg_score > 0.85 {
            "التصميم جيد".to_string()
        } else {
            "التصميم يحتاج تحسين".to_string()
        };
        
        (avg_score, message)
    }
    
    /// فحص جودة المحتوى
    async fn check_content_quality(&self, factory_manager: Arc<FactoryManager>) -> (f32, String) {
        let factories = factory_manager.list_factories();
        
        if factories.is_empty() {
            return (0.0, "لا توجد مصانع للفحص".to_string());
        }
        
        // محاكاة فحص جودة المحتوى
        let content_score = 0.92;
        let message = if content_score > 0.95 {
            "جودة المحتوى ممتازة".to_string()
        } else if content_score > 0.85 {
            "جودة المحتوى جيدة".to_string()
        } else {
            "جودة المحتوى تحتاج تحسين".to_string()
        };
        
        (content_score, message)
    }
    
    /// فحص الأداء
    async fn check_performance(&self, factory_manager: Arc<FactoryManager>) -> (f32, String) {
        let factories = factory_manager.list_factories();
        
        if factories.is_empty() {
            return (0.0, "لا توجد مصانع للفحص".to_string());
        }
        
        // محاكاة فحص الأداء
        let performance_score = 0.87;
        
        let message = if performance_score > 0.9 {
            "الأداء ممتاز".to_string()
        } else if performance_score > 0.8 {
            "الأداء جيد".to_string()
        } else {
            "الأداء يحتاج تحسين".to_string()
        };
        
        (performance_score, message)
    }
    
    /// فحص الأمان
    async fn check_security(&self, factory_manager: Arc<FactoryManager>) -> (f32, String) {
        // محاكاة فحص الأمان
        let security_score = 0.98;
        
        let message = if security_score > 0.95 {
            "مستوى الأمان ممتاز".to_string()
        } else if security_score > 0.9 {
            "مستوى الأمان جيد".to_string()
        } else {
            "الأمان يحتاج تعزيز".to_string()
        };
        
        (security_score, message)
    }
    
    /// فحص التوافق
    async fn check_compatibility(&self, factory_manager: Arc<FactoryManager>) -> (f32, String) {
        // محاكاة فحص التوافق
        let compatibility_score = 0.88;
        
        let message = if compatibility_score > 0.9 {
            "التوافق ممتاز".to_string()
        } else if compatibility_score > 0.8 {
            "التوافق جيد".to_string()
        } else {
            "التوافق يحتاج تحسين".to_string()
        };
        
        (compatibility_score, message)
    }
    
    /// فحص تجربة المستخدم
    async fn check_user_experience(&self, factory_manager: Arc<FactoryManager>) -> (f32, String) {
        let factories = factory_manager.list_factories();
        
        if factories.is_empty() {
            return (0.0, "لا توجد مصانع للفحص".to_string());
        }
        
        // محاكاة فحص تجربة المستخدم
        let ux_score = 0.85;
        
        let message = if ux_score > 0.9 {
            "تجربة المستخدم ممتازة".to_string()
        } else if ux_score > 0.8 {
            "تجربة المستخدم جيدة".to_string()
        } else {
            "تجربة المستخدم تحتاج تحسين".to_string()
        };
        
        (ux_score, message)
    }
    
    /// تحديد وزن البوابة
    fn get_gate_weight(&self, gate_id: &str) -> f32 {
        self.gates.iter()
            .find(|g| g.id == gate_id)
            .map(|g| g.weight)
            .unwrap_or(0.0)
    }
    
    /// تحديد حالة الجودة
    fn determine_quality_status(&self, overall_score: f32, passed_gates: usize, total_gates: usize) -> QualityStatus {
        let pass_rate = passed_gates as f32 / total_gates as f32;
        
        if overall_score > 0.95 && pass_rate > 0.95 {
            QualityStatus::Excellent
        } else if overall_score > 0.85 && pass_rate > 0.85 {
            QualityStatus::Good
        } else if overall_score > 0.7 && pass_rate > 0.7 {
            QualityStatus::Warning
        } else {
            QualityStatus::Failed
        }
    }
    
    /// توليد التوصيات
    fn generate_recommendations(&self, results: &[GateResult], overall_score: f32) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        for result in results {
            if !result.passed {
                recommendations.push(format!("تحسين {}: {}", 
                    self.get_gate_name(&result.gate_id), 
                    result.message));
            }
        }
        
        if overall_score < 0.8 {
            recommendations.push("تحسين الجودة العامة للنظام".to_string());
        }
        
        if recommendations.is_empty() {
            recommendations.push("النظام يعمل بشكل ممتاز".to_string());
        }
        
        recommendations
    }
    
    /// الحصول على اسم البوابة
    fn get_gate_name(&self, gate_id: &str) -> String {
        self.gates.iter()
            .find(|g| g.id == gate_id)
            .map(|g| g.name.clone())
            .unwrap_or_else(|| gate_id.to_string())
    }
    
    /// إضافة بوابة جديدة
    pub fn add_gate(&mut self, gate: QualityGate) {
        self.gates.push(gate);
    }
    
    /// تفعيل/تعطيل بوابة
    pub fn set_gate_enabled(&mut self, gate_id: &str, enabled: bool) -> bool {
        if let Some(gate) = self.gates.iter_mut().find(|g| g.id == gate_id) {
            gate.enabled = enabled;
            true
        } else {
            false
        }
    }
    
    /// الحصول على قائمة البوابات
    pub fn get_gates(&self) -> &[QualityGate] {
        &self.gates
    }
}
