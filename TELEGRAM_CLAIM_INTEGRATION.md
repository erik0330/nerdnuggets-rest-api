# Telegram Claim Integration Guide

This guide explains how to connect your Next.js frontend to the Telegram point claiming endpoints.

## Endpoints

The endpoints are available at:
- `POST /api/v2/verify-telegram` - Verify Telegram login and check points
- `POST /api/v2/claim` - Claim points by signing with a wallet

**Base URL**: Your backend URL (e.g., `http://localhost:8000` in development, or your production URL)

**Full URLs**:
- `{BASE_URL}/api/v2/verify-telegram`
- `{BASE_URL}/api/v2/claim`

## Frontend Integration

### 1. TypeScript Types

Create a types file for the API:

```typescript
// types/telegram.ts

export interface TelegramLoginData {
  id: number;
  first_name: string;
  last_name?: string;
  username?: string;
  photo_url?: string;
  auth_date: number;
  hash: string;
}

export interface VerifyTelegramResponse {
  success: boolean;
  points?: number;
  telegramId?: number;
  username?: string;
  error?: string;
}

export interface ClaimRequest {
  telegramId: number;
  walletAddress: string;
  message: string;
  signature: string;
}

export interface ClaimResponse {
  success: boolean;
  pointsClaimed?: number;
  wallet?: string;
  error?: string;
}
```

### 2. API Client Functions

Create an API client:

```typescript
// lib/api/telegram.ts

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8000';

export async function verifyTelegram(loginData: TelegramLoginData): Promise<VerifyTelegramResponse> {
  const response = await fetch(`${API_BASE_URL}/api/v2/verify-telegram`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(loginData),
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  return response.json();
}

export async function claimPoints(request: ClaimRequest): Promise<ClaimResponse> {
  const response = await fetch(`${API_BASE_URL}/api/v2/claim`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  return response.json();
}
```

### 3. Wallet Signing Helper (Ethers.js)

For signing messages with ethers.js:

```typescript
// lib/wallet/sign.ts

import { ethers } from 'ethers';

/**
 * Sign a message using the connected wallet
 * @param provider - ethers provider (e.g., from window.ethereum)
 * @param message - Message to sign
 * @returns Signature string (0x... format)
 */
export async function signMessage(
  provider: ethers.BrowserProvider,
  message: string
): Promise<string> {
  const signer = await provider.getSigner();
  const signature = await signer.signMessage(message);
  return signature;
}

/**
 * Get the connected wallet address
 * @param provider - ethers provider
 * @returns Wallet address
 */
export async function getWalletAddress(
  provider: ethers.BrowserProvider
): Promise<string> {
  const signer = await provider.getSigner();
  return await signer.getAddress();
}
```

### 4. React Component Example

Example React component for the claim flow:

