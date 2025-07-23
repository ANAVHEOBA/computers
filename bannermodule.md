a@a:~/computers$ curl -X POST http://127.0.0.1:8080/api/banners \
-H "Content-Type: application/json" \
-H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODdlZTUzMjBkN2I3MjE1MWNjZDBiZmYiLCJleHAiOjE3NTMzNzU2OTEsImlhdCI6MTc1MzI4OTI5MSwianRpIjoiYzhmYjM2ZTktNWIxYy00ZGJjLWExMzItNjhkMmY1M2Y0NzMzIiwiZW1haWwiOiJhZG1pbkBleGFtcGxlLmNvbSIsImZpcnN0X25hbWUiOiJBZG1pbiIsImxhc3RfbmFtZSI6IlVzZXIiLCJyb2xlIjoiQWRtaW4ifQ.JUo_rJZfQf3E2JPsGB_JEassC1xqP99HtZgBdM3pNw0" \
-d '{
  "title": "Admin Created Banner",
  "description": "This banner was created with admin privileges",
  "image_data": "./360_F_429972464_g2u64pPItYZlJzKeYY6Um63N54feS2Fz.webp"
}'
{"_id":{"$oid":"688112a64583eedccd6f8b30"},"title":"Admin Created Banner","description":"This banner was created with admin privileges","image_url":"https://res.cloudinary.com/ddchq4qqv/image/upload/v1753289381/banners/a3nknwre2m6zjq5xhnbv.webp","link_url":null,"is_active":true,"display_order":0,"start_date":null,"end_date":null,"created_at":"2025-07-23T16:49:42.484684038Z","updated_at":"2025-07-23T16:49:42.484684038Z"}a@a:~/computers$ 


a@a:~/computers$ curl -X PUT http://127.0.0.1:8080/api/banners/688112a64583eedccd6f8b30 \
a@a:~/computers$ curl -X PUT http://127.0.0.1:8080/api/banners/688112a64583eedccd6f8b30 \
-H "Content-Type: application/json" \KV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODdlZTUzMjBkN2I3MjE1MWNjZDBiZmYiL
-H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODdlZTUzMjBkN2I3MjE1MWNjZDBiZmYiLCJleHAiOjE3NTMzNzU2OTEsImlhdCI6MTc1MzI4OTI5MSwianRpIjoiYzhmYjM2ZTktNWIxYy00ZGJjLWExMzItNjhkMmY1M2Y0NzMzIiwiZW1haWwiOiJhZG1pbkBleGFtcGxlLmNvbSIsImZpcnN0X25hbWUiOiJBZG1pbiIsImxhc3RfbmFtZSI6IlVzZXIiLCJyb2xlIjoiQWRtaW4ifQ.JUo_rJZfQf3E2JPsGB_JEassC1xqP99HtZgBdM3pNw0" \
-d '{tle": "Updated Banner Title",
  "title": "Updated Banner Title",ion",
  "description": "Updated description",
  "is_active": true,
  "display_order": 1
}'
{"_id":{"$oid":"688112a64583eedccd6f8b30"},"title":"Admin Created Banner","description":"This banner was created with admin privileges","image_url":"https://res.cloudinary.com/ddchq4qqv/image/upload/v1753289381/banners/a3nknwre2m6zjq5xhnbv.webp","link_url":null,"is_active":true,"display_order":0,"start_date":null,"end_date":null,"created_at":"2025-07-23T16:49:42.484684038Z","updated_at":"2025-07-23T16:49:42.484684038Z"}a@a:~/computers$ 



a@a:~/computers$ curl -X DELETE http://127.0.0.1:8080/api/banners/688112a64583eedccd6f8b30 \
-H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODdlZTUzMjBkN2I3MjE1MWNjZDBiZmYiLCJleHAiOjE3NTMzNzU2OTEsImlhdCI6MTc1MzI4OTI5MSwianRpIjoiYzhmYjM2ZTktNWIxYy00ZGJjLWExMzItNjhkMmY1M2Y0NzMzIiwiZW1haWwiOiJhZG1pbkBleGFtcGxlLmNvbSIsImZpcnN0X25hbWUiOiJBZG1pbiIsImxhc3RfbmFtZSI6IlVzZXIiLCJyb2xlIjoiQWRtaW4ifQ.JUo_rJZfQf3E2JPsGB_JEassC1xqP99HtZgBdM3pNw0"
a@a:~/computers$ 



a@a:~/computers$ # Get all active banners
curl http://127.0.0.1:8080/api/banners
[{"_id":{"$oid":"68810a8d82bc8e16f0450009"},"title":"My Test Banner","description":"A great new offer!","image_url":"https://res.cloudinary.com/ddchq4qqv/image/upload/v1753287308/banners/whjdba53hyvutdtql6tf.webp","link_url":"https://example.com/offer","is_active":true,"display_order":0,"start_date":null,"end_date":null,"created_at":"2025-07-23T16:15:09.698492043Z","updated_at":"2025-07-23T16:15:09.698492043Z"},{"_id":{"$oid":"68810be382bc8e16f045000a"},"title":"My Test Banner","description":"A great new offer!","image_url":"https://res.cloudinary.com/ddchq4qqv/image/upload/v1753287650/banners/cdkxmqsfsru6junymwk5.webp","link_url":null,"is_active":true,"display_order":0,"start_date":null,"end_date":null,"created_at":"2025-07-23T16:20:51.973111870Z","updated_at":"2025-07-23T16:20:51.973111870Z"}]a@a:~/computers$ 