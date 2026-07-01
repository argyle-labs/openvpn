<p align="center">
  <img src="assets/icon-256.png" width="120" alt="openvpn" />
</p>

# openvpn

OpenVPN is a robust, widely-deployed SSL/TLS VPN server.

A first-party [orca](https://github.com/argyle-labs/orca) plugin (service-backend).

This repo is **self-contained** — the steps below run openvpn **by hand, without orca**. orca automates exactly this (same image, ports, and data) through one generic surface.

---

## Run it without orca

### Docker / Podman

```yaml
# compose.yml
services:
  openvpn:
    image: kylemanna/openvpn:latest
    container_name: openvpn
    restart: unless-stopped
    ports:
      - "1194:1194/udp"   # VPN
    volumes:
      - ./openvpn-data:/etc/openvpn
```

```sh
docker compose up -d
```

Podman: the same file with `podman-compose up -d`.

### Ports & data

| | |
|---|---|
| Default port | `1194` |
| Upstream | <https://openvpn.net/> |


### Backup & restore

Back up the config/data volume(s) above — that's the whole service state (stop the container first for a clean copy). Restore by putting them back and starting it.

> With orca this is **`service.backup` / `service.restore`** — location-agnostic (docker / podman / lxc / vm), one command regardless of where openvpn runs. No per-service backup script.

## With orca

orca drives this plugin through the single generic `service.*` surface — no per-plugin tools:

```sh
orca service.deploy openvpn      # render + launch on any supported runtime
orca service.status openvpn      # health + rich diagnostics (typed payload)
orca service.backup openvpn      # location-agnostic backup (tar; PBS on Proxmox)
orca service.configure openvpn   # apply config via the upstream API
```

## Layout

- `src/` — the plugin (pure Rust): the `ServiceBackend` descriptor + `configure` / `status`.
- [CAPABILITIES.md](CAPABILITIES.md) — the service-backend contract checklist.
- `assets/` — plugin icon.
