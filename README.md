# DynDNS

> [!WARNING]
> Peers is currently in early-stage development.

This is a simple dynamic DNS server that allows you to update A/AAAA/TXT records for a given domain.

Similar to services like [DuckDNS](https://www.duckdns.org/), but you can host it yourself easilly. Currently, it only supports [Porkbun](https://porkbun.com/)'s backend for DNS updates. Future updates may include support for additional DNS providers.

```
Usage: dyndns [OPTIONS] --porkbun-api-key <PORKBUN_API_KEY> --porkbun-secret-key <PORKBUN_SECRET_KEY> --domain <DOMAIN> --token <TOKEN>

Options:
      --porkbun-api-key <PORKBUN_API_KEY>        Porkbun API key
      --porkbun-secret-key <PORKBUN_SECRET_KEY>  Porkbun secret key
      --domain <DOMAIN>                          Domain
  -v, --verbose...                               Increase logging verbosity
  -q, --quiet...                                 Decrease logging verbosity
      --token <TOKEN>                            Authentication token
  -h, --help                                     Print help
  -V, --version                                  Print version
```

## Usage

The spec is very similar to the one used by [DuckDNS](https://www.duckdns.org/spec.jsp).
Using the [nxthdr](https://nxthdr.dev)'s endpoint `https://dyndns.nxthdr.dev`:

```
https://dyndns.nxthdr.dev/update[?subdomain=<DOMAIN>]&token=<TOKEN>[&a=<IPV4>][&aaaa=<IPV6>][&txt=<TXT>][&clear=true]
```

- `subdomain`: The subdomain to update. If not provided, a random subdomain will be generated.
- `token`: The authentication token.
- `a`: an optional IPv4 address to update.
- `aaaa`: an optional IPv6 address to update.
- `txt`: an optional TXT record to update.
- `clear`: an optional flag to clear the provided record(s).

The server will return the updated records in JSON format.

## Example

```sh
$ curl -6s 'https://dyndns.nxthdr.dev?subdomain=test&token=<REDACTED>&a=1.1.1.1' |jq
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