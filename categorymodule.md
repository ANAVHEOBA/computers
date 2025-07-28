a@a:~/computers$ curl -X POST http://127.0.0.1:8080/api/categories   -H "Content-Type: application/json"   -d '{
    "name": "Phones & Tablets",
    "description": "Smartphones, tablets, and mobile accessories"
  }'
{"_id":{"$oid":"68869e5778a65a997f98e654"},"name":"Phones & Tablets","description":"Smartphones, tablets, and mobile accessories","slug":"phones-&-tablets","is_active":true,"display_order":0,"created_at":{"$date":{"$numberLong":"1753652823616"}},"updated_at":{"$date":{"$numberLong":"1753652823616"}}}a@a:~/computers$ 



curl -X GET http://127.0.0.1:8080/api/categories/68869e5778a65a997f98e654
{"_id":{"$oid":"68869e5778a65a997f98e654"},"name":"Phones & Tablets","description":"Smartphones, tablets, and mobile accessories","slug":"phones-&-tablets","is_active":true,"display_order":0,"created_at":{"$date":{"$numberLong":"1753652823616"}},"updated_at":{"$date":{"$numberLong":"1753652823616"}}}




a@a:~/computers$ curl -X GET http://127.0.0.1:8080/api/categories
[{"_id":{"$oid":"68869e5778a65a997f98e654"},"name":"Phones & Tablets","description":"Smartphones, tablets, and mobile accessories","slug":"phones-&-tablets","is_active":true,"display_order":0,"created_at":{"$date":{"$numberLong":"1753652823616"}},"updated_at":{"$date":{"$numberLong":"1753652823616"}}},{"_id":{"$oid":"6886a3b14b3bcead934c6751"},"name":"Phones & Tablets","description":"Smartphones, tablets, and mobile accessories","slug":"phones-&-tablets","is_active":true,"display_order":0,"created_at":{"$date":{"$numberLong":"1753654193948"}},"updated_at":{"$date":{"$numberLong":"1753654193948"}}}]a@a:~/computers$ 
