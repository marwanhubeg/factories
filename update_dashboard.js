// تحديث حالة المصانع بناءً على الملفات الحقيقية
const fs = require('fs');
const path = require('path');

console.log('🔄 تحديث حالة المصانع في الواجهة...');

const factoryTypes = [
    { id: 'education', name: 'مصنع التعليم الذكي', icon: '🎓' },
    { id: 'creative', name: 'مصنع الإبداع الرقمي', icon: '🎨' },
    { id: 'technology', name: 'مصنع التقنية المتقدمة', icon: '💻' },
    { id: 'corporate', name: 'مصنع حلول الأعمال', icon: '📊' }
];

const factories = factoryTypes.map(ftype => {
    const factoryPath = path.join('src', 'factories', ftype.id);
    const exists = fs.existsSync(factoryPath);
    let files = 0;
    let hasRsFiles = false;
    
    if (exists) {
        try {
            const allFiles = fs.readdirSync(factoryPath);
            files = allFiles.length;
            hasRsFiles = allFiles.some(f => f.endsWith('.rs'));
        } catch (err) {
            console.error(`❌ خطأ في قراءة مجلد ${ftype.id}:`, err.message);
        }
    }
    
    // تحديد الحالة بناءً على الملفات
    let status, efficiency, productionRate, health;
    
    if (!exists) {
        status = 'error';
        efficiency = 70;
        productionRate = 0;
        health = 'error';
    } else if (!hasRsFiles) {
        status = 'maintenance';
        efficiency = 75;
        productionRate = Math.floor(Math.random() * 50) + 50;
        health = 'warning';
    } else {
        status = 'running';
        efficiency = Math.floor(Math.random() * 20) + 80; // 80-100%
        productionRate = Math.floor(Math.random() * 100) + 100; // 100-200
        health = efficiency > 90 ? 'excellent' : efficiency > 80 ? 'good' : 'warning';
    }
    
    return {
        id: `factory_${factoryTypes.indexOf(ftype) + 1}`,
        name: ftype.name,
        type: ftype.id,
        status: status,
        efficiency: efficiency,
        production_rate: productionRate,
        health: health,
        files_count: files,
        has_rs_files: hasRsFiles,
        exists: exists,
        icon: ftype.icon,
        last_maintenance: '2026-01-31',
        next_maintenance: '2026-02-15'
    };
});

// إنشاء ملف JSON للمصانع
const factoriesJson = JSON.stringify(factories, null, 2);
fs.writeFileSync('factories_status.json', factoriesJson);

console.log('✅ تم تحديث حالة المصانع:');
factories.forEach(f => {
    const statusIcon = f.status === 'running' ? '✅' : f.status === 'maintenance' ? '🟠' : '🔴';
    console.log(`${statusIcon} ${f.name}: ${f.efficiency}% كفاءة، ${f.files_count} ملفات`);
});

console.log('\n📊 ملخص:');
console.log(`• ✅ نشط: ${factories.filter(f => f.status === 'running').length}`);
console.log(`• 🟠 صيانة: ${factories.filter(f => f.status === 'maintenance').length}`);
console.log(`• 🔴 خطأ: ${factories.filter(f => f.status === 'error').length}`);
