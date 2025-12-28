# Claim Deadline Configuration

## Overview

The Telegram claim system supports a deadline mechanism to end the claim period once the token launches live. This prevents new claims after a specified date/time.

## Configuration

### Environment Variable

Add the `TELEGRAM_CLAIM_DEADLINE` environment variable to control when claims end:

```env
# Unix timestamp (seconds since epoch) when claims should end
# If not set, claims are allowed indefinitely
TELEGRAM_CLAIM_DEADLINE=1735689600  # Example: January 1, 2025 00:00:00 UTC
```

### Setting the Deadline

1. **Get Unix timestamp for your deadline**:
   ```bash
   # Using date command (Unix/Linux/Mac)
   date -d "2025-01-01 00:00:00 UTC" +%s
   
   # Or use an online converter
   # https://www.epochconverter.com/
   ```

2. **Set the environment variable**:
   ```env
   TELEGRAM_CLAIM_DEADLINE=1735689600
   ```

3. **Redeploy the backend** with the new environment variable

## Behavior

- **If `TELEGRAM_CLAIM_DEADLINE` is not set**: Claims are allowed indefinitely
- **If `TELEGRAM_CLAIM_DEADLINE` is set**: 
  - Claims are allowed until the specified timestamp
  - After the deadline, both `/api/v2/verify-telegram` and `/api/v2/claim` will return:
    ```json
    {
      "success": false,
      "error": "Claim period has ended"
    }
  ```

## Example Timeline

1. **Before launch**: `TELEGRAM_CLAIM_DEADLINE` not set → Claims allowed
2. **Token launches live**: Set `TELEGRAM_CLAIM_DEADLINE` to a future date (e.g., 30 days after launch)
3. **After deadline**: All new claims are rejected

## Important Notes

- **Deadline is checked in both endpoints**: 
  - `/api/v2/verify-telegram` - Users won't be able to verify if deadline passed
  - `/api/v2/claim` - Claims will be rejected if deadline passed

- **Already claimed points**: Once a user has successfully claimed (marked in DB with `claimed_at`), they are not affected by the deadline

- **Database state**: The deadline check happens before any database or contract operations, so no state changes occur after the deadline

- **Changing the deadline**: You can extend the deadline by updating the environment variable and redeploying, but shortening it only affects future requests

## Security

- The deadline check happens early in both handlers, preventing unnecessary processing
- Deadline is read from environment variable at startup (not from database), making it harder to tamper with
- Changing the deadline requires a backend deployment, which provides audit trail

## Testing

To test the deadline mechanism:

1. Set a deadline in the near future:
   ```env
   TELEGRAM_CLAIM_DEADLINE=$(($(date +%s) + 3600))  # 1 hour from now
   ```

2. Test verification endpoint - should work before deadline
3. Wait for deadline to pass
4. Test again - should return "Claim period has ended" error

