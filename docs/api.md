# API Documentation for Data Node

This document outlines the API endpoints provided by the Data Node project, detailing request methods, parameters, and example responses. Data Node leverages Redis Stack Server to offer advanced search capabilities.

## Base URL

All URLs referenced in the API documentation have the base path http://localhost/

## Endpoints

### Index

```
curl --location 'http://localhost/index' \
--header 'Content-Type: application/json' \
--data '{
    "index_name": "myIndex",
    "type": "JSON",
    "language": "chinese",
    "prefixes": [
        "espn:"
    ],
    "schema": [
        {
            "field_name": "$.post_title as post_title",
            "field_type": "TEXT",
            "sortable": false
        },
        {
            "field_name": "$.post_message as post_message",
            "field_type": "TEXT",
            "sortable": false
        },
        {
            "field_name": "$.post_timestamp as post_timestamp",
            "field_type": "NUMERIC",
            "sortable": true
        },
        {
            "field_name": "$created_ts.created_ts",
            "field_type": "NUMERIC",
            "sortable": true
        }
    ]
}'
```

### Add

```
curl --location 'http://localhost/add' \
--header 'Content-Type: application/json' \
--data '[
    {
        "source": "espn",
        "medium": "online news platform",
        "site": "ESPN",
        "author": "John Doe",
        "channel": "NBA",
        "post_link": "https://espn.com/news/basketball/nba-highlight-of-the-week",
        "post_title": "NBA Highlight of the Week: Spectacular Dunk by LeBron James",
        "post_message": "In last night'\''s game, LeBron James secured the win for the Lakers with a spectacular dunk in the final seconds, marking a highlight moment of the week.",
        "post_date": "2024-02-27T09:48:52Z",
        "post_timestamp": 1709027273,
        "comment_count": 120,
        "reaction_count": 350,
        "share_count": 45,
        "view_count": 5000
    },
    {
        "source": "espn",
        "medium": "online news platform",
        "site": "ESPN",
        "author": "John Doe",
        "channel": "NBA",
        "post_link": "https://espn.com/news/basketball/nba-highlight-of-the-week",
        "post_title": "NBA Highlight of the Week: Spectacular Dunk by LeBron James",
        "post_message": "In last night'\''s game, LeBron James secured the win for the Lakers with a spectacular dunk in the final seconds, marking a highlight moment of the week.",
        "post_date": "2024-02-27T09:48:52Z",
        "post_timestamp": 1709027273,
        "comment_count": 120,
        "reaction_count": 350,
        "share_count": 45,
        "view_count": 5000
    }
]'
```

### Search

```
curl --location 'http://localhost/search' \
--header 'Content-Type: application/json' \
--data '{
    "index": "myIndex",
    "q": "*",
    "start_time": "2023-01-25 22:36:52",
    "end_time": "2025-01-25 22:36:52",
    "limit": 100,
    "offset": 0
}'
```
