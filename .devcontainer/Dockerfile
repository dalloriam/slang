FROM docker.pkg.github.com/dalloriam/dotfiles/devenv-base:latest

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=noninteractive

# Configure apt and install packages
RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils 2>&1 \
    #
    # Verifty git, process tools, lsb-release (common in install instructions for CLIs) installed
    && apt-get -y install git iproute2 procps lsb-release \
    && apt-get -y install curl pkg-config zip g++ zlib1g-dev unzip python \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    #
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Install Bazel
RUN curl -LO "https://github.com/bazelbuild/bazelisk/releases/download/v1.4.0/bazelisk-linux-amd64" && \
    mv bazelisk-linux-amd64 "/usr/local/bin/bazel" && \
    chmod +x "/usr/local/bin/bazel" && \
    bazel

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=dialog
