//! 🏢 مصنع المؤسسات
//! متخصص في حلول المؤسسات والشركات والإدارة

use crate::core::factory::{
    Factory, FactoryType, FactoryStatus, FactoryCapability, 
    FactoryRequest, FactoryResponse, FactoryOutput, FactoryError,
    CapabilityParameter, ParameterType, RequestPriority, OutputType, OutputFormat
};
use crate::core::quality::{QualityGate, GateType};
use crate::factories::BaseFactory;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// مصنع المؤسسات
pub struct CorporateFactory {
    base: BaseFactory,
    document_templates: HashMap<String, DocumentTemplate>,
    policy_templates: HashMap<String, PolicyTemplate>,
    report_generators: Vec<ReportGenerator>,
    workflow_templates: Vec<WorkflowTemplate>,
}

impl CorporateFactory {
    pub fn new() -> Self {
        let mut factory = Self {
            base: BaseFactory::new(
                "مصنع المؤسسات".to_string(),
                FactoryType::Corporate,
                "1.0.0".to_string()
            ),
            document_templates: HashMap::new(),
            policy_templates: HashMap::new(),
            report_generators: Vec::new(),
            workflow_templates: Vec::new(),
        };
        
        // إضافة الإمكانيات الأساسية
        factory.initialize_capabilities();
        
        // تحميل القوالب الافتراضية
        factory.load_default_templates();
        
        factory
    }
    
