curl -X POST http://127.0.0.1:8080/location -H 'Content-Type: application/json' -d '{"position":{"longitude":11.9632088240578,"latitude":57.716095833676924},"name":"bananpiren"}'

curl -X GET http://127.0.0.1:8080/location -H 'Content-Type: application/json' -d '{"boundingbox":{"north":58,"west":11,"south":57,"east":12}}'