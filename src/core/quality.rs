//! نظام الجودة و MH-OS v2.2 Integration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// بوابة الجودة (Quality Gate)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub gate_type: GateType,
    pub criteria: Vec<QualityCriterion>,
    pub threshold: f64, // 0.0 - 1.0
    pub weight: f64, // وزن البوابة في النتيجة النهائية
    pub enabled: bool,
    pub auto_run: bool,
}

/// نوع بوابة الجودة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GateType {
    Technical,      // الجودة التقنية (كود، أداء)
    Functional,     // الجودة الوظيفية
    Security,       // الجودة الأمنية
    Usability,      // قابلية الاستخدام
    Performance,    // الأداء
    Compliance,     // الامتثال للمعايير
    Documentation,  // التوثيق
    Accessibility,  // إمكانية الوصول
}

impl GateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GateType::Technical => "technical",
            GateType::Functional => "functional",
            GateType::Security => "security",
            GateType::Usability => "usability",
            GateType::Performance => "performance",
            GateType::Compliance => "compliance",
            GateType::Documentation => "documentation",
            GateType::Accessibility => "accessibility",
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            GateType::Technical => "الجودة التقنية",
            GateType::Functional => "الجودة الوظيفية",
            GateType::Security => "الجودة الأمنية",
            GateType::Usability => "قابلية الاستخدام",
            GateType::Performance => "الأداء",
            GateType::Compliance => "الامتثال للمعايير",
            GateType::Documentation => "التوثيق",
            GateType::Accessibility => "إمكانية الوصول",
        }
    }
}

/// معيار الجودة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCriterion {
    pub id: String,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub weight: f64,
    pub check_type: CheckType,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// نوع الفحص
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckType {
    Existence,      // التحقق من الوجود
    Format,         // التحقق من التنسيق
    Content,        // التحقق من المحتوى
    Performance,    // فحص الأداء
    Security,       // فحص الأمان
    Compliance,     // فحص الامتثال
    Custom,         // فحص مخصص
}

/// نتيجة بوابة الجودة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    pub gate_id: String,
    pub gate_name: String,
    pub passed: bool,
    pub score: f64, // 0.0 - 1.0
    pub details: HashMap<String, CriterionResult>,
    pub execution_time_ms: u64,
    pub timestamp: DateTime<Utc>,
    pub recommendations: Vec<String>,
}

/// نتيجة المعيار
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriterionResult {
    pub criterion_id: String,
    pub criterion_name: String,
    pub passed: bool,
    pub score: f64,
    pub message: String,
    pub details: serde_json::Value,
}

/// تقرير الجودة الكامل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityReport {
    pub report_id: String,
    pub entity_id: String, // ID للمشروع أو المنتج
    pub entity_type: String,
    pub overall_score: f64,
    pub passed: bool,
    pub gate_results: HashMap<String, GateResult>,
    pub generated_at: DateTime<Utc>,
    pub generated_by: String,
    pub mhos_version: String,
}

/// نظام إدارة الجودة
pub struct QualityManager {
    gates: HashMap<String, QualityGate>,
    reports: HashMap<String, QualityReport>,
}

impl QualityManager {
    pub fn new() -> Self {
        let mut manager = Self {
            gates: HashMap::new(),
            reports: HashMap::new(),
        };
        
        // إضافة بوابات MH-OS v2.2 الافتراضية
        manager.initialize_mhos_gates();
        
        manager
    }
    
