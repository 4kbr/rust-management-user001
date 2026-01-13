# stage 1
# Build the Rust application
FROM rust:1.91 as builder
WORKDIR /app
COPY . .
RUN cargo build --release
# kita membiarkan docker yang membuat binary / build project, jadi projectnya tidak perlu di build secara manual

# stage 2
# Create a minimal image to run the application
FROM debian:bookworm-slim
WORKDIR /app
# COPY --from=builder /app/target/release/my_rust_app .
# ganti my_rust_app dengan nama binary hasil build project Rust Anda
COPY --from=builder /app/target/release/rust-user-management .
EXPOSE 8000
# CMD ["./my_rust_app"]
CMD ["./rust-user-management"]
# perintah ini akan menjalankan aplikasi ketika container dijalankan