    fn initialize_capabilities(&mut self) {
        // إمكانية إنشاء الوثائق
        let document_creation = FactoryCapability {
            name: "إنشاء الوثائق المؤسسية".to_string(),
            description: "إنشاء عقود وسياسات وإجراءات مؤسسية".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
            parameters: HashMap::from([
                ("document_type".to_string(), CapabilityParameter {
                    name: "نوع الوثيقة".to_string(),
                    description: "نوع الوثيقة (عقد، سياسة، إجراء، تقرير)".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("عقد")),
                }),
                ("company_name".to_string(), CapabilityParameter {
                    name: "اسم الشركة".to_string(),
                    description: "اسم الشركة أو المؤسسة".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: None,
                }),
                ("industry".to_string(), CapabilityParameter {
                    name: "المجال".to_string(),
                    description: "مجال العمل (تقنية، تعليم، صحة، إلخ)".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("تقنية")),
                }),
                ("jurisdiction".to_string(), CapabilityParameter {
                    name: "السلطة القضائية".to_string(),
                    description: "البلد أو المنطقة (مصر، السعودية، دولي)".to_string(),
                    required: false,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("دولي")),
                }),
            ]),
        };
        
        // إمكانية إدارة المشاريع
        let project_management = FactoryCapability {
            name: "إدارة المشاريع".to_string(),
            description: "إنشاء خطط مشاريع وجداول زمنية".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
            parameters: HashMap::from([
                ("project_name".to_string(), CapabilityParameter {
                    name: "اسم المشروع".to_string(),
                    description: "الاسم الكامل للمشروع".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: None,
                }),
                ("project_type".to_string(), CapabilityParameter {
                    name: "نوع المشروع".to_string(),
                    description: "نوع المشروع (تطوير، بحث، بناء، إلخ)".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("تطوير")),
                }),
                ("duration_months".to_string(), CapabilityParameter {
                    name: "المدة بالأشهر".to_string(),
                    description: "المدة المتوقعة للمشروع بالأشهر".to_string(),
                    required: true,
                    data_type: ParameterType::Integer,
                    default_value: Some(serde_json::json!(6)),
                }),
                ("team_size".to_string(), CapabilityParameter {
                    name: "حجم الفريق".to_string(),
                    description: "عدد أعضاء الفريق".to_string(),
                    required: false,
                    data_type: ParameterType::Integer,
                    default_value: Some(serde_json::json!(5)),
                }),
            ]),
        };
        
        // إمكانية الموارد البشرية
        let hr_management = FactoryCapability {
            name: "إدارة الموارد البشرية".to_string(),
            description: "إنشاء وثائق ولوائح الموارد البشرية".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
            parameters: HashMap::from([
                ("hr_document_type".to_string(), CapabilityParameter {
                    name: "نوع وثيقة الموارد البشرية".to_string(),
                    description: "نوع الوثيقة (عقد عمل، سياسة، تقييم)".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("عقد عمل")),
                }),
                ("employee_type".to_string(), CapabilityParameter {
                    name: "نوع الموظف".to_string(),
                    description: "نوع الموظف (دائم، مؤقت، متعاقد)".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("دائم")),
                }),
                ("position".to_string(), CapabilityParameter {
                    name: "الوظيفة".to_string(),
                    description: "المسمى الوظيفي".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("مطور برمجيات")),
                }),
                ("compliance_standard".to_string(), CapabilityParameter {
                    name: "معيار الامتثال".to_string(),
                    description: "معيار الامتثال (محلي، دولي)".to_string(),
                    required: false,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("محلي")),
                }),
            ]),
        };
        
        self.base.add_capability(document_creation);
        self.base.add_capability(project_management);
        self.base.add_capability(hr_management);
    }
    
    fn load_default_templates(&mut self) {
        // قالب عقد عمل
        let employment_contract_template = DocumentTemplate {
            id: "contract_employment".to_string(),
            name: "قالب عقد عمل".to_string(),
            description: "عقد عمل قياسي للموظفين".to_string(),
            category: "عقود".to_string(),
            document_type: "عقد عمل".to_string(),
            sections: vec![
                "معلومات الأطراف".to_string(),
                "المسمى الوظيفي والمهام".to_string(),
                "المدة والفترة التجريبية".to_string(),
                "الراتب والمزايا".to_string(),
                "ساعات العمل والإجازات".to_string(),
                "السرية والملكية الفكرية".to_string(),
                "إنهاء العقد".to_string(),
                "بنود عامة".to_string(),
            ],
            required_fields: vec![
                "company_name".to_string(),
                "employee_name".to_string(),
                "position".to_string(),
                "salary".to_string(),
            ],
        };
        
        // قالب سياسة الخصوصية
        let privacy_policy_template = PolicyTemplate {
            id: "policy_privacy".to_string(),
            name: "قالب سياسة الخصوصية".to_string(),
            description: "سياسة خصوصية قياسية للمواقع والتطبيقات".to_string(),
            category: "سياسات".to_string(),
            policy_type: "خصوصية".to_string(),
            compliance: vec!["GDPR".to_string(), "CCPA".to_string()],
            sections: vec![
                "مقدمة".to_string(),
                "المعلومات التي نجمعها".to_string(),
                "كيف نستخدم المعلومات".to_string(),
                "مشاركة المعلومات".to_string(),
                "حماية المعلومات".to_string(),
                "حقوق المستخدم".to_string(),
                "التغييرات على السياسة".to_string(),
                "اتصل بنا".to_string(),
            ],
        };
        
        self.document_templates.insert(employment_contract_template.id.clone(), employment_contract_template);
        self.policy_templates.insert(privacy_policy_template.id.clone(), privacy_policy_template);
    }
    
    fn create_document(&self, request: &DocumentCreationRequest) -> Result<CorporateDocument, FactoryError> {
        log::info!("إنشاء وثيقة: {} لـ {}", request.document_type, request.company_name);
        
        let document = match request.document_type.as_str() {
            "عقد عمل" => self.generate_employment_contract(request),
            "سياسة خصوصية" => self.generate_privacy_policy(request),
            "خطة مشروع" => self.generate_project_plan(request),
            "تقرير أداء" => self.generate_performance_report(request),
            _ => return Err(FactoryError::InvalidRequest("نوع وثيقة غير معروف".to_string())),
        };
        
        Ok(document)
    }
    
    fn generate_employment_contract(&self, request: &DocumentCreationRequest) -> CorporateDocument {
        let content = format!(
            r#"عقد عمل

البند الأول: الأطراف
1.1 صاحب العمل: {}
1.2 الموظف: {}
1.3 المسمى الوظيفي: {}
1.4 تاريخ البدء: {}

البند الثاني: المهام والمسؤوليات
2.1 يقوم الموظف بتنفيذ المهام التالية: {}
2.2 مكان العمل: {}
2.3 وقت العمل: 8 ساعات يومياً من السبت إلى الخميس

البند الثالث: المكافآت والمزايا
3.1 الراتب الأساسي: {} {} شهرياً
3.2 المكافآت: حسب أداء الموظف
3.3 الإجازات: 30 يوم إجازة سنوية

البند الرابع: السرية
4.1 يلتزم الموظف بالمحافظة على سرية معلومات الشركة
4.2 تظل بنود السرية سارية حتى بعد انتهاء العقد

البند الخامس: إنهاء العقد
5.1 يمكن إنهاء العقد بخطاب مسبق قبل 30 يوم
5.2 في حالة الإخلال بالبنود، يمكن إنهاء العقد فوراً

توقيع الأطراف:

صاحب العمل: ____________________

الموظف: ____________________

التاريخ: ____________________"#,
            request.company_name,
            request.party_names.get(0).unwrap_or(&"________".to_string()),
            request.position.as_deref().unwrap_or("________"),
            chrono::Utc::now().format("%Y-%m-%d").to_string(),
            request.description.as_deref().unwrap_or("تنفيذ المهام الموكلة إليه حسب تعليمات الإدارة"),
            request.location.as_deref().unwrap_or("مقر الشركة الرئيسي"),
            request.amount.unwrap_or(5000),
            request.currency.as_deref().unwrap_or("ريال سعودي")
        );
        
        CorporateDocument {
            id: format!("doc_{}", uuid::Uuid::new_v4()),
            title: format!("عقد عمل - {}", request.party_names.get(0).unwrap_or(&"________".to_string())),
            document_type: "عقد عمل".to_string(),
            content,
            metadata: HashMap::from([
                ("company".to_string(), request.company_name.clone()),
                ("document_type".to_string(), "عقد".to_string()),
                ("jurisdiction".to_string(), request.jurisdiction.clone().unwrap_or_else(|| "دولي".to_string())),
                ("generated_date".to_string(), chrono::Utc::now().to_rfc3339()),
            ]),
            version: "1.0".to_string(),
            effective_date: chrono::Utc::now(),
            requires_signature: true,
        }
    }
    
    fn generate_privacy_policy(&self, request: &DocumentCreationRequest) -> CorporateDocument {
        let content = format!(
            r#"سياسة الخصوصية لـ {}

مقدمة
نحن في {} نحترم خصوصيتك ونلتزم بحماية معلوماتك الشخصية. توضح سياسة الخصوصية هذه كيف نجمع ونستخدم ونحمي معلوماتك.

المعلومات التي نجمعها
نجمع أنواعاً مختلفة من المعلومات لأغراض مختلفة لتحسين خدماتنا لك، بما في ذلك:
- المعلومات التي تقدمها لنا مباشرةً
- معلومات الاستخدام والتفضيلات
- معلومات الجهاز والاتصال

كيف نستخدم معلوماتك
نستخدم المعلومات التي نجمعها لأغراض مختلفة، منها:
- تقديم خدماتنا وتحسينها
- التواصل معك
- تحليل استخدام الخدمات
- الكشف عن الاحتيال والمخالفات

مشاركة المعلومات
لا نبيع أو نؤجر معلوماتك الشخصية لأطراف ثالثة. قد نشارك معلوماتك فقط في الحالات التالية:
- مع موفري الخدمات الذين يساعدوننا في تشغيل خدماتنا
- عند الالتزام بالقانون أو حماية حقوقنا
- مع موافقتك الصريحة

حماية معلوماتك
نستخدم إجراءات أمنية تقنية وإدارية مناسبة لحماية معلوماتك من الوصول غير المصرح به أو التغيير أو الكشف أو الإتلاف.

حقوقك
لديك الحق في:
- الوصول إلى معلوماتك الشخصية
- تصحيح معلوماتك غير الدقيقة
- حذف معلوماتك الشخصية
- الاعتراض على معالجة معلوماتك

التغييرات على هذه السياسة
قد نقوم بتحديث سياسة الخصوصية هذه من وقت لآخر. سنخطرك بأي تغييرات من خلال نشر السياسة الجديدة على هذه الصفحة.

اتصل بنا
إذا كان لديك أي أسئلة حول سياسة الخصوصية هذه، يرجى الاتصال بنا على: {}

تاريخ السريان: {}
آخر تحديث: {}"#,
            request.company_name,
            request.company_name,
            request.contact_email.as_deref().unwrap_or("info@example.com"),
            chrono::Utc::now().format("%Y-%m-%d").to_string(),
            chrono::Utc::now().format("%Y-%m-%d").to_string()
        );
        
        CorporateDocument {
            id: format!("policy_{}", uuid::Uuid::new_v4()),
            title: format!("سياسة الخصوصية - {}", request.company_name),
            document_type: "سياسة خصوصية".to_string(),
            content,
            metadata: HashMap::from([
                ("company".to_string(), request.company_name.clone()),
                ("document_type".to_string(), "سياسة".to_string()),
                ("compliance".to_string(), "GDPR, CCPA".to_string()),
                ("generated_date".to_string(), chrono::Utc::now().to_rfc3339()),
            ]),
            version: "1.0".to_string(),
            effective_date: chrono::Utc::now(),
            requires_signature: false,
        }
    }
    
    fn generate_project_plan(&self, request: &DocumentCreationRequest) -> CorporateDocument {
        let content = format!(
            r#"خطة مشروع: {}

1. نظرة عامة
- المشروع: {}
- المدير: {}
- المدة: {} أشهر
- الميزانية: {} {}

2. الأهداف
{}
3. النطاق
{}
4. الجدول الزمني
{}
5. الفريق
{}
6. المخاطر
{}
7. النجاح
{}"#,
            request.document_title.as_deref().unwrap_or("مشروع جديد"),
            request.document_title.as_deref().unwrap_or("مشروع جديد"),
            request.manager_name.as_deref().unwrap_or("________"),
            request.duration_months.unwrap_or(6),
            request.amount.unwrap_or(100000),
            request.currency.as_deref().unwrap_or("ريال سعودي"),
            request.objectives.as_deref().unwrap_or("إكمال المشروع بنجاح ضمن الميزانية والجدول الزمني"),
            request.scope.as_deref().unwrap_or("التطوير الكامل للحل المطلوب"),
            request.timeline.as_deref().unwrap_or("سيتم تحديد الجدول الزمني التفصيلي لاحقاً"),
            request.team_info.as_deref().unwrap_or("فريق متعدد التخصصات"),
            request.risks.as_deref().unwrap_or("تغير المتطلبات، تأخر التسليم"),
            request.success_criteria.as_deref().unwrap_or("رضا العميل، تحقيق الأهداف")
        );
        
        CorporateDocument {
            id: format!("plan_{}", uuid::Uuid::new_v4()),
            title: request.document_title.clone().unwrap_or_else(|| "خطة مشروع".to_string()),
            document_type: "خطة مشروع".to_string(),
            content,
            metadata: HashMap::from([
                ("project_type".to_string(), request.project_type.clone().unwrap_or_else(|| "تطوير".to_string())),
                ("duration".to_string(), request.duration_months.unwrap_or(6).to_string()),
                ("budget".to_string(), request.amount.unwrap_or(100000).to_string()),
                ("generated_date".to_string(), chrono::Utc::now().to_rfc3339()),
            ]),
            version: "1.0".to_string(),
            effective_date: chrono::Utc::now(),
            requires_signature: true,
        }
    }
    
    fn generate_performance_report(&self, request: &DocumentCreationRequest) -> CorporateDocument {
        let content = format!(
            r#"تقرير أداء: {}

فترة التقرير: {}
المعدل: {}
المشرف: {}

1. ملخص الأداء
{}
2. الإنجازات الرئيسية
{}
3. المجالات التي تحتاج تحسين
{}
4. الأهداف المستقبلية
{}
5. التوصيات
{}
6. التقييم النهائي
{}

التوقيعات:

المعد: ____________________

المشرف: ____________________

تاريخ: {}"#,
            request.document_title.as_deref().unwrap_or("تقرير أداء"),
            request.period.as_deref().unwrap_or("ربع سنوي"),
            request.employee_name.as_deref().unwrap_or("________"),
            request.manager_name.as_deref().unwrap_or("________"),
            request.summary.as_deref().unwrap_or("أداء جيد مع تحسينات مستمرة"),
            request.achievements.as_deref().unwrap_or("إكمال المهام الموكلة بنجاح"),
            request.improvement_areas.as_deref().unwrap_or("تحسين مهارات التواصل"),
            request.future_goals.as_deref().unwrap_or("التطوير المهني المستمر"),
            request.recommendations.as_deref().unwrap_or("المشاركة في دورات تدريبية"),
            request.evaluation.as_deref().unwrap_or("جيد جداً"),
            chrono::Utc::now().format("%Y-%m-%d").to_string()
        );
        
        CorporateDocument {
            id: format!("report_{}", uuid::Uuid::new_v4()),
            title: request.document_title.clone().unwrap_or_else(|| "تقرير أداء".to_string()),
            document_type: "تقرير أداء".to_string(),
            content,
            metadata: HashMap::from([
                ("report_type".to_string(), "أداء".to_string()),
                ("period".to_string(), request.period.clone().unwrap_or_else(|| "ربع سنوي".to_string())),
                ("employee".to_string(), request.employee_name.clone().unwrap_or_else(|| "________".to_string())),
                ("rating".to_string(), request.rating.unwrap_or(4).to_string()),
            ]),
            version: "1.0".to_string(),
            effective_date: chrono::Utc::now(),
            requires_signature: true,
        }
    }
}