    fn initialize_mhos_gates(&mut self) {
        // البوابة 1: الجودة التقنية
        let technical_gate = QualityGate {
            id: "mhos_gate_1".to_string(),
            name: "الجودة التقنية - MH-OS".to_string(),
            description: "فحص الجودة التقنية للمنتج (SOLID Principles, Design Patterns, Code Coverage)".to_string(),
            gate_type: GateType::Technical,
            criteria: vec![
                QualityCriterion {
                    id: "tech_1".to_string(),
                    name: "مبادئ SOLID".to_string(),
                    description: "التحقق من تطبيق مبادئ SOLID في الكود".to_string(),
                    required: true,
                    weight: 0.3,
                    check_type: CheckType::Compliance,
                    parameters: HashMap::new(),
                },
                QualityCriterion {
                    id: "tech_2".to_string(),
                    name: "أنماط التصميم".to_string(),
                    description: "استخدام أنماط التصميم المناسبة".to_string(),
                    required: false,
                    weight: 0.2,
                    check_type: CheckType::Existence,
                    parameters: HashMap::new(),
                },
                QualityCriterion {
                    id: "tech_3".to_string(),
                    name: "تغطية الكود".to_string(),
                    description: "تغطية اختبارات لا تقل عن 80%".to_string(),
                    required: true,
                    weight: 0.3,
                    check_type: CheckType::Performance,
                    parameters: HashMap::from([("min_coverage".to_string(), serde_json::json!(80))]),
                },
                QualityCriterion {
                    id: "tech_4".to_string(),
                    name: "التعقيد الدوري".to_string(),
                    description: "التعقيد الدوري لا يتجاوز 15".to_string(),
                    required: true,
                    weight: 0.2,
                    check_type: CheckType::Performance,
                    parameters: HashMap::from([("max_complexity".to_string(), serde_json::json!(15))]),
                },
            ],
            threshold: 0.8,
            weight: 0.25,
            enabled: true,
            auto_run: true,
        };
        
        // البوابة 2: الأداء
        let performance_gate = QualityGate {
            id: "mhos_gate_2".to_string(),
            name: "الأداء - MH-OS".to_string(),
            description: "فحص معايير الأداء (سرعة التحميل، استهلاك الذاكرة، وقت الاستجابة)".to_string(),
            gate_type: GateType::Performance,
            criteria: vec![
                QualityCriterion {
                    id: "perf_1".to_string(),
                    name: "وقت الاستجابة".to_string(),
                    description: "وقت الاستجابة أقل من 200ms".to_string(),
                    required: true,
                    weight: 0.4,
                    check_type: CheckType::Performance,
                    parameters: HashMap::from([("max_response_time".to_string(), serde_json::json!(200))]),
                },
                QualityCriterion {
                    id: "perf_2".to_string(),
                    name: "استهلاك الذاكرة".to_string(),
                    description: "استهلاك الذاكرة أقل من 100MB".to_string(),
                    required: true,
                    weight: 0.3,
                    check_type: CheckType::Performance,
                    parameters: HashMap::from([("max_memory".to_string(), serde_json::json!(100))]),
                },
                QualityCriterion {
                    id: "perf_3".to_string(),
                    name: "وقت التحميل".to_string(),
                    description: "وقت التحميل الأولي أقل من 3 ثواني".to_string(),
                    required: true,
                    weight: 0.3,
                    check_type: CheckType::Performance,
                    parameters: HashMap::from([("max_load_time".to_string(), serde_json::json!(3000))]),
                },
            ],
            threshold: 0.85,
            weight: 0.25,
            enabled: true,
            auto_run: true,
        };
        
        // البوابة 3: الأمان
        let security_gate = QualityGate {
            id: "mhos_gate_3".to_string(),
            name: "الأمان - MH-OS".to_string(),
            description: "فحص معايير الأمان (OWASP Top 10، التشفير، الحماية)".to_string(),
            gate_type: GateType::Security,
            criteria: vec![
                QualityCriterion {
                    id: "sec_1".to_string(),
                    name: "OWASP Top 10".to_string(),
                    description: "التحقق من تطبيق OWASP Top 10".to_string(),
                    required: true,
                    weight: 0.4,
                    check_type: CheckType::Security,
                    parameters: HashMap::new(),
                },
                QualityCriterion {
                    id: "sec_2".to_string(),
                    name: "تشفير البيانات".to_string(),
                    description: "استخدام التشفير المناسب للبيانات الحساسة".to_string(),
                    required: true,
                    weight: 0.3,
                    check_type: CheckType::Existence,
                    parameters: HashMap::new(),
                },
                QualityCriterion {
                    id: "sec_3".to_string(),
                    name: "التحقق من المدخلات".to_string(),
                    description: "التحقق من جميع المدخلات وتنظيفها".to_string(),
                    required: true,
                    weight: 0.3,
                    check_type: CheckType::Security,
                    parameters: HashMap::new(),
                },
            ],
            threshold: 0.9,
            weight: 0.25,
            enabled: true,
            auto_run: true,
        };
        
        // البوابة 4: قابلية الاستخدام
        let usability_gate = QualityGate {
            id: "mhos_gate_4".to_string(),
            name: "قابلية الاستخدام - MH-OS".to_string(),
            description: "فحص قابلية الاستخدام (تجربة المستخدم، إمكانية الوصول، التجاوب)".to_string(),
            gate_type: GateType::Usability,
            criteria: vec![
                QualityCriterion {
                    id: "usab_1".to_string(),
                    name: "تجربة المستخدم".to_string(),
                    description: "تقييم تجربة المستخدم (يجب أن تكون 8/10 على الأقل)".to_string(),
                    required: true,
                    weight: 0.3,
                    check_type: CheckType::Content,
                    parameters: HashMap::from([("min_ux_score".to_string(), serde_json::json!(8))]),
                },
                QualityCriterion {
                    id: "usab_2".to_string(),
                    name: "إمكانية الوصول".to_string(),
                    description: "تطبيق معايير WCAG 2.1".to_string(),
                    required: true,
                    weight: 0.3,
                    check_type: CheckType::Accessibility,
                    parameters: HashMap::new(),
                },
                QualityCriterion {
                    id: "usab_3".to_string(),
                    name: "التصميم المتجاوب".to_string(),
                    description: "العمل على جميع أحجام الشاشات".to_string(),
                    required: true,
                    weight: 0.2,
                    check_type: CheckType::Compliance,
                    parameters: HashMap::new(),
                },
                QualityCriterion {
                    id: "usab_4".to_string(),
                    name: "سهولة التنقل".to_string(),
                    description: "تنقل سهل ومباشر".to_string(),
                    required: true,
                    weight: 0.2,
                    check_type: CheckType::Usability,
                    parameters: HashMap::new(),
                },
            ],
            threshold: 0.75,
            weight: 0.25,
            enabled: true,
            auto_run: true,
        };
        
        self.gates.insert(technical_gate.id.clone(), technical_gate);
        self.gates.insert(performance_gate.id.clone(), performance_gate);
        self.gates.insert(security_gate.id.clone(), security_gate);
        self.gates.insert(usability_gate.id.clone(), usability_gate);
    }
    
