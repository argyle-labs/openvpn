# OpenVPN — operator notes

OpenVPN is a robust, widely-deployed SSL/TLS VPN **server**. This repo is a
first-party [orca](https://github.com/argyle-labs/orca) plugin
(service-backend) that deploys the upstream `kylemanna/openvpn` server image and
drives it through orca's single generic `service.*` surface.

For deploying services that route through a VPN, see your
download-client / VPN gateway docs.

## Image

Deploys the community `kylemanna/openvpn:latest` server image. This repo does
**not** build a container image of its own — the plugin is pure Rust (see
`src/`) and renders a workload that runs the upstream image on any supported
runtime.

## Ports & data

| | |
| --- | --- |
| Default port | `1194/udp` |
| Config/data volume | `/etc/openvpn` |
| Upstream | <https://openvpn.net/> |

The server listens on `1194/udp`. All service state — the OpenVPN config, PKI,
and issued client certs — lives under the `/etc/openvpn` volume.

## Required capabilities

- `NET_ADMIN`
- Access to `/dev/net/tun`

## Run it without orca

The [README](../README.md) has the by-hand recipes (Docker Compose, Podman,
LXC, VM, Unraid). In short:

```yaml
# compose.yml
services:
  openvpn:
    image: kylemanna/openvpn:latest
    container_name: openvpn
    restart: unless-stopped
    ports:
      - "1194:1194/udp"
    volumes:
      - ./openvpn-data:/etc/openvpn
```

```sh
docker compose up -d
```

## With orca

orca drives this plugin through the generic `service.*` surface — no per-plugin
tools:

```sh
orca service.deploy openvpn      # render + launch on any supported runtime
orca service.status openvpn      # health + rich diagnostics (typed payload)
orca service.backup openvpn      # location-agnostic backup (tar; PBS on Proxmox)
orca service.configure openvpn   # apply config via the upstream API
```

## Backup & restore

Back up the `/etc/openvpn` config/data volume — that is the whole service state
(stop the container first for a clean copy). Restore by putting it back and
starting the container. With orca this is `service.backup` / `service.restore`,
location-agnostic across docker / podman / lxc / vm.

## Troubleshooting

### Container will not start

Confirm the runtime grants `NET_ADMIN` and access to `/dev/net/tun`; the server
cannot create its `tun` interface without them.

### Clients cannot connect

Check that `1194/udp` is published and reachable through any host firewall or
port-forward. For services meant to route through the VPN, see your
download-client / VPN gateway docs.