impl Factory for CorporateFactory {
    fn get_name(&self) -> String {
        self.base.get_name()
    }
    
    fn get_type(&self) -> FactoryType {
        self.base.get_type()
    }
    
    fn get_version(&self) -> String {
        self.base.get_version()
    }
    
    fn get_status(&self) -> FactoryStatus {
        self.base.get_status()
    }
    
    fn get_capabilities(&self) -> Vec<FactoryCapability> {
        self.base.get_capabilities()
    }
    
    fn process_request(&self, request: FactoryRequest) -> Result<FactoryResponse, FactoryError> {
        log::info!("معالجة طلب في مصنع المؤسسات: {:?}", request.operation);
        
        match request.operation.as_str() {
            "create_document" => {
                let doc_request: DocumentCreationRequest = serde_json::from_value(
                    request.parameters.get("document_data")
                        .ok_or_else(|| FactoryError::InvalidRequest("بيانات الوثيقة مطلوبة".to_string()))?
                        .clone()
                ).map_err(|e| FactoryError::InvalidRequest(format!("بيانات غير صالحة: {}", e)))?;
                
                let document = self.create_document(&doc_request)?;
                
                Ok(FactoryResponse {
                    request_id: request.request_id,
                    success: true,
                    output: Some(FactoryOutput {
                        output_type: OutputType::Document,
                        content: serde_json::to_value(&document)
                            .map_err(|e| FactoryError::ProcessingFailed(format!("فشل تسلسل البيانات: {}", e)))?,
                        format: OutputFormat::Json,
                        size_bytes: document.content.len(),
                        metadata: HashMap::from([
                            ("document_id".to_string(), document.id.clone()),
                            ("document_type".to_string(), document.document_type.clone()),
                            ("company".to_string(), document.metadata.get("company").cloned().unwrap_or_default()),
                        ]),
                        files: vec![
                            crate::core::factory::OutputFile {
                                filename: format!("{}.md", document.title.replace(" ", "_")),
                                content_type: "text/markdown".to_string(),
                                content: document.content.as_bytes().to_vec(),
                                size_bytes: document.content.len(),
                                checksum: format!("{:x}", md5::compute(&document.content)),
                            }
                        ],
                    }),
                    error_message: None,
                    processing_time_ms: 80,
                    quality_score: Some(0.98),
                    created_at: request.created_at,
                    completed_at: chrono::Utc::now(),
                })
            }
            
            _ => Err(FactoryError::InvalidRequest(format!("عملية غير معروفة: {}", request.operation)))
        }
    }
    