    pub fn add_gate(&mut self, gate: QualityGate) {
        self.gates.insert(gate.id.clone(), gate);
    }
    
    pub fn remove_gate(&mut self, gate_id: &str) {
        self.gates.remove(gate_id);
    }
    
    pub fn get_gate(&self, gate_id: &str) -> Option<&QualityGate> {
        self.gates.get(gate_id)
    }
    
    pub fn list_gates(&self) -> Vec<&QualityGate> {
        self.gates.values().collect()
    }
    
    pub fn run_quality_check(&self, entity_id: &str, entity_type: &str, data: serde_json::Value) -> QualityReport {
        let mut gate_results = HashMap::new();
        
        for gate in self.gates.values().filter(|g| g.enabled) {
            let result = self.run_gate_check(gate, &data);
            gate_results.insert(gate.id.clone(), result);
        }
        
        // حساب النتيجة الإجمالية
        let overall_score = self.calculate_overall_score(&gate_results);
        let passed = overall_score >= 0.8; // 80% كحد أدنى
        
        QualityReport {
            report_id: format!("qr_{}", uuid::Uuid::new_v4()),
            entity_id: entity_id.to_string(),
            entity_type: entity_type.to_string(),
            overall_score,
            passed,
            gate_results,
            generated_at: Utc::now(),
            generated_by: "MH-OS v2.2".to_string(),
            mhos_version: "2.2.0".to_string(),
        }
    }
    
    fn run_gate_check(&self, gate: &QualityGate, data: &serde_json::Value) -> GateResult {
        let mut details = HashMap::new();
        let mut total_score = 0.0;
        let mut total_weight = 0.0;
        
        for criterion in &gate.criteria {
            let criterion_result = self.run_criterion_check(criterion, data);
            total_score += criterion_result.score * criterion.weight;
            total_weight += criterion.weight;
            details.insert(criterion.id.clone(), criterion_result);
        }
        
        let gate_score = if total_weight > 0.0 {
            total_score / total_weight
        } else {
            0.0
        };
        
        let passed = gate_score >= gate.threshold;
        
        GateResult {
            gate_id: gate.id.clone(),
            gate_name: gate.name.clone(),
            passed,
            score: gate_score,
            details,
            execution_time_ms: 100, // قيمة افتراضية
            timestamp: Utc::now(),
            recommendations: self.generate_recommendations(passed, gate_score),
        }
    }
    
    fn run_criterion_check(&self, criterion: &QualityCriterion, _data: &serde_json::Value) -> CriterionResult {
        // هذا مجرد تنفيذ تجريبي
        // في التنفيذ الحقيقي، سيكون هناك منطق تحقق حقيقي
        
        let passed = rand::random::<f64>() > 0.3; // 70% احتمال للنجاح
        let score = if passed { 1.0 } else { 0.5 };
        
        CriterionResult {
            criterion_id: criterion.id.clone(),
            criterion_name: criterion.name.clone(),
            passed,
            score,
            message: if passed {
                "✅ اجتاز الاختبار".to_string()
            } else {
                "❌ لم يجتز الاختبار".to_string()
            },
            details: serde_json::json!({"simulated": true}),
        }
    }
    
    fn calculate_overall_score(&self, gate_results: &HashMap<String, GateResult>) -> f64 {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;
        
        for (gate_id, result) in gate_results {
            if let Some(gate) = self.gates.get(gate_id) {
                total_score += result.score * gate.weight;
                total_weight += gate.weight;
            }
        }
        
        if total_weight > 0.0 {
            total_score / total_weight
        } else {
            0.0
        }
    }
    