```typescript
// components/TelegramClaim.tsx

'use client';

import { useState, useEffect } from 'react';
import { ethers } from 'ethers';
import { verifyTelegram, claimPoints } from '@/lib/api/telegram';
import { signMessage, getWalletAddress } from '@/lib/wallet/sign';

interface TelegramClaimProps {
  telegramData: TelegramLoginData; // From Telegram Login Widget
}

export default function TelegramClaim({ telegramData }: TelegramClaimProps) {
  const [points, setPoints] = useState<number | null>(null);
  const [telegramId, setTelegramId] = useState<number | null>(null);
  const [username, setUsername] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [claimed, setClaimed] = useState(false);

  // Step 1: Verify Telegram login when component mounts
  useEffect(() => {
    async function verify() {
      try {
        setLoading(true);
        const result = await verifyTelegram(telegramData);
        
        if (result.success && result.points && result.telegramId) {
          setPoints(result.points);
          setTelegramId(result.telegramId);
          setUsername(result.username || null);
          setError(null);
        } else {
          setError(result.error || 'Verification failed');
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to verify Telegram');
      } finally {
        setLoading(false);
      }
    }

    verify();
  }, [telegramData]);

  // Step 2: Claim points with wallet signature
  async function handleClaim() {
    if (!points || !telegramId) {
      setError('Please verify Telegram first');
      return;
    }

    try {
      setLoading(true);
      setError(null);

      // Check if wallet is connected
      if (!window.ethereum) {
        throw new Error('Please install MetaMask or another Web3 wallet');
      }

      // Connect to wallet
      const provider = new ethers.BrowserProvider(window.ethereum);
      await provider.send('eth_requestAccounts', []);

      // Get wallet address
      const walletAddress = await getWalletAddress(provider);

      // Create message to sign
      // Message must contain telegramId or username for verification
      const message = username 
        ? `Claim ${points} points for Telegram user @${username} (ID: ${telegramId})`
        : `Claim ${points} points for Telegram ID: ${telegramId}`;

      // Sign the message
      const signature = await signMessage(provider, message);

      // Call claim endpoint
      const result = await claimPoints({
        telegramId,
        walletAddress,
        message,
        signature,
      });

      if (result.success) {
        setClaimed(true);
        setError(null);
      } else {
        setError(result.error || 'Claim failed');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to claim points');
    } finally {
      setLoading(false);
    }
  }

  if (loading && !points) {
    return <div>Verifying Telegram login...</div>;
  }

  if (error && !points) {
    return (
      <div className="error">
        <p>Error: {error}</p>
      </div>
    );
  }

  if (claimed) {
    return (
      <div className="success">
        <p>✅ Successfully claimed {points} points!</p>
      </div>
    );
  }

  return (
    <div className="claim-container">
      <h2>Claim Your Points</h2>
      {username && <p>Telegram: @{username}</p>}
      <p>Points available: {points?.toLocaleString()}</p>
      
      {error && <div className="error">Error: {error}</div>}
      
      <button 
        onClick={handleClaim} 
        disabled={loading || !points}
      >
        {loading ? 'Processing...' : 'Claim with Wallet'}
      </button>
    </div>
  );
}
```

### 5. Telegram Login Widget Integration

If you're using the Telegram Login Widget, here's how to integrate it:

```tsx
// components/TelegramLoginWidget.tsx

'use client';

import { useEffect } from 'react';
import Script from 'next/script';

declare global {
  interface Window {
    onTelegramAuth?: (user: TelegramLoginData) => void;
  }
}

interface TelegramLoginWidgetProps {
  botName: string; // Your Telegram bot username
  onAuth: (user: TelegramLoginData) => void;
}

export default function TelegramLoginWidget({ 
  botName, 
  onAuth 
}: TelegramLoginWidgetProps) {
  useEffect(() => {
    // Set up the callback for Telegram auth
    window.onTelegramAuth = (user: TelegramLoginData) => {
      onAuth(user);
    };

    return () => {
      delete window.onTelegramAuth;
    };
  }, [onAuth]);

  return (
    <>
      <Script
        src="https://telegram.org/js/telegram-widget.js?22"
        strategy="lazyOnload"
        onLoad={() => {
          // Widget script loaded
        }}
      />
      <script
        async
        src={`https://telegram.org/js/telegram-widget.js?22`}
        data-telegram-login={botName}
        data-size="large"
        data-auth-url="/api/telegram-auth"
        data-request-access="write"
      />
      {/* Or use the div-based approach */}
      <div
        className="telegram-login-widget"
        data-telegram-login={botName}
        data-size="large"
        data-auth-url="/api/telegram-auth"
        data-request-access="write"
      />
    </>
  );
}
```

### 6. Complete Page Example

```tsx
// app/claim/page.tsx

'use client';

import { useState } from 'react';
import TelegramLoginWidget from '@/components/TelegramLoginWidget';
import TelegramClaim from '@/components/TelegramClaim';
import type { TelegramLoginData } from '@/types/telegram';

