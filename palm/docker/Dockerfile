FROM ubuntu:24.04

ARG USERNAME=palm
ARG DEBIAN_FRONTEND=noninteractive
ARG TARGETARCH

# Set mirror for Ubuntu
# RUN if echo "$TARGETARCH" | grep -q "arm"; then \
#     sed -i.bak 's|http://ports.ubuntu.com/ubuntu-ports|<xxxxxxxx>|g' /etc/apt/sources.list.d/ubuntu.sources; \
#     else \
#     sed -i.bak 's|http://archive.ubuntu.com/ubuntu|<xxxxxxxx>|g' /etc/apt/sources.list.d/ubuntu.sources; \
#     fi

RUN apt update -y && apt upgrade -y \
    && apt install -y ca-certificates \
    # Install packages
    && apt install -y build-essential git curl wget vim \
    sudo \
    zsh zsh-autosuggestions zsh-syntax-highlighting \
    && rm -rf /var/lib/apt/lists/* \
    # Set up user
    && userdel -r ubuntu \
    && useradd -m -s /usr/bin/zsh -G sudo "$USERNAME" \
    && echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/$USERNAME && \
    chmod 0440 /etc/sudoers.d/$USERNAME

USER $USERNAME
WORKDIR /home/$USERNAME

RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" \
    && sed -i 's/^ZSH_THEME=.*/ZSH_THEME="ys"/' ~/.zshrc \
    && echo 'source /usr/share/zsh-autosuggestions/zsh-autosuggestions.zsh' >> ~/.zshrc \
    && echo 'source /usr/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh' >> ~/.zshrc \
    # Install rustup
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . "$HOME/.cargo/env" \
    # Set mirror for crates.io
    # ...
    && rustup install nightly-2024-07-21 \
    && rustup component add --toolchain nightly-2024-07-21 rust-src rustc-dev llvm-tools-preview \
    && cargo +stable install cargo-llvm-cov --locked

CMD ["/usr/bin/zsh"]
