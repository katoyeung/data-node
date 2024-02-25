# Data Node

## Docker Operation

```
docker build -t data-node .

docker run -p 8080:8080 data-node

docker run -v ./data/:/data -d --name redis-stack-server -p 16379:6379 -e REDIS_ARGS="--requirepass password" redis/redis-stack-server:latest
```

## Redis Operation

```
FT.CREATE twitterIndex ON HASH PREFIX 1 "twitter:" SCHEMA userId TEXT SORTABLE tweet TEXT content TEXT createdAt DATE SORTABLE hashtags TAG SORTABLE mentions TAG SORTABLE

FT.DROPINDEX YourIndexName
FT.CREATE YourIndexName ON JSON PREFIX 1 json: LANGUAGE chinese SCHEMA $.post_title as post_title TEXT $.post_message as post_message TEXT $.post_timestamp as post_timestamp NUMERIC SORTABLE $created_ts.created_ts NUMERIC SORTABLE

FT.CREATE socialMediaIndex ON HASH PREFIX 3 "twitter:" "facebook:" "instagram:" SCHEMA source TEXT SORTABLE content TEXT createdAt DATE SORTABLE hashtags TAG SORTABLE mentions TAG SORTABLE

SCAN 0 MATCH json\* COUNT 100
```