export default function ClaimPage() {
  const [telegramData, setTelegramData] = useState<TelegramLoginData | null>(null);

  function handleTelegramAuth(user: TelegramLoginData) {
    setTelegramData(user);
  }

  return (
    <div className="container">
      <h1>Claim Your Telegram Points</h1>
      
      {!telegramData ? (
        <div>
          <p>Please sign in with Telegram first:</p>
          <TelegramLoginWidget 
            botName="your_bot_username" 
            onAuth={handleTelegramAuth}
          />
        </div>
      ) : (
        <TelegramClaim telegramData={telegramData} />
      )}
    </div>
  );
}
```

## Environment Variables

Make sure to set these in your `.env.local`:

```env
NEXT_PUBLIC_API_URL=http://localhost:8000  # or your production API URL
```

## Error Handling

The endpoints return these error codes:
- `401` - Invalid Telegram login (verify-telegram)
- `404` - No points found (verify-telegram)
- `400` - Already claimed / Invalid request (both endpoints)

Always check the `success` field in the response and handle the `error` field appropriately.

## Security Notes

1. **Never trust frontend data** - The backend validates everything
2. **Telegram hash verification** - Done server-side for security
3. **Signature recovery** - Wallet signature is verified server-side
4. **Message validation** - Message must contain telegram ID or username
5. **One-time claim** - Points can only be claimed once per telegram ID

## Quick Start Example

Here's a minimal example to get started quickly:

```typescript
// Minimal example - app/claim/page.tsx
'use client';

import { useState } from 'react';
import { ethers } from 'ethers';

export default function ClaimPage() {
  const [status, setStatus] = useState('');

  async function handleTelegramLogin(user: any) {
    try {
      // Step 1: Verify Telegram
      const verifyRes = await fetch('http://localhost:8000/api/v2/verify-telegram', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(user),
      });
      const verifyData = await verifyRes.json();

      if (!verifyData.success) {
        setStatus(`Error: ${verifyData.error}`);
        return;
      }

      setStatus(`Found ${verifyData.points} points! Connecting wallet...`);

      // Step 2: Connect wallet and sign
      if (!window.ethereum) {
        setStatus('Please install MetaMask');
        return;
      }

      const provider = new ethers.BrowserProvider(window.ethereum);
      await provider.send('eth_requestAccounts', []);
      const signer = await provider.getSigner();
      const address = await signer.getAddress();

      // Create message (must contain telegram ID or username)
      const message = verifyData.username
        ? `Claim ${verifyData.points} points for @${verifyData.username}`
        : `Claim ${verifyData.points} points for Telegram ID: ${verifyData.telegramId}`;

      // Sign message
      const signature = await signer.signMessage(message);

      // Step 3: Claim
      const claimRes = await fetch('http://localhost:8000/api/v2/claim', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          telegramId: verifyData.telegramId,
          walletAddress: address,
          message,
          signature,
        }),
      });
      const claimData = await claimRes.json();

      if (claimData.success) {
        setStatus(`✅ Claimed ${claimData.pointsClaimed} points!`);
      } else {
        setStatus(`Error: ${claimData.error}`);
      }
    } catch (error) {
      setStatus(`Error: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  return (
    <div>
      <h1>Claim Points</h1>
      {/* Add Telegram Login Widget here */}
      <div>{status}</div>
    </div>
  );
}
```

## Testing

Test the endpoints with curl:

```bash
# Verify Telegram (replace with actual data from Telegram widget)
curl -X POST http://localhost:8000/api/v2/verify-telegram \
  -H "Content-Type: application/json" \
  -d '{
    "id": 123456789,
    "first_name": "John",
    "username": "johndoe",
    "auth_date": 1234567890,
    "hash": "abc123..."
  }'

# Claim points (after verification)
curl -X POST http://localhost:8000/api/v2/claim \
  -H "Content-Type: application/json" \
  -d '{
    "telegramId": 123456789,
    "walletAddress": "0x...",
    "message": "Claim 1000 points for Telegram ID: 123456789",
    "signature": "0x..."
  }'
```

