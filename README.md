# DynDNS

This is a simple dynamic DNS (dynDNS) server that allows to update A/AAAA/TXT records for a given domain.

Similar to services like [DuckDNS](https://www.duckdns.org/), but you can host it yourself easilly. Currently, it only supports [Porkbun](https://porkbun.com/)'s backend for DNS updates. Future updates may include support for additional DNS providers.

```
Usage: dyndns [OPTIONS] --porkbun-api-key <PORKBUN_API_KEY> --porkbun-secret-key <PORKBUN_SECRET_KEY> --domain <DOMAIN> --token <TOKEN>

Options:
      --host <HOST>                              Host [default: 0.0.0.0:3000]
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
$ dyndns --porkbun-api-key key --porkbun-secret-key secret --domain dyndns.nxthdr.dev --token example
```

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