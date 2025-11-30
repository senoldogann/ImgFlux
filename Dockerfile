# --- 1. AŞAMA: BUILDER (İnşaat Sahası) ---
FROM rust:1.81 as builder

# Yeni bir proje oluştur
WORKDIR /usr/src/imgflux
COPY . .

# Release modunda derle (Maksimum optimizasyon)
RUN cargo build --release

# --- 2. AŞAMA: RUNNER (Vitrin) ---
# Debian'ın çok hafif bir sürümünü kullanıyoruz (Bookworm Slim)
# Alpine kullanmıyoruz çünkü Rust ile bazen performans/uyumluluk sorunu çıkarabilir.
FROM debian:bookworm-slim

# SSL sertifikaları gibi temel araçları yükle (Gerekirse)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Binary dosyasını Builder aşamasından kopyala
COPY --from=builder /usr/src/imgflux/target/release/imgflux /usr/local/bin/imgflux

# Çalışma dizini
WORKDIR /usr/local/bin

# 3000 portunu dışarı aç
EXPOSE 3000

# Uygulamayı başlat
CMD ["imgflux"]
