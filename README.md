# tailscale-dns-sync

A small tool to sync DNS records from your Tailscale tailnet to Route53.

Will add records in the format `<TAILSCALE HOSTNAME>.<HOSTED ZONE NAME>`

## Usage

This assumes your environment has the requisite AWS permissions set up to modify Route5 hosted zones.

After `cargo install`ing locally, run:

```sh
export TAILSCALE_DNS_API_KEY=<your_api_key>
export TAILSCALE_DNS_TAILNET=<your_tailnet_name>
export TAILSCALE_DNS_ROUTE53_DOMAIN=<your_hosted_zone_name>
tailscale-dns-sync
```

## TODO

- [ ] Redesign to allow for easier addition of other cloud providers
- [ ] Flag for disabling ipv6 records
- [ ] Split up domain name to sync from hosted zone name
