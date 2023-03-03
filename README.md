# tailscale-dns-sync

A small tool to sync DNS records from your Tailscale tailnet to Route53, if MagicDNS doesn't take your fancy.

Will add records in the format `<device_hostname>.<target_domain>`

## Usage

This assumes your environment has the requisite AWS permissions set up to modify Route53 hosted zones.

After `cargo install`ing locally, run:

```sh
export TAILSCALE_DNS_API_KEY=<your_api_key>
export TAILSCALE_DNS_TAILNET=<your_tailnet_name>
export TAILSCALE_DNS_ROUTE53_DOMAIN=<your_target_domain>
export TAILSCALE_DNS_ROUTE53_HOSTED_ZONE_ID=<your_hosted_zone_id>
tailscale-dns-sync
```

## TODO

- [ ] Flag for disabling ipv6 records