    fn generate_recommendations(&self, passed: bool, score: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if !passed {
            recommendations.push("تحسين الجودة لتجاوز الحد الأدنى المطلوب".to_string());
            
            if score < 0.5 {
                recommendations.push("مراجعة شاملة للمنتج وإعادة التصميم إذا لزم الأمر".to_string());
            } else if score < 0.7 {
                recommendations.push("تحسين النقاط الضعيفة في المنتج".to_string());
            } else {
                recommendations.push("تحسينات طفيفة لتجاوز الحد الأدنى".to_string());
            }
        } else {
            if score >= 0.9 {
                recommendations.push("✅ الجودة ممتازة - يمكن المضي قدماً".to_string());
            } else if score >= 0.8 {
                recommendations.push("👌 الجودة جيدة - بعض التحسينات الطفيفة ممكنة".to_string());
            }
        }
        
        recommendations
    }
    
    pub fn save_report(&mut self, report: QualityReport) {
        self.reports.insert(report.report_id.clone(), report);
    }
    
    pub fn get_report(&self, report_id: &str) -> Option<&QualityReport> {
        self.reports.get(report_id)
    }
    
    pub fn list_reports(&self) -> Vec<&QualityReport> {
        self.reports.values().collect()
    }
}

/// لوحة تحكم MH-OS المصغرة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniDashboard {
    pub active_tasks: u32,
    pub processing_speed: f64, // عمليات/ثانية
    pub output_quality: f64,   // 0.0 - 1.0
    pub ai_assistance: bool,
    pub last_updates: Vec<Update>,
    pub system_health: SystemHealth,
}

impl MiniDashboard {
    pub fn new() -> Self {
        Self {
            active_tasks: 0,
            processing_speed: 0.0,
            output_quality: 1.0,
            ai_assistance: true,
            last_updates: Vec::new(),
            system_health: SystemHealth::Excellent,
        }
    }
    
    pub fn to_html(&self) -> String {
        format!(
            r#"<div class="mhos-mini-dashboard">
                <h4>📊 MH-OS v2.2 - Mini Dashboard</h4>
                <div class="metric">
                    <span class="label">🏭 المهام النشطة:</span>
                    <span class="value">{}/3 ✅</span>
                </div>
                <div class="metric">
                    <span class="label">⚡ سرعة المعالجة:</span>
                    <span class="value">{}%</span>
                </div>
                <div class="metric">
                    <span class="label">🎯 جودة المخرجات:</span>
                    <span class="value">{}/10</span>
                </div>
                <div class="metric">
                    <span class="label">🤖 الذكاء الاصطناعي:</span>
                    <span class="value">{}</span>
                </div>
            </div>"#,
            self.active_tasks,
            (self.processing_speed * 100.0) as u32,
            (self.output_quality * 10.0) as u32,
            if self.ai_assistance { "ACTIVE 50x" } else { "INACTIVE" }
        )
    }
    
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "active_tasks": self.active_tasks,
            "processing_speed": self.processing_speed,
            "output_quality": self.output_quality,
            "ai_assistance": self.ai_assistance,
            "system_health": self.system_health.as_str(),
            "timestamp": Utc::now().to_rfc3339()
        })
    }
}

/// تحديث النظام
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    pub id: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub applied: bool,
}

/// صحة النظام
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemHealth {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

impl SystemHealth {
    pub fn as_str(&self) -> &'static str {
        match self {
            SystemHealth::Excellent => "ممتاز",
            SystemHealth::Good => "جيد",
            SystemHealth::Fair => "متوسط",
            SystemHealth::Poor => "ضعيف",
            SystemHealth::Critical => "حرج",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quality_manager_initialization() {
        let manager = QualityManager::new();
        let gates = manager.list_gates();
        
        assert_eq!(gates.len(), 4); // بوابات MH-OS الأربعة
        assert!(gates.iter().any(|g| g.name.contains("الجودة التقنية")));
        assert!(gates.iter().any(|g| g.name.contains("الأداء")));
    }
    
    #[test]
    fn test_mini_dashboard() {
        let dashboard = MiniDashboard::new();
        let html = dashboard.to_html();
        
        assert!(html.contains("MH-OS v2.2"));
        assert!(html.contains("Mini Dashboard"));
        assert!(html.contains("المهام النشطة"));
    }
    
    #[test]
    fn test_gate_type_display() {
        assert_eq!(GateType::Technical.display_name(), "الجودة التقنية");
        assert_eq!(GateType::Security.as_str(), "security");
    }
}
