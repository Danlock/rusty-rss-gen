GOAL:
Website that generates an RSS feed containing updates for your specified manga. Updates feed many  times a day.
Every manga on Mangaupdates should return an entry on the RSS feed, at least to the current entry in Mangaupdates.
The database and code should be structured in a way thast makes this easy to expand to anime at least, tv and comic possibly.

Pulls in data from several (arbitrary?) sources:

  https://www.mangaupdates.com/releases.html (Highest priority, web scrape necessary)
  https://www.tokyotosho.info/rss_customize.php  (Decent users, RSS feed easily customizable)
  https://nyaa.si/ (No real API, unfortunately most popular)
  https://nyaa.pantsu.cat/apidoc/ (Low users but easy to integrate)


1. Scrape releases page of mangaupdates
SETUP:
reqwest requires OpenSSL headers

` apt install libssl-dev `
