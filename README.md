GOAL:
Website that generates an RSS feed containing updates for your specified manga. Updates feed 8 times a day.
Every manga on Mangaupdates should return an entry on the RSS feed, at least to the current entry in Mangaupdates.

Pulls in data from several (arbitrary?) sources:

  https://www.mangaupdates.com/releases.html (Highest priority, web scrape necessary)
  https://www.tokyotosho.info/rss_customize.php  (Decent users, RSS feed easily customizable)
  https://nyaa.si/ (No real API, unfortunately most popular)
  https://nyaa.pantsu.cat/apidoc/ (Low users but easy to integrate)


1. Scrape releases page of mangaupdates
SETUP:
reqwest requires OpenSSL headers

` apt install libssl-dev `
