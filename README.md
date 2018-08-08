# getwms

This tool allows you to query a webmention backend, get the webmentions and store them in JSON files ready to be used by website generators.

## Usage

**webmention.io:**
```
getwms -u https://webmention.io/api/mentions?domain=yourdomain.com&token=YOURAPIKEY
```

## TODO list

- [ ] Store last query time
- [ ] Query only new webmentions
- [ ] Build incrementally the JSON files
