# DynDNS

This is a simple dynamic DNS (dynDNS) server that allows to update A/AAAA/TXT records for a given domain.

Similar to services like [DuckDNS](https://www.duckdns.org/), but you can host it yourself easilly. Currently, it only supports [Porkbun](https://porkbun.com/)'s backend for DNS updates. Future updates may include support for additional DNS providers.

```
Usage: dyndns [OPTIONS] --porkbun-api-key <PORKBUN_API_KEY> --porkbun-secret-key <PORKBUN_SECRET_KEY> --domain <DOMAIN>

Options:
      --host <HOST>                                  Host [default: 0.0.0.0:3000]
      --porkbun-api-key <PORKBUN_API_KEY>            Porkbun API key
      --porkbun-secret-key <PORKBUN_SECRET_KEY>      Porkbun secret key
      --domain <DOMAIN>                              Domain
      --token <TOKEN>                                Authentication token
      --client-ip-source <CLIENT_IP_SOURCE>          How to determine the client IP [default: ConnectInfo]
  -v, --verbose...                                   Increase logging verbosity
  -q, --quiet...                                     Decrease logging verbosity
  -h, --help                                         Print help
  -V, --version                                      Print version
```

### Client IP source

When no `a` or `aaaa` parameter is provided, the server falls back to the client's IP. How that IP is determined is controlled by `--client-ip-source`:

- `ConnectInfo` (default) — the socket peer address. Use this when clients connect directly to `dyndns`.
- `RightmostXForwardedFor` — rightmost IP in the `X-Forwarded-For` header. Use this behind a trusted reverse proxy that appends to `X-Forwarded-For`.
- `XRealIp` — the `X-Real-IP` header (commonly set by nginx).
- `RightmostForwarded`, `CfConnectingIp`, `FlyClientIp`, `TrueClientIp`, `CloudFrontViewerAddress` — for other proxy setups.

Only the rightmost / proxy-set variants are trustworthy when a reverse proxy is in front; do not use `X-Forwarded-For`-derived sources when `dyndns` is exposed directly to clients.

## Usage

The spec is very similar to the one used by [DuckDNS](https://www.duckdns.org/spec.jsp).
Using the [nxthdr](https://nxthdr.dev)'s endpoint `https://dyndns.nxthdr.dev`:

```
https://dyndns.nxthdr.dev?token=<TOKEN>[&subdomain=<DOMAIN>][&a=<IPV4>][&aaaa=<IPV6>][&txt=<TXT>][&clear=true]
```

- `subdomain`: the user's subdomain to update. If not provided, a random subdomain will be generated.
- `token`: the authentication token.
- `a`: an optional IPv4 address to update.
- `aaaa`: an optional IPv6 address to update.
- `txt`: an optional TXT record to update.
- `clear`: an optional flag to clear the provided record(s).

The server will return the updated records in JSON format.

## Examples

On those examples, the server has been started with the following command:

```sh
$ dyndns --porkbun-api-key key --porkbun-secret-key secret --domain dyndns.nxthdr.dev --token example --client-ip-source RightmostXForwardedFor
```

(`--client-ip-source RightmostXForwardedFor` is used here because `dyndns.nxthdr.dev` runs behind a reverse proxy. Omit it — or set `ConnectInfo` — if clients connect to `dyndns` directly.)

* Update the `A` record for the user subdomain `test` with the IP `1.1.1.1`

```sh
$ curl -6s 'https://dyndns.nxthdr.dev?token=example&subdomain=test&a=1.1.1.1' |jq
{
  "message": "OK",
  "domain": "test.dyndns.nxthdr.dev",
  "clear": false,
  "records": [
    {
      "type": "A",
      "content": "1.1.1.1"
    }
  ]
}
```

* Update subdomain `test` with the user's IPv6 address `2606:4700:4700::1111`

```sh
$ curl -6s 'https://dyndns.nxthdr.dev?token=example&subdomain=test' |jq
{
  "message": "OK",
  "domain": "test.dyndns.nxthdr.dev",
  "clear": false,
  "records": [
    {
      "type": "AAAA",
      "content": "2606:4700:4700::1111"
    }
  ]
}
```

* Clear the `A` record for the user subdomain `test`

```sh
$ curl -6s 'https://dyndns.nxthdr.dev?token=example&subdomain=test&a=&clear=true' |jq
{
  "message": "OK",
  "domain": "test.dyndns.nxthdr.dev",
  "clear": true,
  "records": [
    {
      "type": "A",
      "content": ""
    }
  ]
}
```

* Update the `TXT` record for the user subdomain `test` with the content `test`. In this case, the user's IPv6 address is also updated.
```sh
$ curl -6s 'https://dyndns.nxthdr.dev?token=example&subdomain=test&txt=test' |jq
{
  "message": "OK",
  "domain": "test.dyndns.nxthdr.dev",
  "clear": false,
  "records": [
    {
      "type": "TXT",
      "content": "test"
    },
    {
      "type": "AAAA",
      "content": "2606:4700:4700::1111"
    }
  ]
}
```
