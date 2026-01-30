//! 📚 مصنع التعليم
//! متخصص في إنشاء المحتوى التعليمي والتدريبي الذكي

use crate::core::factory::{
    Factory, FactoryType, FactoryStatus, FactoryCapability, 
    FactoryRequest, FactoryResponse, FactoryOutput, FactoryError,
    CapabilityParameter, ParameterType, RequestPriority, OutputType, OutputFormat
};
use crate::core::quality::{QualityGate, GateType};
use crate::factories::BaseFactory;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// مصنع التعليم
pub struct EducationFactory {
    base: BaseFactory,
    course_templates: HashMap<String, CourseTemplate>,
    lesson_generators: Vec<LessonGenerator>,
    assessment_tools: Vec<AssessmentTool>,
}

impl EducationFactory {
    pub fn new() -> Self {
        let mut factory = Self {
            base: BaseFactory::new(
                "مصنع التعليم".to_string(),
                FactoryType::Education,
                "1.0.0".to_string()
            ),
            course_templates: HashMap::new(),
            lesson_generators: Vec::new(),
            assessment_tools: Vec::new(),
        };
        
        // إضافة الإمكانيات الأساسية
        factory.initialize_capabilities();
        
        // تحميل القوالب الافتراضية
        factory.load_default_templates();
        
        factory
    }
    
    fn initialize_capabilities(&mut self) {
        // إمكانية إنشاء الدورات
        let course_creation = FactoryCapability {
            name: "إنشاء الدورات التعليمية".to_string(),
            description: "إنشاء دورات تعليمية متكاملة مع مواد وتمارين".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
            parameters: HashMap::from([
                ("course_title".to_string(), CapabilityParameter {
                    name: "عنوان الدورة".to_string(),
                    description: "عنوان الدورة التعليمية".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: None,
                }),
                ("course_level".to_string(), CapabilityParameter {
                    name: "مستوى الدورة".to_string(),
                    description: "مستوى الصعوبة (مبتدئ، متوسط، متقدم)".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("مبتدئ")),
                }),
                ("lesson_count".to_string(), CapabilityParameter {
                    name: "عدد الدروس".to_string(),
                    description: "عدد الدروس في الدورة".to_string(),
                    required: true,
                    data_type: ParameterType::Integer,
                    default_value: Some(serde_json::json!(10)),
                }),
            ]),
        };
        
        // إمكانية إنشاء الدروس
        let lesson_creation = FactoryCapability {
            name: "إنشاء الدروس التفصيلية".to_string(),
            description: "إنشاء دروس تعليمية مع شرح مفصل وأمثلة".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
            parameters: HashMap::from([
                ("lesson_topic".to_string(), CapabilityParameter {
                    name: "موضوع الدرس".to_string(),
                    description: "الموضوع الرئيسي للدرس".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: None,
                }),
                ("difficulty".to_string(), CapabilityParameter {
                    name: "الصعوبة".to_string(),
                    description: "مستوى صعوبة الدرس (1-5)".to_string(),
                    required: false,
                    data_type: ParameterType::Integer,
                    default_value: Some(serde_json::json!(3)),
                }),
                ("include_examples".to_string(), CapabilityParameter {
                    name: "تضمين الأمثلة".to_string(),
                    description: "تضمين أمثلة عملية".to_string(),
                    required: false,
                    data_type: ParameterType::Boolean,
                    default_value: Some(serde_json::json!(true)),
                }),
            ]),
        };
        
