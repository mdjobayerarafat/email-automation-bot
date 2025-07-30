export interface PremadeTemplate {
  id: string;
  name: string;
  category: string;
  subject: string;
  html_content: string;
  text_content: string;
  variables: string[];
  preview_image?: string;
  description: string;
}

export const TEMPLATE_CATEGORIES = [
  'Business',
  'Marketing',
  'Newsletter',
  'Welcome',
  'Transactional',
  'Event',
  'E-commerce',
  'Personal'
];

export const PREMADE_TEMPLATES: PremadeTemplate[] = [
  {
    id: 'welcome-modern',
    name: 'Modern Welcome',
    category: 'Welcome',
    subject: 'Welcome to {{company_name}}, {{first_name}}!',
    description: 'A clean, modern welcome email with gradient design',
    variables: ['first_name', 'company_name', 'support_email'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome Email</title>
</head>
<body style="margin: 0; padding: 0; font-family: Arial, sans-serif; background-color: #f4f4f4;">
    <table width="100%" cellpadding="0" cellspacing="0" style="background-color: #f4f4f4; padding: 20px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 10px; overflow: hidden;">
                    <tr>
                        <td style="padding: 40px; text-align: center; color: white;">
                            <h1 style="margin: 0; font-size: 28px; font-weight: bold;">Welcome to {{company_name}}!</h1>
                            <p style="margin: 20px 0; font-size: 18px; opacity: 0.9;">Hi {{first_name}}, we're excited to have you on board!</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: white; padding: 40px; text-align: center;">
                            <h2 style="color: #333; margin-bottom: 20px;">Get Started</h2>
                            <p style="color: #666; line-height: 1.6; margin-bottom: 30px;">Thank you for joining our community. We're here to help you succeed every step of the way.</p>
                            <a href="#" style="display: inline-block; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 15px 30px; text-decoration: none; border-radius: 5px; font-weight: bold;">Get Started</a>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: #f8f9fa; padding: 20px; text-align: center; color: #666; font-size: 14px;">
                            <p>Need help? Contact us at <a href="mailto:{{support_email}}" style="color: #667eea;">{{support_email}}</a></p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `Welcome to {{company_name}}!\n\nHi {{first_name}}, we're excited to have you on board!\n\nThank you for joining our community. We're here to help you succeed every step of the way.\n\nNeed help? Contact us at {{support_email}}`
  },
  {
    id: 'newsletter-tech',
    name: 'Tech Newsletter',
    category: 'Newsletter',
    subject: '{{newsletter_title}} - {{date}}',
    description: 'Professional newsletter template for tech companies',
    variables: ['newsletter_title', 'date', 'main_article_title', 'main_article_excerpt', 'company_name'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Newsletter</title>
</head>
<body style="margin: 0; padding: 0; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #f5f5f5;">
    <table width="100%" cellpadding="0" cellspacing="0" style="background-color: #f5f5f5; padding: 20px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: white; border-radius: 8px; overflow: hidden; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="background: #2563eb; padding: 30px; text-align: center;">
                            <h1 style="color: white; margin: 0; font-size: 24px;">{{newsletter_title}}</h1>
                            <p style="color: #bfdbfe; margin: 10px 0 0 0;">{{date}}</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="color: #1f2937; margin-bottom: 15px; font-size: 20px;">{{main_article_title}}</h2>
                            <p style="color: #4b5563; line-height: 1.6; margin-bottom: 25px;">{{main_article_excerpt}}</p>
                            <a href="#" style="display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; font-weight: 500;">Read More</a>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: #f9fafb; padding: 30px; text-align: center; border-top: 1px solid #e5e7eb;">
                            <p style="color: #6b7280; margin: 0; font-size: 14px;">© {{company_name}}. All rights reserved.</p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `{{newsletter_title}} - {{date}}\n\n{{main_article_title}}\n\n{{main_article_excerpt}}\n\n© {{company_name}}. All rights reserved.`
  },
  {
    id: 'business-proposal',
    name: 'Business Proposal',
    category: 'Business',
    subject: 'Business Proposal: {{proposal_title}}',
    description: 'Professional business proposal email template',
    variables: ['client_name', 'proposal_title', 'company_name', 'contact_person', 'phone'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Business Proposal</title>
</head>
<body style="margin: 0; padding: 0; font-family: Georgia, serif; background-color: #ffffff;">
    <table width="100%" cellpadding="0" cellspacing="0" style="padding: 40px 20px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="border: 2px solid #1f2937; background: white;">
                    <tr>
                        <td style="background: #1f2937; padding: 30px; text-align: center;">
                            <h1 style="color: white; margin: 0; font-size: 26px; letter-spacing: 1px;">{{company_name}}</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <p style="color: #374151; margin-bottom: 20px; font-size: 16px;">Dear {{client_name}},</p>
                            <h2 style="color: #1f2937; margin-bottom: 20px; font-size: 22px;">{{proposal_title}}</h2>
                            <p style="color: #4b5563; line-height: 1.7; margin-bottom: 25px;">We are pleased to present this business proposal for your consideration. Our team has carefully analyzed your requirements and developed a comprehensive solution.</p>
                            <p style="color: #4b5563; line-height: 1.7; margin-bottom: 30px;">We believe this partnership will bring significant value to your organization and look forward to discussing the details with you.</p>
                            <div style="background: #f9fafb; padding: 20px; border-left: 4px solid #1f2937; margin-bottom: 30px;">
                                <p style="margin: 0; color: #374151; font-style: italic;">"Excellence is not a skill, it's an attitude."</p>
                            </div>
                            <p style="color: #4b5563; margin-bottom: 30px;">Please feel free to contact us to schedule a meeting at your convenience.</p>
                            <p style="color: #374151; margin-bottom: 5px;">Best regards,</p>
                            <p style="color: #1f2937; font-weight: bold; margin-bottom: 5px;">{{contact_person}}</p>
                            <p style="color: #6b7280; margin: 0;">{{phone}}</p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `Dear {{client_name}},\n\n{{proposal_title}}\n\nWe are pleased to present this business proposal for your consideration. Our team has carefully analyzed your requirements and developed a comprehensive solution.\n\nWe believe this partnership will bring significant value to your organization and look forward to discussing the details with you.\n\nPlease feel free to contact us to schedule a meeting at your convenience.\n\nBest regards,\n{{contact_person}}\n{{phone}}`
  },
  {
    id: 'marketing-promo',
    name: 'Promotional Campaign',
    category: 'Marketing',
    subject: '🎉 {{discount_percent}}% OFF - {{product_name}}',
    description: 'Eye-catching promotional email with discount offer',
    variables: ['customer_name', 'product_name', 'discount_percent', 'promo_code', 'expiry_date'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Special Offer</title>
</head>
<body style="margin: 0; padding: 0; font-family: Arial, sans-serif; background: linear-gradient(135deg, #ff6b6b, #feca57);">
    <table width="100%" cellpadding="0" cellspacing="0" style="padding: 20px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: white; border-radius: 15px; overflow: hidden; box-shadow: 0 10px 30px rgba(0,0,0,0.2);">
                    <tr>
                        <td style="background: linear-gradient(135deg, #ff6b6b, #feca57); padding: 40px; text-align: center;">
                            <h1 style="color: white; margin: 0; font-size: 32px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);">🎉 SPECIAL OFFER!</h1>
                            <p style="color: white; margin: 15px 0 0 0; font-size: 18px; opacity: 0.9;">Limited Time Only</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px; text-align: center;">
                            <h2 style="color: #333; margin-bottom: 15px; font-size: 24px;">Hi {{customer_name}}!</h2>
                            <p style="color: #666; font-size: 18px; margin-bottom: 30px;">Get <strong style="color: #ff6b6b; font-size: 24px;">{{discount_percent}}% OFF</strong> on {{product_name}}</p>
                            <div style="background: #f8f9fa; border: 2px dashed #ff6b6b; padding: 20px; margin: 30px 0; border-radius: 10px;">
                                <p style="margin: 0; color: #333; font-size: 16px;">Use code:</p>
                                <p style="margin: 10px 0 0 0; font-size: 24px; font-weight: bold; color: #ff6b6b; letter-spacing: 2px;">{{promo_code}}</p>
                            </div>
                            <a href="#" style="display: inline-block; background: linear-gradient(135deg, #ff6b6b, #feca57); color: white; padding: 18px 40px; text-decoration: none; border-radius: 50px; font-weight: bold; font-size: 18px; box-shadow: 0 4px 15px rgba(255,107,107,0.4);">SHOP NOW</a>
                            <p style="color: #999; margin-top: 30px; font-size: 14px;">Offer expires on {{expiry_date}}</p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `🎉 SPECIAL OFFER!\n\nHi {{customer_name}}!\n\nGet {{discount_percent}}% OFF on {{product_name}}\n\nUse code: {{promo_code}}\n\nOffer expires on {{expiry_date}}`
  },
  {
    id: 'ecommerce-order',
    name: 'Order Confirmation',
    category: 'E-commerce',
    subject: 'Order Confirmation #{{order_number}}',
    description: 'Professional order confirmation email for e-commerce',
    variables: ['customer_name', 'order_number', 'order_total', 'shipping_address', 'estimated_delivery'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Order Confirmation</title>
</head>
<body style="margin: 0; padding: 0; font-family: 'Helvetica Neue', Arial, sans-serif; background-color: #f7f7f7;">
    <table width="100%" cellpadding="0" cellspacing="0" style="background-color: #f7f7f7; padding: 20px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: white; border-radius: 8px; overflow: hidden;">
                    <tr>
                        <td style="background: #28a745; padding: 30px; text-align: center;">
                            <h1 style="color: white; margin: 0; font-size: 24px;">✅ Order Confirmed!</h1>
                            <p style="color: #d4edda; margin: 10px 0 0 0;">Thank you for your purchase</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="color: #333; margin-bottom: 20px;">Hi {{customer_name}},</h2>
                            <p style="color: #666; line-height: 1.6; margin-bottom: 25px;">Your order has been confirmed and is being processed. Here are your order details:</p>
                            <table width="100%" style="border: 1px solid #e9ecef; border-radius: 5px; margin-bottom: 25px;">
                                <tr style="background: #f8f9fa;">
                                    <td style="padding: 15px; border-bottom: 1px solid #e9ecef; font-weight: bold;">Order Number:</td>
                                    <td style="padding: 15px; border-bottom: 1px solid #e9ecef;">#{{order_number}}</td>
                                </tr>
                                <tr>
                                    <td style="padding: 15px; border-bottom: 1px solid #e9ecef; font-weight: bold;">Total Amount:</td>
                                    <td style="padding: 15px; border-bottom: 1px solid #e9ecef; color: #28a745; font-weight: bold;">\${{order_total}}</td>
                                </tr>
                                <tr style="background: #f8f9fa;">
                                    <td style="padding: 15px; border-bottom: 1px solid #e9ecef; font-weight: bold;">Shipping Address:</td>
                                    <td style="padding: 15px; border-bottom: 1px solid #e9ecef;">{{shipping_address}}</td>
                                </tr>
                                <tr>
                                    <td style="padding: 15px; font-weight: bold;">Estimated Delivery:</td>
                                    <td style="padding: 15px;">{{estimated_delivery}}</td>
                                </tr>
                            </table>
                            <a href="#" style="display: inline-block; background: #28a745; color: white; padding: 15px 30px; text-decoration: none; border-radius: 5px; font-weight: bold;">Track Your Order</a>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: #f8f9fa; padding: 20px; text-align: center; color: #666; font-size: 14px;">
                            <p>Questions? Contact our support team anytime.</p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `✅ Order Confirmed!\n\nHi {{customer_name}},\n\nYour order has been confirmed and is being processed.\n\nOrder Number: #{{order_number}}\nTotal Amount: \${{order_total}}\nShipping Address: {{shipping_address}}\nEstimated Delivery: {{estimated_delivery}}\n\nQuestions? Contact our support team anytime.`
  },
  {
    id: 'event-invitation',
    name: 'Event Invitation',
    category: 'Event',
    subject: 'You\'re Invited: {{event_name}}',
    description: 'Elegant event invitation template',
    variables: ['recipient_name', 'event_name', 'event_date', 'event_time', 'event_location', 'rsvp_link'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Event Invitation</title>
</head>
<body style="margin: 0; padding: 0; font-family: 'Times New Roman', serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
    <table width="100%" cellpadding="0" cellspacing="0" style="padding: 40px 20px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: white; border-radius: 15px; overflow: hidden; box-shadow: 0 15px 35px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 50px; text-align: center; position: relative;">
                            <div style="background: rgba(255,255,255,0.1); border: 2px solid rgba(255,255,255,0.3); border-radius: 10px; padding: 30px; backdrop-filter: blur(10px);">
                                <h1 style="color: white; margin: 0; font-size: 28px; font-weight: normal; letter-spacing: 2px;">YOU'RE INVITED</h1>
                                <p style="color: rgba(255,255,255,0.9); margin: 15px 0 0 0; font-size: 16px; font-style: italic;">to a special event</p>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 50px; text-align: center;">
                            <h2 style="color: #333; margin-bottom: 10px; font-size: 24px; font-weight: normal;">Dear {{recipient_name}},</h2>
                            <h3 style="color: #667eea; margin-bottom: 30px; font-size: 32px; font-weight: bold;">{{event_name}}</h3>
                            <div style="background: #f8f9ff; border-left: 4px solid #667eea; padding: 25px; margin: 30px 0; text-align: left;">
                                <p style="margin: 0 0 10px 0; color: #333;"><strong>📅 Date:</strong> {{event_date}}</p>
                                <p style="margin: 0 0 10px 0; color: #333;"><strong>🕐 Time:</strong> {{event_time}}</p>
                                <p style="margin: 0; color: #333;"><strong>📍 Location:</strong> {{event_location}}</p>
                            </div>
                            <p style="color: #666; line-height: 1.6; margin-bottom: 35px;">We would be honored by your presence at this special occasion. Please join us for an unforgettable experience.</p>
                            <a href="{{rsvp_link}}" style="display: inline-block; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 18px 40px; text-decoration: none; border-radius: 50px; font-weight: bold; font-size: 16px; letter-spacing: 1px; box-shadow: 0 8px 25px rgba(102,126,234,0.3);">RSVP NOW</a>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: #f8f9ff; padding: 25px; text-align: center; color: #666; font-size: 14px;">
                            <p style="margin: 0;">We can't wait to celebrate with you!</p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `YOU'RE INVITED\n\nDear {{recipient_name}},\n\n{{event_name}}\n\nDate: {{event_date}}\nTime: {{event_time}}\nLocation: {{event_location}}\n\nWe would be honored by your presence at this special occasion. Please join us for an unforgettable experience.\n\nRSVP: {{rsvp_link}}\n\nWe can't wait to celebrate with you!`
  },
  {
    id: 'password-reset',
    name: 'Password Reset',
    category: 'Transactional',
    subject: 'Reset Your Password',
    description: 'Secure password reset email template',
    variables: ['user_name', 'reset_link', 'expiry_time', 'support_email'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Password Reset</title>
</head>
<body style="margin: 0; padding: 0; font-family: Arial, sans-serif; background-color: #f4f4f4;">
    <table width="100%" cellpadding="0" cellspacing="0" style="background-color: #f4f4f4; padding: 20px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: white; border-radius: 8px; overflow: hidden; border: 1px solid #e1e5e9;">
                    <tr>
                        <td style="background: #dc3545; padding: 30px; text-align: center;">
                            <h1 style="color: white; margin: 0; font-size: 24px;">🔒 Password Reset</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="color: #333; margin-bottom: 20px;">Hi {{user_name}},</h2>
                            <p style="color: #666; line-height: 1.6; margin-bottom: 25px;">We received a request to reset your password. If you didn't make this request, you can safely ignore this email.</p>
                            <div style="background: #fff3cd; border: 1px solid #ffeaa7; border-radius: 5px; padding: 20px; margin: 25px 0;">
                                <p style="margin: 0; color: #856404; font-size: 14px;">⚠️ <strong>Security Notice:</strong> This link will expire in {{expiry_time}} for your security.</p>
                            </div>
                            <div style="text-align: center; margin: 35px 0;">
                                <a href="{{reset_link}}" style="display: inline-block; background: #dc3545; color: white; padding: 15px 30px; text-decoration: none; border-radius: 5px; font-weight: bold; font-size: 16px;">Reset Password</a>
                            </div>
                            <p style="color: #666; line-height: 1.6; margin-bottom: 20px;">If the button doesn't work, copy and paste this link into your browser:</p>
                            <p style="background: #f8f9fa; padding: 15px; border-radius: 5px; word-break: break-all; color: #495057; font-family: monospace; font-size: 14px;">{{reset_link}}</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: #f8f9fa; padding: 20px; text-align: center; color: #666; font-size: 14px; border-top: 1px solid #e9ecef;">
                            <p style="margin: 0;">Need help? Contact us at <a href="mailto:{{support_email}}" style="color: #dc3545;">{{support_email}}</a></p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `🔒 Password Reset\n\nHi {{user_name}},\n\nWe received a request to reset your password. If you didn't make this request, you can safely ignore this email.\n\n⚠️ Security Notice: This link will expire in {{expiry_time}} for your security.\n\nReset your password: {{reset_link}}\n\nNeed help? Contact us at {{support_email}}`
  },
  {
    id: 'birthday-wishes',
    name: 'Birthday Wishes',
    category: 'Personal',
    subject: '🎉 Happy Birthday {{recipient_name}}!',
    description: 'Cheerful birthday celebration template',
    variables: ['recipient_name', 'age', 'gift_message', 'sender_name'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Happy Birthday</title>
</head>
<body style="margin: 0; padding: 0; font-family: 'Comic Sans MS', cursive, Arial, sans-serif; background: linear-gradient(45deg, #ff6b6b, #4ecdc4, #45b7d1, #96ceb4, #ffeaa7);">
    <table width="100%" cellpadding="0" cellspacing="0" style="padding: 30px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: white; border-radius: 20px; overflow: hidden; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 40px; text-align: center; position: relative;">
                            <div style="font-size: 60px; margin-bottom: 10px;">🎉🎂🎈</div>
                            <h1 style="color: white; margin: 0; font-size: 36px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);">HAPPY BIRTHDAY!</h1>
                            <p style="color: rgba(255,255,255,0.9); margin: 10px 0 0 0; font-size: 18px;">It's time to celebrate!</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 50px; text-align: center;">
                            <h2 style="color: #333; margin-bottom: 20px; font-size: 28px;">Dear {{recipient_name}},</h2>
                            <div style="background: linear-gradient(135deg, #ffeaa7 0%, #fab1a0 100%); border-radius: 15px; padding: 30px; margin: 30px 0;">
                                <h3 style="color: #2d3436; margin: 0 0 15px 0; font-size: 24px;">🎊 Turning {{age}} looks amazing on you! 🎊</h3>
                                <p style="color: #636e72; line-height: 1.6; margin: 0;">{{gift_message}}</p>
                            </div>
                            <p style="color: #666; line-height: 1.8; font-size: 16px; margin: 30px 0;">Wishing you a day filled with happiness, laughter, and all your favorite things. May this new year of life bring you endless joy, exciting adventures, and dreams come true!</p>
                            <div style="background: #f8f9ff; border-radius: 10px; padding: 25px; margin: 30px 0;">
                                <p style="color: #667eea; font-size: 18px; margin: 0; font-weight: bold;">🌟 Hope your special day is absolutely wonderful! 🌟</p>
                            </div>
                            <p style="color: #333; font-size: 18px; margin-top: 40px;">With love and best wishes,<br><strong>{{sender_name}}</strong></p>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: linear-gradient(135deg, #ffeaa7 0%, #fab1a0 100%); padding: 20px; text-align: center;">
                            <p style="margin: 0; color: #2d3436; font-size: 16px; font-weight: bold;">🎈 Let the celebrations begin! 🎈</p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `🎉 HAPPY BIRTHDAY! 🎂\n\nDear {{recipient_name}},\n\n🎊 Turning {{age}} looks amazing on you! 🎊\n\n{{gift_message}}\n\nWishing you a day filled with happiness, laughter, and all your favorite things. May this new year of life bring you endless joy, exciting adventures, and dreams come true!\n\n🌟 Hope your special day is absolutely wonderful! 🌟\n\nWith love and best wishes,\n{{sender_name}}\n\n🎈 Let the celebrations begin! 🎈`
  },
  {
    id: 'survey-feedback',
    name: 'Survey & Feedback',
    category: 'Survey',
    subject: 'We\'d love your feedback!',
    description: 'Professional survey request template',
    variables: ['customer_name', 'survey_link', 'incentive', 'company_name'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Survey Request</title>
</head>
<body style="margin: 0; padding: 0; font-family: Arial, sans-serif; background-color: #f5f7fa;">
    <table width="100%" cellpadding="0" cellspacing="0" style="background-color: #f5f7fa; padding: 30px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: white; border-radius: 10px; overflow: hidden; box-shadow: 0 5px 15px rgba(0,0,0,0.08);">
                    <tr>
                        <td style="background: #4a90e2; padding: 40px; text-align: center;">
                            <div style="font-size: 48px; margin-bottom: 15px;">💭</div>
                            <h1 style="color: white; margin: 0; font-size: 28px;">Your Opinion Matters</h1>
                            <p style="color: rgba(255,255,255,0.9); margin: 10px 0 0 0; font-size: 16px;">Help us serve you better</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="color: #333; margin-bottom: 20px; font-size: 24px;">Hi {{customer_name}},</h2>
                            <p style="color: #666; line-height: 1.6; margin-bottom: 25px; font-size: 16px;">We hope you've been enjoying our services! Your feedback is incredibly valuable to us and helps us improve our offerings.</p>
                            <div style="background: #e8f4fd; border-left: 4px solid #4a90e2; padding: 20px; margin: 25px 0;">
                                <p style="margin: 0; color: #2c5282; font-size: 16px;">📊 <strong>Quick Survey:</strong> Takes only 2-3 minutes to complete</p>
                            </div>
                            <p style="color: #666; line-height: 1.6; margin-bottom: 30px;">We'd love to hear about your experience and any suggestions you might have. Your insights help us create better products and services for you and our community.</p>
                            <div style="text-align: center; margin: 35px 0;">
                                <a href="{{survey_link}}" style="display: inline-block; background: #4a90e2; color: white; padding: 18px 35px; text-decoration: none; border-radius: 8px; font-weight: bold; font-size: 16px; box-shadow: 0 4px 12px rgba(74,144,226,0.3);">Take Survey</a>
                            </div>
                            <div style="background: #f0f9ff; border: 1px solid #bae6fd; border-radius: 8px; padding: 20px; margin: 25px 0; text-align: center;">
                                <p style="margin: 0; color: #0369a1; font-size: 14px;">🎁 <strong>Thank you gift:</strong> {{incentive}}</p>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: #f8fafc; padding: 25px; text-align: center; color: #666; font-size: 14px; border-top: 1px solid #e2e8f0;">
                            <p style="margin: 0;">Thank you for being a valued customer!<br><strong>{{company_name}} Team</strong></p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `💭 Your Opinion Matters\n\nHi {{customer_name}},\n\nWe hope you've been enjoying our services! Your feedback is incredibly valuable to us and helps us improve our offerings.\n\n📊 Quick Survey: Takes only 2-3 minutes to complete\n\nWe'd love to hear about your experience and any suggestions you might have. Your insights help us create better products and services for you and our community.\n\nTake Survey: {{survey_link}}\n\n🎁 Thank you gift: {{incentive}}\n\nThank you for being a valued customer!\n{{company_name}} Team`
  },
  {
    id: 'holiday-greetings',
    name: 'Holiday Greetings',
    category: 'Holiday',
    subject: '🎄 Season\'s Greetings from {{company_name}}',
    description: 'Festive holiday wishes template',
    variables: ['recipient_name', 'company_name', 'holiday_message', 'year'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Holiday Greetings</title>
</head>
<body style="margin: 0; padding: 0; font-family: Georgia, serif; background: linear-gradient(135deg, #2c5530 0%, #1a3d1f 100%);">
    <table width="100%" cellpadding="0" cellspacing="0" style="padding: 30px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: white; border-radius: 15px; overflow: hidden; box-shadow: 0 15px 30px rgba(0,0,0,0.2);">
                    <tr>
                        <td style="background: linear-gradient(135deg, #c41e3a 0%, #2c5530 100%); padding: 50px; text-align: center; position: relative;">
                            <div style="font-size: 50px; margin-bottom: 15px;">🎄✨🎁</div>
                            <h1 style="color: white; margin: 0; font-size: 32px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);">Season's Greetings</h1>
                            <p style="color: rgba(255,255,255,0.9); margin: 10px 0 0 0; font-size: 16px; font-style: italic;">Wishing you joy and happiness</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 50px; text-align: center;">
                            <h2 style="color: #2c5530; margin-bottom: 25px; font-size: 26px;">Dear {{recipient_name}},</h2>
                            <div style="background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%); border-radius: 10px; padding: 30px; margin: 30px 0; border: 2px solid #c41e3a;">
                                <p style="color: #2c5530; line-height: 1.8; font-size: 18px; margin: 0; font-style: italic;">{{holiday_message}}</p>
                            </div>
                            <p style="color: #666; line-height: 1.6; font-size: 16px; margin: 30px 0;">As we reflect on the year {{year}}, we're filled with gratitude for wonderful people like you. May this holiday season bring you peace, joy, and precious moments with loved ones.</p>
                            <div style="background: #fff3cd; border: 2px solid #ffc107; border-radius: 10px; padding: 25px; margin: 30px 0;">
                                <p style="color: #856404; font-size: 16px; margin: 0; font-weight: bold;">🌟 Wishing you a wonderful holiday season and a bright New Year! 🌟</p>
                            </div>
                            <p style="color: #2c5530; font-size: 18px; margin-top: 40px; font-weight: bold;">Warm regards,<br>The {{company_name}} Team</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: linear-gradient(135deg, #c41e3a 0%, #2c5530 100%); padding: 20px; text-align: center;">
                            <p style="margin: 0; color: white; font-size: 14px;">🎊 May your holidays sparkle with joy and laughter! 🎊</p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `🎄 Season's Greetings ✨\n\nDear {{recipient_name}},\n\n{{holiday_message}}\n\nAs we reflect on the year {{year}}, we're filled with gratitude for wonderful people like you. May this holiday season bring you peace, joy, and precious moments with loved ones.\n\n🌟 Wishing you a wonderful holiday season and a bright New Year! 🌟\n\nWarm regards,\nThe {{company_name}} Team\n\n🎊 May your holidays sparkle with joy and laughter! 🎊`
  },
  {
    id: 'appointment-reminder',
    name: 'Appointment Reminder',
    category: 'Reminder',
    subject: 'Reminder: Your appointment on {{appointment_date}}',
    description: 'Professional appointment reminder template',
    variables: ['client_name', 'appointment_date', 'appointment_time', 'location', 'service_type', 'contact_info'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Appointment Reminder</title>
</head>
<body style="margin: 0; padding: 0; font-family: Arial, sans-serif; background-color: #f0f2f5;">
    <table width="100%" cellpadding="0" cellspacing="0" style="background-color: #f0f2f5; padding: 30px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: white; border-radius: 8px; overflow: hidden; border: 1px solid #d1d5db;">
                    <tr>
                        <td style="background: #059669; padding: 30px; text-align: center;">
                            <div style="font-size: 40px; margin-bottom: 10px;">📅</div>
                            <h1 style="color: white; margin: 0; font-size: 24px;">Appointment Reminder</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="color: #333; margin-bottom: 20px; font-size: 22px;">Hello {{client_name}},</h2>
                            <p style="color: #666; line-height: 1.6; margin-bottom: 25px;">This is a friendly reminder about your upcoming appointment with us.</p>
                            <div style="background: #ecfdf5; border: 1px solid #a7f3d0; border-radius: 8px; padding: 25px; margin: 25px 0;">
                                <h3 style="color: #059669; margin: 0 0 15px 0; font-size: 18px;">📋 Appointment Details</h3>
                                <table style="width: 100%; border-collapse: collapse;">
                                    <tr>
                                        <td style="padding: 8px 0; color: #374151; font-weight: bold; width: 30%;">Service:</td>
                                        <td style="padding: 8px 0; color: #374151;">{{service_type}}</td>
                                    </tr>
                                    <tr>
                                        <td style="padding: 8px 0; color: #374151; font-weight: bold;">Date:</td>
                                        <td style="padding: 8px 0; color: #374151;">{{appointment_date}}</td>
                                    </tr>
                                    <tr>
                                        <td style="padding: 8px 0; color: #374151; font-weight: bold;">Time:</td>
                                        <td style="padding: 8px 0; color: #374151;">{{appointment_time}}</td>
                                    </tr>
                                    <tr>
                                        <td style="padding: 8px 0; color: #374151; font-weight: bold;">Location:</td>
                                        <td style="padding: 8px 0; color: #374151;">{{location}}</td>
                                    </tr>
                                </table>
                            </div>
                            <div style="background: #fef3c7; border: 1px solid #fbbf24; border-radius: 8px; padding: 20px; margin: 25px 0;">
                                <p style="margin: 0; color: #92400e; font-size: 14px;">⏰ <strong>Please arrive 10 minutes early</strong> to complete any necessary paperwork.</p>
                            </div>
                            <p style="color: #666; line-height: 1.6; margin-bottom: 25px;">If you need to reschedule or cancel your appointment, please contact us as soon as possible.</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: #f9fafb; padding: 25px; text-align: center; color: #666; font-size: 14px; border-top: 1px solid #e5e7eb;">
                            <p style="margin: 0;">Questions? Contact us at {{contact_info}}<br>We look forward to seeing you!</p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `📅 Appointment Reminder\n\nHello {{client_name}},\n\nThis is a friendly reminder about your upcoming appointment with us.\n\n📋 Appointment Details:\nService: {{service_type}}\nDate: {{appointment_date}}\nTime: {{appointment_time}}\nLocation: {{location}}\n\n⏰ Please arrive 10 minutes early to complete any necessary paperwork.\n\nIf you need to reschedule or cancel your appointment, please contact us as soon as possible.\n\nQuestions? Contact us at {{contact_info}}\nWe look forward to seeing you!`
  },
  {
    id: 'thank-you-note',
    name: 'Thank You Note',
    category: 'Personal',
    subject: 'Thank you, {{recipient_name}}!',
    description: 'Heartfelt thank you message template',
    variables: ['recipient_name', 'reason', 'personal_message', 'sender_name'],
    html_content: `
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Thank You</title>
</head>
<body style="margin: 0; padding: 0; font-family: 'Georgia', serif; background: linear-gradient(135deg, #ffecd2 0%, #fcb69f 100%);">
    <table width="100%" cellpadding="0" cellspacing="0" style="padding: 40px;">
        <tr>
            <td align="center">
                <table width="600" cellpadding="0" cellspacing="0" style="background: white; border-radius: 15px; overflow: hidden; box-shadow: 0 10px 25px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="background: linear-gradient(135deg, #ff9a9e 0%, #fecfef 100%); padding: 50px; text-align: center;">
                            <div style="font-size: 60px; margin-bottom: 15px;">🙏</div>
                            <h1 style="color: #8b5a3c; margin: 0; font-size: 36px; text-shadow: 1px 1px 2px rgba(0,0,0,0.1);">Thank You</h1>
                            <p style="color: #a0522d; margin: 10px 0 0 0; font-size: 18px; font-style: italic;">From the bottom of our hearts</p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 50px; text-align: center;">
                            <h2 style="color: #8b5a3c; margin-bottom: 25px; font-size: 28px;">Dear {{recipient_name}},</h2>
                            <div style="background: linear-gradient(135deg, #fff5ee 0%, #ffeee6 100%); border-radius: 12px; padding: 35px; margin: 30px 0; border: 2px solid #ff9a9e;">
                                <h3 style="color: #8b5a3c; margin: 0 0 20px 0; font-size: 22px;">💝 {{reason}}</h3>
                                <p style="color: #a0522d; line-height: 1.8; font-size: 16px; margin: 0; font-style: italic;">{{personal_message}}</p>
                            </div>
                            <p style="color: #666; line-height: 1.8; font-size: 16px; margin: 30px 0;">Your kindness and thoughtfulness mean more than words can express. Thank you for being such a wonderful person and for all that you do.</p>
                            <div style="background: #f0f9ff; border-radius: 10px; padding: 25px; margin: 30px 0; border-left: 4px solid #ff9a9e;">
                                <p style="color: #8b5a3c; font-size: 18px; margin: 0; font-weight: bold;">✨ You truly make a difference! ✨</p>
                            </div>
                            <p style="color: #8b5a3c; font-size: 20px; margin-top: 40px;">With heartfelt gratitude,<br><strong>{{sender_name}}</strong></p>
                        </td>
                    </tr>
                    <tr>
                        <td style="background: linear-gradient(135deg, #ff9a9e 0%, #fecfef 100%); padding: 25px; text-align: center;">
                            <p style="margin: 0; color: #8b5a3c; font-size: 16px; font-weight: bold;">🌸 Grateful today and always 🌸</p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>`,
    text_content: `🙏 Thank You\n\nDear {{recipient_name}},\n\n💝 {{reason}}\n\n{{personal_message}}\n\nYour kindness and thoughtfulness mean more than words can express. Thank you for being such a wonderful person and for all that you do.\n\n✨ You truly make a difference! ✨\n\nWith heartfelt gratitude,\n{{sender_name}}\n\n🌸 Grateful today and always 🌸`
  }
];

// Helper functions
export const getPremadeTemplates = (): PremadeTemplate[] => {
  return PREMADE_TEMPLATES;
};

export const getTemplatesByCategory = (category: string): PremadeTemplate[] => {
  return PREMADE_TEMPLATES.filter(template => template.category === category);
};

export const getTemplateById = (id: string): PremadeTemplate | undefined => {
  return PREMADE_TEMPLATES.find(template => template.id === id);
};

export const getAllCategories = (): string[] => {
  return TEMPLATE_CATEGORIES;
};

export const replaceVariables = (content: string, variables: Record<string, string>): string => {
  let result = content;
  Object.entries(variables).forEach(([key, value]) => {
    const regex = new RegExp(`{{${key}}}`, 'g');
    result = result.replace(regex, value);
  });
  return result;
};

export const getTemplatePreview = (template: PremadeTemplate, variables: Record<string, string> = {}): { html: string; text: string; subject: string } => {
  // Use placeholder values for missing variables
  const defaultVariables: Record<string, string> = {};
  template.variables.forEach(variable => {
    defaultVariables[variable] = variables[variable] || `[${variable}]`;
  });

  return {
    html: replaceVariables(template.html_content, defaultVariables),
    text: replaceVariables(template.text_content, defaultVariables),
    subject: replaceVariables(template.subject, defaultVariables)
  };
};

// Exports are already declared above with individual export statements