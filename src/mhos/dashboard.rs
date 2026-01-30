use std::sync::Arc;
use serde::Serialize;
use std::collections::HashMap;
use crate::core::factory_manager::FactoryManager;

/// لوحة تحكم MH-OS
pub struct Dashboard {
    factory_manager: Arc<FactoryManager>,
    widgets: HashMap<String, DashboardWidget>,
    layout: DashboardLayout,
}

/// عنصر في لوحة التحكم
#[derive(Debug, Clone, Serialize)]
pub struct DashboardWidget {
    pub id: String,
    pub widget_type: WidgetType,
    pub title: String,
    pub data: serde_json::Value,
    pub position: (u32, u32),
    pub size: (u32, u32),
    pub refresh_interval: u64,
}

/// نوع العنصر
#[derive(Debug, Clone, Serialize)]
pub enum WidgetType {
    PerformanceChart,
    QualityGauge,
    FactoryStatus,
    ProductionStats,
    SystemHealth,
    RecentActivity,
    Alerts,
    Recommendations,
}

/// تخطيط لوحة التحكم
#[derive(Debug, Clone, Serialize)]
pub struct DashboardLayout {
    pub columns: u32,
    pub rows: u32,
    pub widgets: Vec<WidgetPosition>,
}

/// موقع العنصر
#[derive(Debug, Clone, Serialize)]
pub struct WidgetPosition {
    pub widget_id: String,
    pub col: u32,
    pub row: u32,
    pub width: u32,
    pub height: u32,
}

/// بيانات لوحة التحكم
#[derive(Debug, Clone, Serialize)]
pub struct DashboardData {
    pub widgets: Vec<DashboardWidget>,
    pub layout: DashboardLayout,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub system_status: SystemStatus,
}

/// حالة النظام
#[derive(Debug, Clone, Serialize)]
pub struct SystemStatus {
    pub overall_health: String,
    pub performance_score: f32,
    pub quality_score: f32,
    pub active_factories: usize,
    pub total_production: u64,
    pub efficiency: f32,
}

impl Dashboard {
    /// إنشاء لوحة تحكم جديدة
    pub fn new(factory_manager: Arc<FactoryManager>) -> Self {
        let mut dashboard = Self {
            factory_manager,
            widgets: HashMap::new(),
            layout: DashboardLayout {
                columns: 12,
                rows: 8,
                widgets: Vec::new(),
            },
        };
        
        // تهيئة العناصر الافتراضية
        dashboard.initialize_default_widgets();
        
        dashboard
    }
    
