curl -X POST http://127.0.0.1:8080/api/users/register -H "Content-Type: application/json" -d '{"first_name": "Wisdom", "last_name": "Volt", "email": "wisdomabraham92@gmail.com", "phone_number": "1234567899", "password": "password123", "confirm_password": "password123"}'
{"status":"success","message":"Registration successful. Please check your email for a verification code.","data":{"user_id":{"$oid":"687d8a1c3531cbe56ca3604f"}}}

a@a:~/computers$ curl -X POST http://127.0.0.1:8080/api/users/verify-email -H "Content-Type: application/json" -d '{"email": "anavheobaabraham@gmail.com", "verification_code": "685911"}'
{"status":"success","message":"Email verified successfully."}a@a:~/computers$ 


curl -X POST http://127.0.0.1:8080/api/users/delete-user -H "Content-Type: application/json" -d '{"email": "wisdomvolt@gmail.com"}'


curl -X POST http://127.0.0.1:8080/api/users/delete-user -H "Content-Type: application/json" -d '{"email": "wisdomabraham92@gmail.com"}'


curl -X POST http://127.0.0.1:8080/api/users/register -H "Content-Type: application/json" -d '{"first_name": "Wisdom", "last_name": "Volt", "email": "wisdomvolt@gmail.com", "phone_number": "1234467899", "password": "password123", "confirm_password": "password123"}'
{"status":"success","message":"Registration successful. Please check your email for a verification code.","data":{"user_id":{"$oid":"687d8a1c3531cbe56ca3604f"}}}



a@a:~/computers$ curl -X POST http://127.0.0.1:8080/api/users/resend-verification -H "Content-Type: application/json" -d '{"email": "wisdomabraham92@gmail.com"}'
{"status":"success","message":"Verification code has been resent. Please check your email (including spam folder)."}a@a:~/computers$ 





a@a:~/computers$ curl -X POST http://127.0.0.1:8080/api/users/login -H "Content-Type: application/json" -d '{"email": "wisdomabraham92@gmail.com", "password": "password123"}'
{"status":"success","message":"Login successful","data":{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODdkOWU4NzE4YzMwMzI2NGNlNmIzYjAiLCJleHAiOjE3NTMxNTIwOTMsImlhdCI6MTc1MzA2NTY5MywianRpIjoiNTIyODE1ZTctM2RjYi00ZTJkLTk4YTktYTk0ZmNhODY0NzBhIiwiZW1haWwiOiJ3aXNkb21hYnJhaGFtOTJAZ21haWwuY29tIiwiZmlyc3RfbmFtZSI6IlRlc3QiLCJsYXN0X25hbWUiOiJVc2VyIn0.mHChkvPhnrDnir60up7fKGEk-lM_FR5FIDW6wmgvAjo","user":{"user_id":"687d9e8718c303264ce6b3b0","email":"wisdomabraham92@gmail.com","first_name":"Test","last_name":"User","is_verified":true}}}a@a:~/computers$ 