import re

with open('src/cli/commands/docs.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# إصلاح خطأ التنسيق
fixed_content = content.replace(
    '{\\\"topic\\\":\\\"الذكاء الاصطناعي\\\"}',
    '{"topic": "الذكاء الاصطناعي"}'
)

with open('src/cli/commands/docs.rs', 'w', encoding='utf-8') as f:
    f.write(fixed_content)

print("✅ تم إصلاح ملف docs.rs")