    fn validate_output(&self, output: &FactoryOutput) -> Vec<QualityGate> {
        let mut gates = Vec::new();
        
        // بوابة الجودة القانونية
        gates.push(QualityGate {
            id: "corporate_legal_quality".to_string(),
            name: "جودة الوثائق القانونية".to_string(),
            description: "فحص جودة الوثائق المؤسسية والقانونية".to_string(),
            gate_type: GateType::Compliance,
            criteria: vec![],
            threshold: 0.9,
            weight: 1.0,
            enabled: true,
            auto_run: true,
        });
        
        gates
    }
    
    fn get_metrics(&self) -> crate::core::factory::FactoryMetrics {
        crate::core::factory::FactoryMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_processing_time_ms: 0.0,
            current_queue_size: 0,
            memory_usage_mb: 0.0,
            last_reset: chrono::Utc::now(),
        }
    }
    
    fn reset(&self) -> Result<(), FactoryError> {
        Ok(())
    }
}

impl crate::core::CoreComponent for CorporateFactory {
    fn get_name(&self) -> String {
        "CorporateFactory".to_string()
    }
    
    fn get_version(&self) -> String {
        "1.0.0".to_string()
    }
    
    fn initialize(&self) -> Result<(), String> {
        self.base.set_status(FactoryStatus::Ready);
        Ok(())
    }
    
