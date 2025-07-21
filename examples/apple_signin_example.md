# Apple Sign-In Integration Example

This example demonstrates how to use the Apple Sign-In integration in the NerdNuggets REST API.

## Frontend Implementation

### Web Application (JavaScript)

```html
<!DOCTYPE html>
<html>
<head>
    <title>Apple Sign-In Example</title>
    <script type="text/javascript" src="https://appleid.cdn-apple.com/appleauth/static/jsapi/appleid/1/en_US/appleid.auth.js"></script>
</head>
<body>
    <div id="appleid-signin" data-color="black" data-border="true" data-type="sign in"></div>
    
    <script type="text/javascript">
        AppleID.auth.init({
            clientId: 'your.app.bundle.id',
            scope: 'name email',
            redirectURI: 'https://your-domain.com/callback',
            state: 'origin:web'
        });
        
        document.addEventListener('AppleIDSignInOnSuccess', (event) => {
            // Send the authorization code to your backend
            const { authorization } = event.detail;
            
            fetch('/auth/apple', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    authorization_code: authorization.code
                })
            })
            .then(response => response.json())
            .then(data => {
                console.log('Login successful:', data);
                // Store the JWT token
                localStorage.setItem('token', data.token);
                // Redirect or update UI
                window.location.href = '/dashboard';
            })
            .catch(error => {
                console.error('Login failed:', error);
            });
        });
        
        document.addEventListener('AppleIDSignInOnFailure', (event) => {
            console.error('Apple Sign-In failed:', event.detail);
        });
    </script>
</body>
</html>
```

### iOS Application (Swift)

```swift
import AuthenticationServices
import UIKit

class ViewController: UIViewController {
    
    override func viewDidLoad() {
        super.viewDidLoad()
        setupAppleSignIn()
    }
    
    func setupAppleSignIn() {
        let button = ASAuthorizationAppleIDButton(type: .signIn, style: .black)
        button.addTarget(self, action: #selector(handleAppleSignIn), for: .touchUpInside)
        button.frame = CGRect(x: 50, y: 100, width: 200, height: 50)
        view.addSubview(button)
    }
    
    @objc func handleAppleSignIn() {
        let request = ASAuthorizationAppleIDProvider().createRequest()
        request.requestedScopes = [.fullName, .email]
        
        let controller = ASAuthorizationController(authorizationRequests: [request])
        controller.delegate = self
        controller.presentationContextProvider = self
        controller.performRequests()
    }
}

extension ViewController: ASAuthorizationControllerDelegate {
    func authorizationController(controller: ASAuthorizationController, didCompleteWithAuthorization authorization: ASAuthorization) {
        if let appleIDCredential = authorization.credential as? ASAuthorizationAppleIDCredential {
            // Get the authorization code
            guard let authorizationCode = appleIDCredential.authorizationCode,
                  let codeString = String(data: authorizationCode, encoding: .utf8) else {
                return
            }
            
            // Send to your backend
            sendToBackend(authorizationCode: codeString)
        }
    }
    
    func authorizationController(controller: ASAuthorizationController, didCompleteWithError error: Error) {
        print("Apple Sign-In failed: \(error)")
    }
    
    func sendToBackend(authorizationCode: String) {
        guard let url = URL(string: "https://your-api-domain.com/auth/apple") else { return }
        
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        
        let body = ["authorization_code": authorizationCode]
        request.httpBody = try? JSONSerialization.data(withJSONObject: body)
        
        URLSession.shared.dataTask(with: request) { data, response, error in
            if let data = data,
               let response = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
               let token = response["token"] as? String {
                // Store the token
                UserDefaults.standard.set(token, forKey: "auth_token")
                print("Login successful")
            }
        }.resume()
    }
}

extension ViewController: ASAuthorizationControllerPresentationContextProviding {
    func presentationAnchor(for controller: ASAuthorizationController) -> ASPresentationAnchor {
        return view.window!
    }
}
```

## Backend API Usage

### Request Example

```bash
curl -X POST https://your-api-domain.com/auth/apple \
  -H "Content-Type: application/json" \
  -d '{
    "authorization_code": "c1234567890abcdef..."
  }'
```

### Response Example

```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "name": "John Doe",
    "email": "john.doe@privaterelay.appleid.com",
    "roles": ["member"],
    "institution": "",
    "interests": [],
    "avatar_url": null,
    "bio": null,
    "tier": "bronze",
    "nerd_balance": 0,
    "wallet_address": null,
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-15T10:30:00Z"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

## Error Handling

### Common Error Responses

```json
{
  "error": "An error occurred while trying to retrieve user information.",
  "status": 500
}
```

```json
{
  "error": "Unable to create your account.",
  "status": 400
}
```

## Testing

### 1. Set up Environment Variables

```env
APPLE_CLIENT_ID=your.app.bundle.id
APPLE_TEAM_ID=ABC123DEF4
APPLE_KEY_ID=ABC123DEF4
APPLE_PRIVATE_KEY="-----BEGIN PRIVATE KEY-----
MIGTAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBHkwdwIBAQQg...
-----END PRIVATE KEY-----"
```

### 2. Run Database Migration

```bash
cargo run --bin migrate
```

### 3. Test the Integration

1. Use Apple's sandbox environment for testing
2. Create test Apple IDs in the Apple Developer Console
3. Test both new user registration and existing user login
4. Verify email linking works correctly

## Security Notes

1. **Token Verification**: In production, implement proper JWT signature verification using Apple's public keys
2. **HTTPS**: Always use HTTPS in production
3. **State Parameter**: Implement state parameter validation to prevent CSRF attacks
4. **Token Storage**: Store JWT tokens securely (httpOnly cookies for web, Keychain for iOS)
5. **Error Logging**: Log authentication errors for monitoring and debugging

## Troubleshooting

### Common Issues

1. **Invalid Client Secret**: Check your private key format and Key ID
2. **Team ID Mismatch**: Verify your Team ID in the Apple Developer Console
3. **App ID Issues**: Ensure "Sign In with Apple" capability is enabled
4. **Redirect URI**: Make sure your redirect URI matches exactly what's configured in Apple Developer Console

### Debug Steps

1. Check Apple Developer Console logs
2. Verify environment variables are set correctly
3. Test with Apple's sandbox environment first
4. Review authentication logs in your application 