    /// تهيئة العناصر الافتراضية
    fn initialize_default_widgets(&mut self) {
        // عنصر أداء النظام
        let performance_widget = DashboardWidget {
            id: "performance_chart".to_string(),
            widget_type: WidgetType::PerformanceChart,
            title: "أداء النظام".to_string(),
            data: serde_json::json!({
                "cpu_usage": 35,
                "memory_usage": 45,
                "disk_usage": 25,
                "network_usage": 15,
                "trend": "stable"
            }),
            position: (0, 0),
            size: (6, 4),
            refresh_interval: 30,
        };
        
        self.add_widget(performance_widget);
        
        // عنصر جودة المصانع
        let quality_widget = DashboardWidget {
            id: "quality_gauge".to_string(),
            widget_type: WidgetType::QualityGauge,
            title: "جودة المصانع".to_string(),
            data: serde_json::json!({
                "overall_quality": 0.92,
                "by_factory": {
                    "education": 0.95,
                    "creative": 0.88,
                    "corporate": 0.90,
                    "technology": 0.93
                },
                "trend": "improving"
            }),
            position: (6, 0),
            size: (6, 4),
            refresh_interval: 60,
        };
        
        self.add_widget(quality_widget);
        
        // عنصر حالة المصانع
        let status_widget = DashboardWidget {
            id: "factory_status".to_string(),
            widget_type: WidgetType::FactoryStatus,
            title: "حالة المصانع".to_string(),
            data: serde_json::json!({
                "total": 4,
                "active": 4,
                "idle": 0,
                "error": 0,
                "details": [
                    {"name": "مصنع التعليم", "status": "active", "production": 450},
                    {"name": "المصنع الإبداعي", "status": "active", "production": 320},
                    {"name": "مصنع الشركات", "status": "active", "production": 280},
                    {"name": "مصنع التكنولوجيا", "status": "active", "production": 200}
                ]
            }),
            position: (0, 4),
            size: (4, 4),
            refresh_interval: 15,
        };
        
        self.add_widget(status_widget);
        
        // عنصر إحصائيات الإنتاج
        let production_widget = DashboardWidget {
            id: "production_stats".to_string(),
            widget_type: WidgetType::ProductionStats,
            title: "إحصائيات الإنتاج".to_string(),
            data: serde_json::json!({
                "today": 125,
                "this_week": 850,
                "this_month": 3250,
                "total": 1250,
                "top_products": ["دورة تعليمية", "تصميم موقع", "وثيقة شركة", "كود برمجي"]
            }),
            position: (4, 4),
            size: (4, 4),
            refresh_interval: 300,
        };
        
        self.add_widget(production_widget);
        
        // عنصر النشاط الأخير
        let activity_widget = DashboardWidget {
            id: "recent_activity".to_string(),
            widget_type: WidgetType::RecentActivity,
            title: "النشاط الأخير".to_string(),
            data: serde_json::json!({
                "activities": [
                    {"time": "10:30", "action": "تم إنشاء دورة تعليمية جديدة", "user": "النظام"},
                    {"time": "10:15", "action": "تم تحسين جودة المصنع الإبداعي", "user": "MH-OS"},
                    {"time": "09:45", "action": "تم توليد موقع إلكتروني", "user": "مستخدم"},
                    {"time": "09:30", "action": "تم فحص بوابة الجودة", "user": "النظام"},
                    {"time": "09:00", "action": "بدء تشغيل النظام", "user": "النظام"}
                ]
            }),
            position: (8, 4),
            size: (4, 4),
            refresh_interval: 10,
        };
        
        self.add_widget(activity_widget);
    }
    
    /// إضافة عنصر جديد
    pub fn add_widget(&mut self, widget: DashboardWidget) {
        self.widgets.insert(widget.id.clone(), widget.clone());
        
        self.layout.widgets.push(WidgetPosition {
            widget_id: widget.id,
            col: widget.position.0,
            row: widget.position.1,
            width: widget.size.0,
            height: widget.size.1,
        });
    }
    
    /// تحديث بيانات العنصر
    pub async fn update_widget_data(&mut self, widget_id: &str) {
        if let Some(widget) = self.widgets.get_mut(widget_id) {
            match widget.widget_type {
                WidgetType::FactoryStatus => {
                    widget.data = self.get_factory_status_data().await;
                }
                WidgetType::ProductionStats => {
                    widget.data = self.get_production_stats_data().await;
                }
                WidgetType::PerformanceChart => {
                    widget.data = self.get_performance_data().await;
                }
                WidgetType::QualityGauge => {
                    widget.data = self.get_quality_data().await;
                }
                _ => {
                    // لا تحتاج بيانات حية
                }
            }
            
            log::debug!("تم تحديث عنصر: {}", widget_id);
        }
    }
    
    /// الحصول على بيانات حالة المصانع
    async fn get_factory_status_data(&self) -> serde_json::Value {
        let factories = self.factory_manager.list_factories();
        let mut details = Vec::new();
        
        for factory in factories {
            details.push(serde_json::json!({
                "name": factory.name(),
                "type": factory.factory_type(),
                "status": factory.status(),
                "production": factory.production_count(),
                "quality": factory.quality_score()
            }));
        }
        
        serde_json::json!({
            "total": details.len(),
            "active": details.iter().filter(|d| d["status"] == "active").count(),
            "idle": details.iter().filter(|d| d["status"] == "idle").count(),
            "error": details.iter().filter(|d| d["status"] == "error").count(),
            "details": details
        })
    }
    
