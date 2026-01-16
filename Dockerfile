# stage 1
# Build the Rust application
FROM rust:1.91 as builder
WORKDIR /app
COPY . .
RUN cargo build --release
# kita membiarkan docker yang membuat binary / build project, jadi projectnya tidak perlu di build secara manual

# stage 2
# Create a minimal image to run the application
# line ini menggunakan image debian slim sebagai base image untuk menjalankan aplikasi
FROM debian:bookworm-slim
# menambahkan library yang dibutuhkan oleh aplikasi Rust di /app dalam container
WORKDIR /app
# ganti my_rust_app dengan nama binary hasil build project Rust Anda
# COPY --from=builder /app/target/release/my_rust_app .
# ganti rust-user-management dengan nama binary hasil build project Rust Anda
# menyalin binary hasil build dari stage builder ke stage final
COPY --from=builder /app/target/release/rust-user-management .
# mengekspose port 8000 untuk aplikasi
EXPOSE 8000
# menjalankan aplikasi, karena kebetulan hasil rust adalah bin, jadi cara menjalankannya cukup dengan ./nama_binary
CMD ["./rust-user-management"]
# perintah ini akan menjalankan aplikasi ketika container dijalankan