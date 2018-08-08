getwms -u "https://webmention.io/api/mentions?domain=https://yourdomain.com&token=YOURAPIKEY"
hugo
cd public
git commit -a -m "Update"
git push