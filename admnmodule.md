curl -X POST http://127.0.0.1:8080/api/admin-public/login \
-H "Content-Type: application/json" \
-d '{"email": "admin@example.com", "password": "admin123456"}'
{"status":"success","message":"Login successful","data":{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODdlZTUzMjBkN2I3MjE1MWNjZDBiZmYiLCJleHAiOjE3NTMzNzA5NzcsImlhdCI6MTc1MzI4NDU3NywianRpIjoiYTY2N2QzOGItOWMyZC00YmVlLTg1MzctOGIwOGY3ZTViOWFmIiwiZW1haWwiOiJhZG1pbkBleGFtcGxlLmNvbSIsImZpcnN0X25hbWUiOiJBZG1pbiIsImxhc3RfbmFtZSI6IlVzZXIiLCJyb2xlIjoiQWRtaW4ifQ.u8AID_HEGcGtXWM6V66yzWDU1T23UaR01lpk6o5-qfk"}}