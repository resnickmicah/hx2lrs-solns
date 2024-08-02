curl --location 'localhost:8000/ticket' \
--header 'Content-Type: application/json' \
--data '{"title": "Add ticket list endpoint", "description": "Add GET endpoint for /ticket which lists all tickets with status != Done"}'

curl --location 'localhost:8000/ticket/1'

