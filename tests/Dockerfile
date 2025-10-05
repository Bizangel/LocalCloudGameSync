FROM debian:bookworm-slim

ENV DEBIAN_FRONTEND=noninteractive

# Install restic and OpenSSH server
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    restic \
    openssh-server \
    && rm -rf /var/lib/apt/lists/*

# Create SSH runtime directory
RUN mkdir /var/run/sshd

# Create test user with no password
RUN useradd -m -s /bin/bash testuser && \
    passwd -d testuser

# Allow empty passwords for SSH (insecure, but fine for testing)
RUN sed -i 's/#PermitEmptyPasswords no/PermitEmptyPasswords yes/' /etc/ssh/sshd_config && \
    sed -i 's/#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config

# Expose SSH port
EXPOSE 22

# Start sshd in foreground
CMD ["/usr/sbin/sshd", "-D"]
