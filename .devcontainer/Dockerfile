FROM mcr.microsoft.com/devcontainers/rust:latest

ENV HOME=/home/vscode

USER vscode
WORKDIR /workspace

RUN curl -sS https://starship.rs/install.sh | sh -s -- -y && \
    echo 'eval "$(starship init bash)"' >> ~/.bashrc
