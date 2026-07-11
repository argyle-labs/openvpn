# OpenVPN Container Image

Docker container for a commercial VPN provider (e.g. PIA / Private Internet Access) with a Privoxy HTTP proxy and a microsocks SOCKS5 proxy.

This directory contains the Dockerfile and scripts for building the container image. For deploying services that route through this container, see your downloaders/VPN gateway docs.

## Image

Published to a container registry, e.g. `ghcr.io/<owner>/openvpn`.

The image is automatically built and pushed by CI when the `compose/openvpn/` sources change.

## Container Configuration

| Variable         | Default       | Description                 |
| ---------------- | ------------- | --------------------------- |
| `PIA_USERNAME`   | *(required)*  | VPN account username        |
| `PIA_PASSWORD`   | *(required)*  | VPN account password        |
| `REGION`         | `ca-montreal` | VPN region                  |
| `PROXY_PORT`     | `8888`        | Privoxy HTTP proxy port     |
| `SOCKS_PORT`     | `1080`        | microsocks SOCKS5 port      |
| `KILLSWITCH`     | `true`        | Enable iptables kill switch |
| `UPDATE_CONFIGS` | `true`        | Update provider configs on boot |
| `DEBUG`          | `false`       | Enable debug logging        |

> Store the VPN username/password as secrets — never commit them.

### Required Capabilities

- `NET_ADMIN` and `NET_RAW`
- Access to `/dev/net/tun`
- Sysctls: `net.ipv4.conf.all.src_valid_mark=1`, `net.ipv6.conf.all.disable_ipv6=1`

## Directory Structure

```
compose/openvpn/
├── Dockerfile          # Container image definition
├── README.md           # Stub pointing here
└── scripts/
    ├── entrypoint.sh   # Container entrypoint
    ├── check-proxy.sh  # Proxy health check
    ├── test-vpn.sh     # VPN connection test
    └── tail-logs.sh    # Log viewing helper
```

## Building Locally

```bash
cd compose/openvpn
docker build -t openvpn .

# Multi-platform build
docker buildx build --platform linux/amd64,linux/arm64 -t ghcr.io/<owner>/openvpn:latest --push .
```

## Troubleshooting

### Provider Config Download Timeout

The Dockerfile downloads VPN configs during build. If the provider's servers are slow, the build may fail. Retry later.

### Container Issues

See your downloaders/VPN gateway docs for troubleshooting the running container.