        // إمكانية إنشاء التمارين
        let exercise_creation = FactoryCapability {
            name: "إنشاء التمارين والاختبارات".to_string(),
            description: "إنشاء تمارين عملية واختبارات تقييمية".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
            parameters: HashMap::from([
                ("exercise_type".to_string(), CapabilityParameter {
                    name: "نوع التمرين".to_string(),
                    description: "نوع التمرين (اختيار من متعدد، صح/خطأ، برمجة)".to_string(),
                    required: true,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("اختيار من متعدد")),
                }),
                ("question_count".to_string(), CapabilityParameter {
                    name: "عدد الأسئلة".to_string(),
                    description: "عدد الأسئلة في التمرين".to_string(),
                    required: true,
                    data_type: ParameterType::Integer,
                    default_value: Some(serde_json::json!(10)),
                }),
                ("difficulty_level".to_string(), CapabilityParameter {
                    name: "مستوى الصعوبة".to_string(),
                    description: "مستوى صعوبة الأسئلة (سهل، متوسط، صعب)".to_string(),
                    required: false,
                    data_type: ParameterType::String,
                    default_value: Some(serde_json::json!("متوسط")),
                }),
            ]),
        };
        
        self.base.add_capability(course_creation);
        self.base.add_capability(lesson_creation);
        self.base.add_capability(exercise_creation);
    }
    
    fn load_default_templates(&mut self) {
        // قالب دورة البرمجة
        let programming_course = CourseTemplate {
            id: "prog_101".to_string(),
            name: "دورة البرمجة للمبتدئين".to_string(),
            description: "دورة شاملة لتعلم أساسيات البرمجة".to_string(),
            category: "برمجة".to_string(),
            difficulty: "مبتدئ".to_string(),
            estimated_hours: 40,
            lessons: vec![
                LessonTemplate {
                    title: "مقدمة في البرمجة".to_string(),
                    duration_minutes: 60,
                    topics: vec!["ما هي البرمجة".to_string(), "لغات البرمجة".to_string()],
                },
                LessonTemplate {
                    title: "المتغيرات وأنواع البيانات".to_string(),
                    duration_minutes: 90,
                    topics: vec!["المتغيرات".to_string(), "أنواع البيانات".to_string()],
                },
            ],
            prerequisites: vec![],
            learning_outcomes: vec![
                "فهم أساسيات البرمجة".to_string(),
                "القدرة على كتابة برامج بسيطة".to_string(),
            ],
        };
        
        // قالب دورة تصميم الويب
        let web_design_course = CourseTemplate {
            id: "web_101".to_string(),
            name: "تصميم الويب الأساسي".to_string(),
            description: "تعلم أساسيات HTML و CSS".to_string(),
            category: "تصميم ويب".to_string(),
            difficulty: "مبتدئ".to_string(),
            estimated_hours: 30,
            lessons: vec![
                LessonTemplate {
                    title: "مقدمة في HTML".to_string(),
                    duration_minutes: 60,
                    topics: vec!["تركيب HTML".to_string(), "العناصر الأساسية".to_string()],
                },
                LessonTemplate {
                    title: "أساسيات CSS".to_string(),
                    duration_minutes: 90,
                    topics: vec!["تنسيق النصوص".to_string(), "الألوان والخلفيات".to_string()],
                },
            ],
            prerequisites: vec![],
            learning_outcomes: vec![
                "بناء صفحات ويب بسيطة".to_string(),
                "تطبيق التنسيقات الأساسية".to_string(),
            ],
        };
        
        self.course_templates.insert(programming_course.id.clone(), programming_course);
        self.course_templates.insert(web_design_course.id.clone(), web_design_course);
    }
    
    fn create_course(&self, request: &CourseCreationRequest) -> Result<Course, FactoryError> {
        log::info!("إنشاء دورة تعليمية: {}", request.title);
        
        // البحث عن قالب مناسب
        let template = self.find_suitable_template(&request.category, &request.difficulty)
            .ok_or_else(|| FactoryError::ProcessingFailed("لم يتم العثور على قالب مناسب".to_string()))?;
        
        // إنشاء الدروس
        let lessons = self.generate_lessons(&template, request.lesson_count);
        
        // إنشاء التمارين
        let exercises = self.generate_exercises(&lessons, request.exercise_per_lesson);
        
        // إنشاء مواد الدورة
        let materials = self.generate_course_materials(&template);
        
        Ok(Course {
            id: format!("course_{}", uuid::Uuid::new_v4()),
            title: request.title.clone(),
            description: request.description.clone(),
            category: request.category.clone(),
            difficulty: request.difficulty.clone(),
            estimated_hours: template.estimated_hours,
            lessons,
            exercises,
            materials,
            created_at: chrono::Utc::now(),
        })
    }
    
    fn find_suitable_template(&self, category: &str, difficulty: &str) -> Option<&CourseTemplate> {
        self.course_templates.values()
            .find(|t| t.category == category && t.difficulty == difficulty)
    }
    
    fn generate_lessons(&self, template: &CourseTemplate, count: usize) -> Vec<Lesson> {
        let mut lessons = Vec::new();
        
        for i in 0..count.min(template.lessons.len()) {
            let lesson_template = &template.lessons[i];
            
            lessons.push(Lesson {
                id: format!("lesson_{}", i + 1),
                title: lesson_template.title.clone(),
                content: self.generate_lesson_content(&lesson_template.title, &lesson_template.topics),
                duration_minutes: lesson_template.duration_minutes,
                order: i as u32,
                has_exercises: true,
            });
        }
        
        lessons
    }
    
    fn generate_lesson_content(&self, title: &str, topics: &[String]) -> String {
        format!(
            "# {}\n\n## الأهداف التعليمية\n- فهم {}\n- تطبيق المبادئ العملية\n\n## المحتوى\n{}\n\n## الخلاصة\nتم تغطية {} بشكل شامل.",
            title,
            topics.join(" و"),
            topics.iter().map(|t| format!("### {}\nشرح مفصل عن {}", t, t)).collect::<Vec<_>>().join("\n\n"),
            title
        )
    }
    
    fn generate_exercises(&self, lessons: &[Lesson], exercises_per_lesson: usize) -> Vec<Exercise> {
        let mut all_exercises = Vec::new();
        
        for (lesson_index, lesson) in lessons.iter().enumerate() {
            for ex_index in 0..exercises_per_lesson {
                all_exercises.push(Exercise {
                    id: format!("ex_{}_{}", lesson_index, ex_index),
                    lesson_id: lesson.id.clone(),
                    question: format!("سؤال {} عن {}", ex_index + 1, lesson.title),
                    options: vec![
                        "الخيار الأول".to_string(),
                        "الخيار الثاني".to_string(),
                        "الخيار الثالث".to_string(),
                        "الخيار الرابع".to_string(),
                    ],
                    correct_answer: 0,
                    difficulty: "متوسط".to_string(),
                    explanation: "شرح الإجابة الصحيحة".to_string(),
                });
            }
        }
        
        all_exercises
    }
    
    fn generate_course_materials(&self, template: &CourseTemplate) -> CourseMaterials {
        CourseMaterials {
            syllabus: format!("مقرر دورة {}", template.name),
            reading_list: vec![
                "الكتاب الأول".to_string(),
                "الكتاب الثاني".to_string(),
            ],
            slides: vec![
                "العرض التقديمي الأول".to_string(),
                "العرض التقديمي الثاني".to_string(),
            ],
            projects: vec![
                "مشروع نهائي".to_string(),
            ],
            certificate_template: "قالب شهادة إتمام الدورة".to_string(),
        }
    }
}

