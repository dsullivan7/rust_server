#!/bin/bash
API_URL="https://vested-server.pongo.us/api"

curl  -d '{"name": "Biotech"}' -H 'Content-Type: application/json' -H "Authorization: Bearer $1" -X POST "$API_URL/tags"
curl  -d '{"name": "Health Care"}' -H 'Content-Type: application/json' -H "Authorization: Bearer $1" -X POST "$API_URL/tags"
curl  -d '{"name": "Information Technology"}' -H 'Content-Type: application/json' -H "Authorization: Bearer $1" -X POST "$API_URL/tags"
curl  -d '{"name": "Renewable Energy"}' -H 'Content-Type: application/json' -H "Authorization: Bearer $1" -X POST "$API_URL/tags"
curl  -d '{"name": "Entertainment"}' -H 'Content-Type: application/json' -H "Authorization: Bearer $1" -X POST "$API_URL/tags"
curl  -d '{"name": "Real Estate"}' -H 'Content-Type: application/json' -H "Authorization: Bearer $1" -X POST "$API_URL/tags"
curl  -d '{"name": "Blockchain"}' -H 'Content-Type: application/json' -H "Authorization: Bearer $1" -X POST "$API_URL/tags"
curl  -d '{"name": "Financial Services"}' -H 'Content-Type: application/json' -H "Authorization: Bearer $1" -X POST "$API_URL/tags"
