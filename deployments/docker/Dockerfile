# Dockerfile لـ Marwan Hub Factories v3.0.0
FROM rust:1.70-alpine as builder

# تثبيت الأدوات المطلوبة
RUN apk add --no-cache musl-dev openssl-dev

# إنشاء دليل العمل
WORKDIR /usr/src/marwan-hub

# نسخ ملفات المشروع
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# بناء المشروع
RUN cargo build --release

# ----------------------------
# المرحلة النهائية
FROM alpine:3.18

# تثبيت المكتبات المطلوبة
RUN apk add --no-cache libgcc openssl ca-certificates

# إنشاء مستخدم جديد
RUN addgroup -g 1000 marwan && \
    adduser -D -u 1000 -G marwan marwan

# نسخ الملفات التنفيذية
COPY --from=builder /usr/src/marwan-hub/target/release/marwan-hub /usr/local/bin/
COPY --chown=marwan:marwan docker-compose.yml ./
COPY --chown=marwan:marwan setup.sh ./

# إنشاء دليل البيانات
RUN mkdir -p /data/marwan-hub && \
    chown -R marwan:marwan /data/marwan-hub

# تبديل المستخدم
USER marwan

# إنشاء مجلدات التكوين
WORKDIR /data/marwan-hub
RUN mkdir -p config logs data templates

# نسخ ملفات التكوين الافتراضية
COPY --chown=marwan:marwan config/default.toml ./config/

# تعريف المنفذ
EXPOSE 8080 8443

# متغيرات البيئة
ENV RUST_LOG=info
ENV MHOS_VERSION=v2.2
ENV DATA_DIR=/data/marwan-hub

# نقطة الدخول
ENTRYPOINT ["marwan-hub"]
CMD ["serve", "--port", "8080"]
