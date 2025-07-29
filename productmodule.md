a@a:~/computers$ curl -X POST http://127.0.0.1:8080/api/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Samsung Galaxy S24",
    "description": "Latest Samsung smartphone with advanced camera and performance",
    "price": 45000000,
    "sale_price": 42000000,
    "sku": "SGS24-BLK-001",
    "stock_quantity": 25,
    "category_id": "68869e5778a65a997f98e654",
    "brand_id": "6886ae12356d11b628d38b31",
    "is_featured": true,
    "is_best_seller": false
  }'
{"_id":{"$oid":"688786dddd29989011abbe65"},"name":"Samsung Galaxy S24","description":"Latest Samsung smartphone with advanced camera and performance","slug":"samsung-galaxy-s24","price":45000000,"sale_price":42000000,"sku":"SGS24-BLK-001","stock_quantity":25,"category_id":{"$oid":"68869e5778a65a997f98e654"},"brand_id":{"$oid":"6886ae12356d11b628d38b31"},"images":[],"is_active":true,"is_featured":true,"is_new_arrival":true,"is_best_seller":false,"created_at":{"$date":{"$numberLong":"1753712349175"}},"updated_at":{"$date":{"$numberLong":"1753712349175"}}}a@a:~/computers$ 








a@a:~/computers$ curl -X GET http://127.0.0.1:8080/api/products
[{"_id":{"$oid":"688786dddd29989011abbe65"},"name":"Samsung Galaxy S24","description":"Latest Samsung smartphone with advanced camera and performance","slug":"samsung-galaxy-s24","price":45000000,"sale_price":42000000,"sku":"SGS24-BLK-001","stock_quantity":25,"category_id":{"$oid":"68869e5778a65a997f98e654"},"brand_id":{"$oid":"6886ae12356d11b628d38b31"},"images":[],"is_active":true,"is_featured":true,"is_new_arrival":true,"is_best_seller":false,"created_at":{"$date":{"$numberLong":"1753712349175"}},"updated_at":{"$date":{"$numberLong":"1753712349175"}}}]a@a:~/computers$ 






a@a:~/computers$ curl -X GET http://127.0.0.1:8080/api/products/688786dddd29989011abbe65
{"_id":{"$oid":"688786dddd29989011abbe65"},"name":"Samsung Galaxy S24","description":"Latest Samsung smartphone with advanced camera and performance","slug":"samsung-galaxy-s24","price":45000000,"sale_price":42000000,"sku":"SGS24-BLK-001","stock_quantity":25,"category_id":{"$oid":"68869e5778a65a997f98e654"},"brand_id":{"$oid":"6886ae12356d11b628d38b31"},"images":[],"is_active":true,"is_featured":true,"is_new_arrival":true,"is_best_seller":false,"created_at":{"$date":{"$numberLong":"1753712349175"}},"updated_at":{"$date":{"$numberLong":"1753712349175"}}}a@a:~/computers$ 





a@a:~/computers$ curl -X GET http://127.0.0.1:8080/api/products/featured
[{"_id":{"$oid":"688786dddd29989011abbe65"},"name":"Samsung Galaxy S24","description":"Latest Samsung smartphone with advanced camera and performance","slug":"samsung-galaxy-s24","price":45000000,"sale_price":42000000,"sku":"SGS24-BLK-001","stock_quantity":25,"category_id":{"$oid":"68869e5778a65a997f98e654"},"brand_id":{"$oid":"6886ae12356d11b628d38b31"},"images":[],"is_active":true,"is_featured":true,"is_new_arrival":true,"is_best_seller":false,"created_at":{"$date":{"$numberLong":"1753712349175"}},"updated_at":{"$date":{"$numberLong":"1753712349175"}}}]a@a:~/computers$ 





a@a:~/computers$ curl -X GET http://127.0.0.1:8080/api/products/new
[{"_id":{"$oid":"688786dddd29989011abbe65"},"name":"Samsung Galaxy S24","description":"Latest Samsung smartphone with advanced camera and performance","slug":"samsung-galaxy-s24","price":45000000,"sale_price":42000000,"sku":"SGS24-BLK-001","stock_quantity":25,"category_id":{"$oid":"68869e5778a65a997f98e654"},"brand_id":{"$oid":"6886ae12356d11b628d38b31"},"images":[],"is_active":true,"is_featured":true,"is_new_arrival":true,"is_best_seller":false,"created_at":{"$date":{"$numberLong":"1753712349175"}},"updated_at":{"$date":{"$numberLong":"1753712349175"}}}]a@a:~/computers$ 







a@a:~/computers$ curl -X GET http://127.0.0.1:8080/api/products/best-sellers
[]a@a:~/computers$ 


