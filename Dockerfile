# -------------------------------------------------------------------------------------------
# Dockerfile
#
# docker build --platform linux/amd64 -t rust .
# docker run -it --rm --platform linux/amd64 -v `pwd`:/rust/ rust
# -------------------------------------------------------------------------------------------

FROM rust:1.58.1
WORKDIR /rust
