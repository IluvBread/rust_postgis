curl -X POST http://127.0.0.1:8080/location -H 'Content-Type: application/json' -d '{"position":{"longitude":11.9632088240578,"latitude":57.716095833676924},"name":"bananpiren"}'

curl -X GET http://127.0.0.1:8080/location -H 'Content-Type: application/json' -d '{"boundingbox":{"north":58,"west":11,"south":57,"east":12}}'

curl -X POST http://127.0.0.1:8080/location -H 'Content-Type: application/json' -d '{"boundingbox":{"north":58,"west":11,"south":57,"east":12}}'

curl -X POST http://127.0.0.1:8080/location/timeslot -H 'Content-Type: application/json' -d '{
   "location_id":"6e6f8684-6c8f-4a47-90a7-baf55f49d283",
   "start_time": "2020-12-09T16:09:53+00:00",
   "end_time": "2020-12-09T16:09:53+00:00"
}'