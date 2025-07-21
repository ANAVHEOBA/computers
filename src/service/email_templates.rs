use rand::Rng;

/// Generates a random 6-digit verification code.
pub fn generate_verification_code() -> String {
    let code = rand::rng().random_range(0..1_000_000);
    format!("{:06}", code)
}

/// Creates the HTML content for the verification email.
pub fn get_verification_email_template(name: &str, code: &str) -> (String, String) {
    let subject = "Your Email Verification Code".to_string();

    let body = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Email Verification</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333333;
            max-width: 600px;
            margin: 0 auto;
            padding: 20px;
        }}
        .container {{
            background-color: #ffffff;
            border-radius: 5px;
            padding: 30px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
        }}
        .header {{
            text-align: center;
            padding-bottom: 20px;
            border-bottom: 1px solid #eeeeee;
        }}
        .verification-code {{
            background-color: #f5f5f5;
            font-size: 32px;
            font-weight: bold;
            text-align: center;
            padding: 20px;
            margin: 20px 0;
            letter-spacing: 5px;
            border-radius: 5px;
        }}
        .footer {{
            margin-top: 30px;
            font-size: 12px;
            color: #666666;
            text-align: center;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h2>Email Verification Required</h2>
        </div>
        
        <p>Hello {},</p>
        
        <p>Thank you for creating an account. To ensure the security of your account, please verify your email address using the following verification code:</p>
        
        <div class="verification-code">
            {}
        </div>
        
        <p><strong>Important:</strong></p>
        <ul>
            <li>This code will expire in 10 minutes</li>
            <li>If you didn't request this code, please ignore this email</li>
            <li>Do not share this code with anyone</li>
        </ul>
        
        <div class="footer">
            <p>This is an automated message, please do not reply to this email.</p>
            <p>&copy; 2024 Your Application Name. All rights reserved.</p>
        </div>
    </div>
</body>
</html>"#,
        name, code
    );

    (subject, body)
}
