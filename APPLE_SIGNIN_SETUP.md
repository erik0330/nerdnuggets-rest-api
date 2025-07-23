# Apple Sign-In Integration Setup Guide

This guide explains how to set up Apple Sign-In for the NerdNuggets REST API.

## Overview

Apple Sign-In has been integrated into the authentication system alongside existing email and Google OAuth methods. Users can now sign in or register using their Apple ID.

## Database Changes

A new migration has been added to support Apple Sign-In:

- **Migration**: `20250718000000_apple_id.up.sql`
- **Changes**: Adds `apple_id` column to the `users` table
- **Index**: Creates an index on `apple_id` for efficient lookups

## Environment Variables

Add the following environment variables to your `.env` file:

```env
# Apple Sign-In Configuration
APPLE_CLIENT_ID=your.app.bundle.id
APPLE_TEAM_ID=your_team_id
APPLE_KEY_ID=your_key_id
APPLE_PRIVATE_KEY="-----BEGIN PRIVATE KEY-----
your_private_key_content_here
-----END PRIVATE KEY-----"
```

### How to Get Apple Sign-In Credentials

1. **Apple Developer Account**: You need an Apple Developer account
2. **App ID**: Create an App ID in the Apple Developer Console
3. **Sign-In Capability**: Enable "Sign In with Apple" capability for your App ID
4. **Service ID**: Create a Service ID for web authentication
5. **Private Key**: Generate a private key in the Apple Developer Console

### Detailed Setup Steps

#### 1. Apple Developer Console Setup

1. Go to [Apple Developer Console](https://developer.apple.com/account/)
2. Navigate to "Certificates, Identifiers & Profiles"
3. Select "Identifiers" and create a new App ID
4. Enable "Sign In with Apple" capability
5. Create a Service ID for web authentication
6. Generate a private key for Sign In with Apple

#### 2. Private Key Generation

1. In Apple Developer Console, go to "Keys"
2. Click the "+" button to create a new key
3. Enable "Sign In with Apple"
4. Download the private key file
5. Note the Key ID (you'll need this for `APPLE_KEY_ID`)

#### 3. Team ID

Your Team ID can be found in the top-right corner of the Apple Developer Console.

## API Endpoints

### Apple Sign-In

**POST** `/auth/apple`

**Request Body:**
```json
{
  "authorization_code": "apple_authorization_code"
}
```

**Response:**
```json
{
  "user": {
    "id": "uuid",
    "username": "string",
    "name": "string",
    "email": "string",
    "roles": ["string"],
    "institution": "string",
    "interests": ["string"],
    "avatar_url": "string|null",
    "bio": "string|null",
    "tier": "string",
    "nerd_balance": 0,
    "wallet_address": "string|null",
    "created_at": "string",
    "updated_at": "string"
  },
  "token": "jwt_token"
}
```

## Implementation Details

### User Flow

1. **New User**: Creates account with Apple ID, email, and name
2. **Existing Apple User**: Logs in with existing Apple ID
3. **Email Merge**: If user exists with same email, Apple ID is linked to existing account
4. **Gmail Merge**: If user exists with Gmail, Apple ID is linked to existing account

### Database Schema

The `users` table now includes:
- `apple_id`: VARCHAR(255) - Apple's unique user identifier
- Index on `apple_id` for efficient lookups

### Code Structure

- **Apple OAuth Service**: `backend/libraries/third_party_api/src/apple_oauth.rs`
- **DTOs**: `backend/libraries/types/src/dto/user_dto.rs`
- **Models**: `backend/libraries/types/src/models/user.rs`
- **Repository**: `backend/database/src/repository/user_repository.rs`
- **Service**: `backend/database/src/service/user_service.rs`
- **Handler**: `backend/server/src/handler/auth_handler.rs`
- **Routes**: `backend/server/src/routes/auth.rs`

## Security Considerations

1. **JWT Verification**: Apple's identity tokens are verified using their public keys
2. **Client Secret**: Generated dynamically using your private key
3. **Token Expiration**: Client secrets expire after 6 months
4. **Email Privacy**: Apple may provide private relay emails

## Testing

### Local Development

1. Set up environment variables
2. Run database migrations
3. Test with Apple's sandbox environment
4. Use Apple's test accounts for development

### Production

1. Use production Apple Developer account
2. Configure proper redirect URLs
3. Test with real Apple IDs
4. Monitor authentication logs

## Troubleshooting

### Common Issues

1. **Invalid Client Secret**: Check private key format and Key ID
2. **Team ID Mismatch**: Verify Team ID in Apple Developer Console
3. **App ID Issues**: Ensure "Sign In with Apple" is enabled
4. **Token Verification**: Check JWT signature verification

### Error Handling

The API returns appropriate error messages for:
- Invalid authorization codes
- Apple API errors
- User creation failures
- Database connection issues

## Frontend Integration

### iOS/Swift

```swift
import AuthenticationServices

let request = ASAuthorizationAppleIDProvider().createRequest()
request.requestedScopes = [.fullName, .email]

let controller = ASAuthorizationController(authorizationRequests: [request])
controller.delegate = self
controller.presentationContextProvider = self
controller.performRequests()
```

### Web/JavaScript

```javascript
// Initialize Apple Sign-In
AppleID.auth.init({
  clientId: 'your.client.id',
  scope: 'name email',
  redirectURI: 'https://your-domain.com/callback',
  state: 'state'
});

// Sign in
AppleID.auth.signIn().then(response => {
  // Send authorization_code to your backend
  fetch('/auth/apple', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      authorization_code: response.authorization.code
    })
  });
});
```

## Migration Notes

- Existing users are not affected
- New Apple users get `verified_email: true` by default
- Apple ID linking preserves existing user data
- No breaking changes to existing authentication flows

## Support

For issues with Apple Sign-In integration:
1. Check Apple Developer documentation
2. Verify environment variables
3. Review authentication logs
4. Test with Apple's sandbox environment 