    /// الحصول على بيانات الإنتاج
    async fn get_production_stats_data(&self) -> serde_json::Value {
        let factories = self.factory_manager.list_factories();
        let total_production: u64 = factories.iter()
            .map(|f| f.production_count())
            .sum();
        
        serde_json::json!({
            "total": total_production,
            "today": (total_production as f32 * 0.1) as u64, // محاكاة
            "this_week": (total_production as f32 * 0.7) as u64,
            "this_month": total_production,
            "average_daily": total_production / 30,
            "growth_rate": 0.15
        })
    }
    
    /// الحصول على بيانات الأداء
    async fn get_performance_data(&self) -> serde_json::Value {
        // محاكاة بيانات الأداء
        serde_json::json!({
            "cpu_usage": rand::random::<u8>() % 30 + 20,
            "memory_usage": rand::random::<u8>() % 30 + 30,
            "disk_usage": rand::random::<u8>() % 20 + 10,
            "network_usage": rand::random::<u8>() % 10 + 5,
            "response_time": rand::random::<u16>() % 50 + 50,
            "throughput": rand::random::<u16>() % 100 + 100,
            "trend": if rand::random::<bool>() { "improving" } else { "stable" }
        })
    }
    
    /// الحصول على بيانات الجودة
    async fn get_quality_data(&self) -> serde_json::Value {
        let factories = self.factory_manager.list_factories();
        let mut by_factory = serde_json::Map::new();
        
        for factory in factories {
            by_factory.insert(
                factory.factory_type().to_string(),
                serde_json::Value::from(factory.quality_score() as f64)
            );
        }
        
        let quality_scores: Vec<f32> = factories.iter()
            .map(|f| f.quality_score())
            .collect();
        
        let overall_quality = if !quality_scores.is_empty() {
            quality_scores.iter().sum::<f32>() / quality_scores.len() as f32
        } else { 0.0 };
        
        serde_json::json!({
            "overall_quality": overall_quality,
            "by_factory": by_factory,
            "threshold": 0.85,
            "status": if overall_quality > 0.9 { "excellent" } else if overall_quality > 0.8 { "good" } else { "needs_improvement" }
        })
    }
    
    /// الحصول على بيانات لوحة التحكم الكاملة
    pub async fn get_dashboard_data(&mut self) -> DashboardData {
        // تحديث جميع العناصر
        let widget_ids: Vec<String> = self.widgets.keys().cloned().collect();
        for widget_id in widget_ids {
            self.update_widget_data(&widget_id).await;
        }
        
        let factories = self.factory_manager.list_factories();
        let quality_scores: Vec<f32> = factories.iter()
            .map(|f| f.quality_score())
            .collect();
        
        let overall_quality = if !quality_scores.is_empty() {
            quality_scores.iter().sum::<f32>() / quality_scores.len() as f32
        } else { 0.0 };
        
        let total_production: u64 = factories.iter()
            .map(|f| f.production_count())
            .sum();
        
        DashboardData {
            widgets: self.widgets.values().cloned().collect(),
            layout: self.layout.clone(),
            last_updated: chrono::Utc::now(),
            system_status: SystemStatus {
                overall_health: if overall_quality > 0.9 { "excellent".to_string() } else { "good".to_string() },
                performance_score: 0.88,
                quality_score: overall_quality,
                active_factories: factories.len(),
                total_production,
                efficiency: 0.85,
            },
        }
    }
    
    /// إعادة تعيين تخطيط لوحة التحكم
    pub fn reset_layout(&mut self) {
        self.layout.widgets.clear();
        self.widgets.clear();
        self.initialize_default_widgets();
    }
    
    /// تصدير لوحة التحكم
    pub fn export_dashboard(&self) -> serde_json::Value {
        serde_json::json!({
            "version": "1.0",
            "widgets": self.widgets.values().collect::<Vec<_>>(),
            "layout": &self.layout,
            "exported_at": chrono::Utc::now().to_rfc3339()
        })
    }
}