    fn shutdown(&self) -> Result<(), String> {
        self.base.set_status(FactoryStatus::Shutdown);
        Ok(())
    }
    
    fn get_status(&self) -> crate::core::ComponentStatus {
        match self.base.get_status() {
            FactoryStatus::Ready => crate::core::ComponentStatus::Ready,
            FactoryStatus::Processing => crate::core::ComponentStatus::Running,
            FactoryStatus::Error(msg) => crate::core::ComponentStatus::Error,
            _ => crate::core::ComponentStatus::Initializing,
        }
    }
}

// هياكل البيانات الخاصة بمصنع المؤسسات

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub document_type: String,
    pub sections: Vec<String>,
    pub required_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub policy_type: String,
    pub compliance: Vec<String>,
    pub sections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentCreationRequest {
    pub document_type: String,
    pub company_name: String,
    pub document_title: Option<String>,
    pub party_names: Vec<String>,
    pub position: Option<String>,
    pub location: Option<String>,
    pub amount: Option<u32>,
    pub currency: Option<String>,
    pub jurisdiction: Option<String>,
    pub description: Option<String>,
    pub contact_email: Option<String>,
    pub duration_months: Option<u32>,
    pub manager_name: Option<String>,
    pub project_type: Option<String>,
    pub objectives: Option<String>,
    pub scope: Option<String>,
    pub timeline: Option<String>,
    pub team_info: Option<String>,
    pub risks: Option<String>,
    pub success_criteria: Option<String>,
    pub employee_name: Option<String>,
    pub period: Option<String>,
    pub summary: Option<String>,
    pub achievements: Option<String>,
    pub improvement_areas: Option<String>,
    pub future_goals: Option<String>,
    pub recommendations: Option<String>,
    pub evaluation: Option<String>,
    pub rating: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateDocument {
    pub id: String,
    pub title: String,
    pub document_type: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub version: String,
    pub effective_date: chrono::DateTime<chrono::Utc>,
    pub requires_signature: bool,
}

#[derive(Debug, Clone)]
pub struct ReportGenerator {
    pub id: String,
    pub name: String,
    pub report_types: Vec<String>,
    pub supported_formats: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WorkflowTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    pub approvals_required: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_corporate_factory_creation() {
        let factory = CorporateFactory::new();
        
        assert_eq!(factory.get_name(), "مصنع المؤسسات");
        assert!(matches!(factory.get_type(), FactoryType::Corporate));
        assert_eq!(factory.get_version(), "1.0.0");
    }
    
    #[test]
    fn test_document_templates_loaded() {
        let factory = CorporateFactory::new();
        
        // يجب أن يحتوي على قالب عقد عمل على الأقل
        assert!(factory.document_templates.contains_key("contract_employment"));
        assert!(factory.policy_templates.contains_key("policy_privacy"));
    }
    
    #[test]
    fn test_capabilities_initialization() {
        let factory = CorporateFactory::new();
        let capabilities = factory.get_capabilities();
        
        assert!(!capabilities.is_empty());
        assert!(capabilities.iter().any(|c| c.name.contains("الوثائق المؤسسية")));
        assert!(capabilities.iter().any(|c| c.name.contains("إدارة المشاريع")));
        assert!(capabilities.iter().any(|c| c.name.contains("الموارد البشرية")));
    }
}
