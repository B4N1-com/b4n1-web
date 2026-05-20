# B4n1Web E2E Test Container — Base Image
# Contains all language toolchains. Binary and SDKs are mounted at runtime.

FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

# ── Base tools ──
RUN apt-get update && apt-get install -y \
    curl ca-certificates wget gnupg unzip git jq \
    && rm -rf /var/lib/apt/lists/*

# ── Python 3 ──
RUN apt-get update && apt-get install -y \
    python3 python3-pip python3-venv \
    && pip3 install --upgrade pip \
    && rm -rf /var/lib/apt/lists/*

# ── Node.js 18 ──
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# ── Go 1.21 ──
RUN wget -q https://go.dev/dl/go1.21.5.linux-amd64.tar.gz \
    && tar -C /usr/local -xzf go1.21.5.linux-amd64.tar.gz \
    && rm go1.21.5.linux-amd64.tar.gz
ENV PATH="/usr/local/go/bin:${PATH}"

# ── .NET 8.0 SDK ──
RUN wget https://packages.microsoft.com/config/ubuntu/22.04/packages-microsoft-prod.deb -O packages-microsoft-prod.deb \
    && dpkg -i packages-microsoft-prod.deb \
    && rm packages-microsoft-prod.deb \
    && apt-get update \
    && apt-get install -y dotnet-sdk-8.0 \
    && rm -rf /var/lib/apt/lists/*

# ── Java 17 + Maven ──
RUN apt-get update && apt-get install -y \
    openjdk-17-jdk-headless maven \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
CMD ["bash"]
