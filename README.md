# tailscale-dns-sync

A small tool to sync DNS records from your Tailscale tailnet to Route53.

Will add records in the format "<TAILSCALE HOSTNAME>.<HOSTED ZONE NAME>"

## Usage

After `cargo install`ing locally, run:

```sh
export TAILSCALE_DNS_API_KEY=<your_api_key>
export TAILSCALE_DNS_TAILNET=<your_tailnet_name>
export TAILSCALE_DNS_ROUTE53_DOMAIN=<your_hosted_zone_name>
tailscale-dns-sync
```
