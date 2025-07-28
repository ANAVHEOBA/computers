a@a:~/computers$ curl -X POST http://127.0.0.1:8080/api/brands \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Samsung",
    "description": "Leading technology company specializing in smartphones, tablets, and electronics",
    "website": "https://www.samsung.com"
  }'
{"_id":{"$oid":"6886ae12356d11b628d38b31"},"name":"Samsung","description":"Leading technology company specializing in smartphones, tablets, and electronics","slug":"samsung","website":"https://www.samsung.com","is_active":true,"display_order":0,"created_at":{"$date":{"$numberLong":"1753656850688"}},"updated_at":{"$date":{"$numberLong":"1753656850688"}}}a@a:~/computers$ 



a@a:~/computers$ curl -X GET http://127.0.0.1:8080/api/brands
[{"_id":{"$oid":"6886ae12356d11b628d38b31"},"name":"Samsung","description":"Leading technology company specializing in smartphones, tablets, and electronics","slug":"samsung","website":"https://www.samsung.com","is_active":true,"display_order":0,"created_at":{"$date":{"$numberLong":"1753656850688"}},"updated_at":{"$date":{"$numberLong":"1753656850688"}}}]a@a:~/computers$ 




a@a:~/computers$ curl -X GET http://127.0.0.1:8080/api/brands/6886ae12356d11b628d38b31
{"_id":{"$oid":"6886ae12356d11b628d38b31"},"name":"Samsung","description":"Leading technology company specializing in smartphones, tablets, and electronics","slug":"samsung","website":"https://www.samsung.com","is_active":true,"display_order":0,"created_at":{"$date":{"$numberLong":"1753656850688"}},"updated_at":{"$date":{"$numberLong":"1753656850688"}}}a@a:~/computers$ 