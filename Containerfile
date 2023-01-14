# This Dockerfile is for the Makefile to use, not for builds to Dockerhub
FROM registry.fedoraproject.org/fedora:37
LABEL maintainer="Randy Barlow <randy@electronsweatshop.com>"

RUN dnf upgrade -y
# openssl-devel is needed for cargo-audit
RUN dnf install -y cargo clippy openssl-devel rustfmt
RUN cargo install cargo-audit
# This is useful for finding all the licenses of the bundled libraries
RUN cargo install cargo-license

CMD ["bash"]