impl Factory for EducationFactory {
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
        log::info!("معالجة طلب في مصنع التعليم: {:?}", request.operation);
        
        match request.operation.as_str() {
            "create_course" => {
                let course_request: CourseCreationRequest = serde_json::from_value(
                    request.parameters.get("course_data")
                        .ok_or_else(|| FactoryError::InvalidRequest("بيانات الدورة مطلوبة".to_string()))?
                        .clone()
                ).map_err(|e| FactoryError::InvalidRequest(format!("بيانات غير صالحة: {}", e)))?;
                
                let course = self.create_course(&course_request)?;
                
                Ok(FactoryResponse {
                    request_id: request.request_id,
                    success: true,
                    output: Some(FactoryOutput {
                        output_type: OutputType::Document,
                        content: serde_json::to_value(&course)
                            .map_err(|e| FactoryError::ProcessingFailed(format!("فشل تسلسل البيانات: {}", e)))?,
                        format: OutputFormat::Json,
                        size_bytes: 0,
                        metadata: HashMap::from([
                            ("course_id".to_string(), course.id.clone()),
                            ("lesson_count".to_string(), course.lessons.len().to_string()),
                        ]),
                        files: Vec::new(),
                    }),
                    error_message: None,
                    processing_time_ms: 100,
                    quality_score: Some(0.95),
                    created_at: request.created_at,
                    completed_at: chrono::Utc::now(),
                })
            }
            
            "create_lesson" => {
                // تنفيذ إنشاء درس
                Err(FactoryError::ProcessingFailed("لم يتم تنفيذ هذه الوظيفة بعد".to_string()))
            }
            
            "create_exercise" => {
                // تنفيذ إنشاء تمرين
                Err(FactoryError::ProcessingFailed("لم يتم تنفيذ هذه الوظيفة بعد".to_string()))
            }
            
            _ => Err(FactoryError::InvalidRequest(format!("عملية غير معروفة: {}", request.operation)))
        }
    }
    
    fn validate_output(&self, output: &FactoryOutput) -> Vec<QualityGate> {
        let mut gates = Vec::new();
        
        // بوابة الجودة التقنية للدورات
        gates.push(QualityGate {
            id: "edu_tech_quality".to_string(),
            name: "جودة المحتوى التعليمي".to_string(),
            description: "فحص جودة المحتوى التعليمي".to_string(),
            gate_type: GateType::Technical,
            criteria: vec![],
            threshold: 0.8,
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

impl crate::core::CoreComponent for EducationFactory {
    fn get_name(&self) -> String {
        "EducationFactory".to_string()
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

// هياكل البيانات الخاصة بمصنع التعليم

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub difficulty: String,
    pub estimated_hours: u32,
    pub lessons: Vec<LessonTemplate>,
    pub prerequisites: Vec<String>,
    pub learning_outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonTemplate {
    pub title: String,
    pub duration_minutes: u32,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCreationRequest {
    pub title: String,
    pub description: String,
    pub category: String,
    pub difficulty: String,
    pub lesson_count: usize,
    pub exercise_per_lesson: usize,
    pub include_certificate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub difficulty: String,
    pub estimated_hours: u32,
    pub lessons: Vec<Lesson>,
    pub exercises: Vec<Exercise>,
    pub materials: CourseMaterials,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub content: String,
    pub duration_minutes: u32,
    pub order: u32,
    pub has_exercises: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub lesson_id: String,
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub difficulty: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseMaterials {
    pub syllabus: String,
    pub reading_list: Vec<String>,
    pub slides: Vec<String>,
    pub projects: Vec<String>,
    pub certificate_template: String,
}

#[derive(Debug, Clone)]
pub struct LessonGenerator {
    pub id: String,
    pub name: String,
    pub supported_topics: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AssessmentTool {
    pub id: String,
    pub name: String,
    pub question_types: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_education_factory_creation() {
        let factory = EducationFactory::new();
        
        assert_eq!(factory.get_name(), "مصنع التعليم");
        assert!(matches!(factory.get_type(), FactoryType::Education));
        assert_eq!(factory.get_version(), "1.0.0");
    }
    
    #[test]
    fn test_course_templates_loaded() {
        let factory = EducationFactory::new();
        
        // يجب أن يحتوي على قالبين افتراضيين على الأقل
        assert!(factory.course_templates.contains_key("prog_101"));
        assert!(factory.course_templates.contains_key("web_101"));
    }
    
    #[test]
    fn test_capabilities_initialization() {
        let factory = EducationFactory::new();
        let capabilities = factory.get_capabilities();
        
        assert!(!capabilities.is_empty());
        assert!(capabilities.iter().any(|c| c.name.contains("الدورات التعليمية")));
        assert!(capabilities.iter().any(|c| c.name.contains("الدروس التفصيلية")));
    }
}
