# cato

Fetches the titles of the newset 20 posts from [r/dota2](https://reddit.com/r/dota2/new) and stores them into a redis cache.

The titles are stored simply as a key-value pair using [`SET`](https://redis.io/commands/set), where the key is the post's ID.

This program will do one fetch-store operation per run (as opposed to continuously running). Set up a `cron` task if you want to update your store at regular intervals.

## redis

You need to have `redis-server` available at `redis://127.0.0.1`.

## Output

The program will output all errors to `stderr`. When posts are stored, a message will be output to `stdout`.