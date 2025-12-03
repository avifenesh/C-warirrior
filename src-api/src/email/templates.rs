//! Email templates for transactional emails

/// Email template types
pub enum EmailTemplate {
    /// Email verification after registration
    Verification {
        username: Option<String>,
        verification_link: String,
    },
    /// Password reset request
    PasswordReset {
        username: Option<String>,
        reset_link: String,
    },
    /// Welcome email after verification
    Welcome { username: Option<String> },
}

impl EmailTemplate {
    /// Get the subject line for this template
    pub fn subject(&self) -> &'static str {
        match self {
            EmailTemplate::Verification { .. } => "Verify your Code Warrior account",
            EmailTemplate::PasswordReset { .. } => "Reset your Code Warrior password",
            EmailTemplate::Welcome { .. } => "Welcome to Code Warrior!",
        }
    }

    /// Render the HTML body for this template
    pub fn render_html(&self) -> String {
        match self {
            EmailTemplate::Verification {
                username,
                verification_link,
            } => self.render_verification_html(username.as_deref(), verification_link),
            EmailTemplate::PasswordReset {
                username,
                reset_link,
            } => self.render_password_reset_html(username.as_deref(), reset_link),
            EmailTemplate::Welcome { username } => self.render_welcome_html(username.as_deref()),
        }
    }

    /// Render plain text version for this template
    pub fn render_text(&self) -> String {
        match self {
            EmailTemplate::Verification {
                username,
                verification_link,
            } => self.render_verification_text(username.as_deref(), verification_link),
            EmailTemplate::PasswordReset {
                username,
                reset_link,
            } => self.render_password_reset_text(username.as_deref(), reset_link),
            EmailTemplate::Welcome { username } => self.render_welcome_text(username.as_deref()),
        }
    }

    fn render_verification_html(&self, username: Option<&str>, link: &str) -> String {
        let greeting = username
            .map(|u| format!("Hey {},", u))
            .unwrap_or_else(|| "Hey there,".to_string());

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Verify your email</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
            color: #e2e8f0;
            margin: 0;
            padding: 40px 20px;
        }}
        .container {{
            max-width: 560px;
            margin: 0 auto;
            background: #1e293b;
            border-radius: 12px;
            border: 1px solid #334155;
            overflow: hidden;
        }}
        .header {{
            background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
            padding: 32px;
            text-align: center;
        }}
        .header h1 {{
            margin: 0;
            color: #0f172a;
            font-size: 28px;
            font-weight: 700;
            letter-spacing: 2px;
        }}
        .content {{
            padding: 32px;
        }}
        .greeting {{
            font-size: 18px;
            margin-bottom: 16px;
        }}
        .message {{
            color: #94a3b8;
            line-height: 1.6;
            margin-bottom: 24px;
        }}
        .button {{
            display: inline-block;
            background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
            color: #fff !important;
            text-decoration: none;
            padding: 14px 32px;
            border-radius: 8px;
            font-weight: 600;
            font-size: 16px;
        }}
        .button:hover {{
            background: linear-gradient(135deg, #16a34a 0%, #15803d 100%);
        }}
        .footer {{
            padding: 24px 32px;
            background: #0f172a;
            text-align: center;
            color: #64748b;
            font-size: 12px;
        }}
        .link-fallback {{
            color: #64748b;
            font-size: 12px;
            word-break: break-all;
            margin-top: 16px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>⚔️ CODE WARRIOR</h1>
        </div>
        <div class="content">
            <p class="greeting">{greeting}</p>
            <p class="message">
                Thanks for joining Code Warrior! Before you can start your journey to master C programming,
                please verify your email address by clicking the button below.
            </p>
            <p style="text-align: center; margin: 32px 0;">
                <a href="{link}" class="button">Verify Email</a>
            </p>
            <p class="message">
                This link will expire in 24 hours. If you didn't create an account, you can safely ignore this email.
            </p>
            <p class="link-fallback">
                If the button doesn't work, copy and paste this link into your browser:<br>
                {link}
            </p>
        </div>
        <div class="footer">
            <p>© 2024 Code Warrior. Master C programming through adventure.</p>
        </div>
    </div>
</body>
</html>"#
        )
    }

    fn render_verification_text(&self, username: Option<&str>, link: &str) -> String {
        let greeting = username
            .map(|u| format!("Hey {},", u))
            .unwrap_or_else(|| "Hey there,".to_string());

        format!(
            r#"CODE WARRIOR - Verify Your Email

{greeting}

Thanks for joining Code Warrior! Before you can start your journey to master C programming, please verify your email address by visiting the link below:

{link}

This link will expire in 24 hours. If you didn't create an account, you can safely ignore this email.

---
© 2024 Code Warrior. Master C programming through adventure."#
        )
    }

    fn render_password_reset_html(&self, username: Option<&str>, link: &str) -> String {
        let greeting = username
            .map(|u| format!("Hey {},", u))
            .unwrap_or_else(|| "Hey there,".to_string());

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Reset your password</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
            color: #e2e8f0;
            margin: 0;
            padding: 40px 20px;
        }}
        .container {{
            max-width: 560px;
            margin: 0 auto;
            background: #1e293b;
            border-radius: 12px;
            border: 1px solid #334155;
            overflow: hidden;
        }}
        .header {{
            background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
            padding: 32px;
            text-align: center;
        }}
        .header h1 {{
            margin: 0;
            color: #fff;
            font-size: 28px;
            font-weight: 700;
            letter-spacing: 2px;
        }}
        .content {{
            padding: 32px;
        }}
        .greeting {{
            font-size: 18px;
            margin-bottom: 16px;
        }}
        .message {{
            color: #94a3b8;
            line-height: 1.6;
            margin-bottom: 24px;
        }}
        .button {{
            display: inline-block;
            background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
            color: #0f172a !important;
            text-decoration: none;
            padding: 14px 32px;
            border-radius: 8px;
            font-weight: 600;
            font-size: 16px;
        }}
        .footer {{
            padding: 24px 32px;
            background: #0f172a;
            text-align: center;
            color: #64748b;
            font-size: 12px;
        }}
        .link-fallback {{
            color: #64748b;
            font-size: 12px;
            word-break: break-all;
            margin-top: 16px;
        }}
        .warning {{
            background: #451a03;
            border: 1px solid #92400e;
            border-radius: 8px;
            padding: 12px 16px;
            color: #fbbf24;
            font-size: 13px;
            margin-top: 24px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 PASSWORD RESET</h1>
        </div>
        <div class="content">
            <p class="greeting">{greeting}</p>
            <p class="message">
                We received a request to reset your Code Warrior password. Click the button below to create a new password.
            </p>
            <p style="text-align: center; margin: 32px 0;">
                <a href="{link}" class="button">Reset Password</a>
            </p>
            <p class="message">
                This link will expire in 1 hour for security reasons.
            </p>
            <div class="warning">
                ⚠️ If you didn't request a password reset, please ignore this email. Your password will remain unchanged.
            </div>
            <p class="link-fallback">
                If the button doesn't work, copy and paste this link into your browser:<br>
                {link}
            </p>
        </div>
        <div class="footer">
            <p>© 2024 Code Warrior. Master C programming through adventure.</p>
        </div>
    </div>
</body>
</html>"#
        )
    }

    fn render_password_reset_text(&self, username: Option<&str>, link: &str) -> String {
        let greeting = username
            .map(|u| format!("Hey {},", u))
            .unwrap_or_else(|| "Hey there,".to_string());

        format!(
            r#"CODE WARRIOR - Password Reset

{greeting}

We received a request to reset your Code Warrior password. Visit the link below to create a new password:

{link}

This link will expire in 1 hour for security reasons.

⚠️ If you didn't request a password reset, please ignore this email. Your password will remain unchanged.

---
© 2024 Code Warrior. Master C programming through adventure."#
        )
    }

    fn render_welcome_html(&self, username: Option<&str>) -> String {
        let greeting = username
            .map(|u| format!("Welcome, {}!", u))
            .unwrap_or_else(|| "Welcome, Warrior!".to_string());

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome to Code Warrior</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
            color: #e2e8f0;
            margin: 0;
            padding: 40px 20px;
        }}
        .container {{
            max-width: 560px;
            margin: 0 auto;
            background: #1e293b;
            border-radius: 12px;
            border: 1px solid #334155;
            overflow: hidden;
        }}
        .header {{
            background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
            padding: 32px;
            text-align: center;
        }}
        .header h1 {{
            margin: 0;
            color: #fff;
            font-size: 28px;
            font-weight: 700;
            letter-spacing: 2px;
        }}
        .content {{
            padding: 32px;
        }}
        .greeting {{
            font-size: 24px;
            margin-bottom: 16px;
            color: #fbbf24;
        }}
        .message {{
            color: #94a3b8;
            line-height: 1.6;
            margin-bottom: 24px;
        }}
        .feature {{
            display: flex;
            align-items: flex-start;
            margin-bottom: 16px;
        }}
        .feature-icon {{
            font-size: 24px;
            margin-right: 12px;
        }}
        .feature-text {{
            color: #cbd5e1;
        }}
        .button {{
            display: inline-block;
            background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
            color: #0f172a !important;
            text-decoration: none;
            padding: 14px 32px;
            border-radius: 8px;
            font-weight: 600;
            font-size: 16px;
        }}
        .footer {{
            padding: 24px 32px;
            background: #0f172a;
            text-align: center;
            color: #64748b;
            font-size: 12px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>⚔️ CODE WARRIOR</h1>
        </div>
        <div class="content">
            <p class="greeting">{greeting}</p>
            <p class="message">
                Your email has been verified and you're ready to begin your journey to master C programming!
            </p>
            <p class="message" style="font-weight: 600; color: #e2e8f0;">What awaits you:</p>
            <div class="feature">
                <span class="feature-icon">🗡️</span>
                <span class="feature-text">Battle through coding challenges that teach real C concepts</span>
            </div>
            <div class="feature">
                <span class="feature-icon">🏰</span>
                <span class="feature-text">Explore a world where memory management is magic</span>
            </div>
            <div class="feature">
                <span class="feature-icon">⭐</span>
                <span class="feature-text">Earn XP and unlock new levels as you progress</span>
            </div>
            <p style="text-align: center; margin: 32px 0;">
                <a href="https://code-warrior-seven.vercel.app" class="button">Start Your Adventure</a>
            </p>
        </div>
        <div class="footer">
            <p>© 2024 Code Warrior. Master C programming through adventure.</p>
        </div>
    </div>
</body>
</html>"#
        )
    }

    fn render_welcome_text(&self, username: Option<&str>) -> String {
        let greeting = username
            .map(|u| format!("Welcome, {}!", u))
            .unwrap_or_else(|| "Welcome, Warrior!".to_string());

        format!(
            r#"CODE WARRIOR - Welcome!

{greeting}

Your email has been verified and you're ready to begin your journey to master C programming!

What awaits you:

🗡️ Battle through coding challenges that teach real C concepts
🏰 Explore a world where memory management is magic
⭐ Earn XP and unlock new levels as you progress

Start your adventure: https://code-warrior-seven.vercel.app

---
© 2024 Code Warrior. Master C programming through adventure."#
        )
    }
}

