import feedparser
import os
import sys

url = sys.argv[1]
feed = feedparser.parse(url)

if len(feed.entries) > 0 and not os.path.isdir('posts'):
    os.mkdir('posts')
for entry in feed.entries:
    filename = entry.link.split("/")[-1].replace(".html", ".md")
    date_parts = entry.published.split(".")
    published_date = date_parts[0] + "+" + date_parts[1].split("+")[1]
    if "tags" in entry:
        tags = ",".join([x.term for x in entry.tags])
    else:
        tags = ""
    post = open('posts/' + filename, "w")
    meta_data = "<!--\n\
.. title: {title}\n\
.. slug: {slug}\n\
.. date: {date}\n\
.. tags: {tags}\n\
.. link: \n\
.. description:\n\
.. type: text\n\
-->\n\n".format(
        title=entry.title,
        slug=filename.replace(".md",""),
        date=published_date,
        tags=tags
    )
    post.write(meta_data)
    post.write(entry.content[0].value)
    post.close()