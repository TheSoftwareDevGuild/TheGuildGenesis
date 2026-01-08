# LinkedIn Account Backend Implementation

This document describes the implementation of LinkedIn account support in the backend, addressing issue #163.

## Overview

This implementation adds LinkedIn account functionality to user profiles, following the same pattern as the existing GitHub login feature. Users can now add their LinkedIn account to their profile with global, case-insensitive uniqueness enforcement (first-come-first-serve).

## Changes Made

### 1. Database Migration (`backend/migrations/004_add_linkedin_account.sql`)

```sql
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS linkedin_account TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS unique_linkedin_account_lower ON profiles (LOWER(linkedin_account));
```

**Features:**
- Adds `linkedin_account` column to profiles table
- Creates unique case-insensitive index for global uniqueness
- Follows same pattern as `github_login` implementation

### 2. Domain Entity (`backend/src/domain/entities/profile.rs`)

**Changes:**
- Added `linkedin_account: Option<String>` field to `Profile` struct
- Initialized to `None` in `Profile::new()`
- Serializable for API responses

### 3. Repository Trait (`backend/src/domain/repositories/profile_repository.rs`)

**New Method:**
```rust
async fn find_by_linkedin_account(
    &self,
    linkedin_account: &str,
) -> Result<Option<Profile>, Box<dyn std::error::Error + Send + Sync>>;
```

**Purpose:**
- Enables case-insensitive lookup by LinkedIn account
- Used for uniqueness validation during profile updates

### 4. Repository Implementation (`backend/src/infrastructure/repositories/postgres_profile_repository.rs`)

**Changes:**
- Added `linkedin_account` to all SELECT queries
- Included `linkedin_account` in INSERT and UPDATE operations
- Implemented `find_by_linkedin_account` with case-insensitive lookup using `LOWER()`

**Example Query:**
```rust
SELECT address, name, description, avatar_url, github_login, linkedin_account, created_at, updated_at
FROM profiles
WHERE LOWER(linkedin_account) = LOWER($1)
```

### 5. DTOs (`backend/src/application/dtos/profile_dtos.rs`)

**UpdateProfileRequest:**
```rust
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub github_login: Option<String>,
    pub linkedin_account: Option<String>,  // NEW
}
```

**ProfileResponse:**
```rust
pub struct ProfileResponse {
    pub address: WalletAddress,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub github_login: Option<String>,
    pub linkedin_account: Option<String>,  // NEW
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

### 6. Update Profile Command (`backend/src/application/commands/update_profile.rs`)

**Validation Logic:**
```rust
if let Some(ref account) = request.linkedin_account {
    let trimmed = account.trim();

    if trimmed.is_empty() {
        profile.linkedin_account = None;
    } else {
        // Validate format (3-100 chars, alphanumeric + hyphens)
        let valid_format = regex::Regex::new(r"^[a-zA-Z0-9-]{3,100}$").unwrap();
        if !valid_format.is_match(trimmed) {
            return Err("Invalid LinkedIn account format".to_string());
        }
        
        // Check uniqueness
        if let Some(conflicting_profile) = profile_repository
            .find_by_linkedin_account(trimmed)
            .await
            .map_err(|e| e.to_string())?
        {
            if conflicting_profile.address != wallet_address {
                return Err("LinkedIn account already taken".to_string());
            }
        }
        
        profile.linkedin_account = Some(trimmed.to_string());
    }
}
```

**Features:**
- Format validation: 3-100 characters, alphanumeric and hyphens
- Case-insensitive uniqueness check
- Allows empty values (sets to `None`)
- Prevents conflicts with existing LinkedIn accounts
- Returns appropriate error messages

### 7. Query Functions

**get_profile.rs:**
- Added `linkedin_account` to `ProfileResponse`

**get_all_profiles.rs:**
- Added `linkedin_account` to `ProfileResponse` mapping

## API Usage

### Update Profile with LinkedIn Account

**Endpoint:** `PUT /api/profiles`

**Request Body:**
```json
{
  "name": "John Doe",
  "description": "Software Developer",
  "avatar_url": "https://example.com/avatar.jpg",
  "github_login": "johndoe",
  "linkedin_account": "john-doe-123"
}
```

**Success Response (200 OK):**
```json
{
  "address": "0x1234567890abcdef",
  "name": "John Doe",
  "description": "Software Developer",
  "avatar_url": "https://example.com/avatar.jpg",
  "github_login": "johndoe",
  "linkedin_account": "john-doe-123",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-02T00:00:00Z"
}
```

**Error Responses:**

**409 Conflict** - LinkedIn account already taken:
```json
{
  "error": "LinkedIn account already taken"
}
```

**400 Bad Request** - Invalid format:
```json
{
  "error": "Invalid LinkedIn account format"
}
```

### Get Profile

**Endpoint:** `GET /api/profiles/:address`

**Response:**
```json
{
  "address": "0x1234567890abcdef",
  "name": "John Doe",
  "description": "Software Developer",
  "avatar_url": "https://example.com/avatar.jpg",
  "github_login": "johndoe",
  "linkedin_account": "john-doe-123",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-02T00:00:00Z"
}
```

### Get All Profiles

**Endpoint:** `GET /api/profiles`

**Response:**
```json
[
  {
    "address": "0x1234567890abcdef",
    "name": "John Doe",
    "linkedin_account": "john-doe-123",
    ...
  },
  {
    "address": "0xfedcba0987654321",
    "name": "Jane Smith",
    "linkedin_account": "jane-smith-456",
    ...
  }
]
```

## Validation Rules

### LinkedIn Account Format

- **Length:** 3-100 characters
- **Allowed Characters:** Alphanumeric (a-z, A-Z, 0-9) and hyphens (-)
- **Regex Pattern:** `^[a-zA-Z0-9-]{3,100}$`

### Examples

✅ **Valid:**
- `john-doe`
- `jane-smith-123`
- `developer-2024`
- `abc`

❌ **Invalid:**
- `ab` (too short, minimum 3 characters)
- `john_doe` (underscores not allowed)
- `john.doe` (dots not allowed)
- `john doe` (spaces not allowed)
- `john@doe` (special characters not allowed)

### Uniqueness

- **Case-Insensitive:** `john-doe` and `JOHN-DOE` are considered the same
- **Global:** LinkedIn accounts are unique across all profiles
- **First-Come-First-Serve:** The first user to claim a LinkedIn account owns it
- **Conflict Prevention:** Users cannot take LinkedIn accounts already claimed by others
- **Self-Update Allowed:** Users can update their own LinkedIn account without conflict

### Empty Values

- Sending an empty string (`""`) or whitespace-only string sets `linkedin_account` to `NULL`
- Omitting the field in the request leaves the existing value unchanged

## Database Schema

### profiles Table

```sql
CREATE TABLE profiles (
    address VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    description TEXT,
    avatar_url TEXT,
    github_login TEXT,
    linkedin_account TEXT,  -- NEW
    login_nonce BIGINT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Existing index
CREATE UNIQUE INDEX unique_github_login_lower ON profiles (LOWER(github_login));

-- New index
CREATE UNIQUE INDEX unique_linkedin_account_lower ON profiles (LOWER(linkedin_account));
```

## Testing

### Manual Testing Steps

1. **Add LinkedIn Account:**
   ```bash
   curl -X PUT http://localhost:8080/api/profiles \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer <token>" \
     -d '{"linkedin_account": "john-doe-123"}'
   ```

2. **Verify Uniqueness:**
   ```bash
   # Try to claim the same LinkedIn account with different wallet
   # Should return 409 Conflict
   ```

3. **Test Case-Insensitivity:**
   ```bash
   # Try "JOHN-DOE-123" after claiming "john-doe-123"
   # Should return 409 Conflict
   ```

4. **Test Format Validation:**
   ```bash
   # Try invalid formats
   curl -X PUT http://localhost:8080/api/profiles \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer <token>" \
     -d '{"linkedin_account": "ab"}'  # Too short
   # Should return 400 Bad Request
   ```

5. **Test Empty Value:**
   ```bash
   curl -X PUT http://localhost:8080/api/profiles \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer <token>" \
     -d '{"linkedin_account": ""}'
   # Should set linkedin_account to NULL
   ```

6. **Verify in Database:**
   ```sql
   SELECT address, name, github_login, linkedin_account 
   FROM profiles 
   WHERE linkedin_account IS NOT NULL;
   ```

## Migration Instructions

### Running the Migration

```bash
# From backend directory
sqlx migrate run
```

### Rollback (if needed)

```bash
# Create rollback migration
cat > migrations/005_rollback_linkedin.sql << EOF
DROP INDEX IF EXISTS unique_linkedin_account_lower;
ALTER TABLE profiles DROP COLUMN IF EXISTS linkedin_account;
EOF

sqlx migrate run
```

## Consistency with GitHub Login

This implementation follows the exact same pattern as the GitHub login feature:

| Feature | GitHub Login | LinkedIn Account |
|---------|-------------|------------------|
| Database Column | `github_login` | `linkedin_account` |
| Unique Index | `unique_github_login_lower` | `unique_linkedin_account_lower` |
| Repository Method | `find_by_github_login` | `find_by_linkedin_account` |
| Validation Regex | `^[a-zA-Z0-9-]{1,39}$` | `^[a-zA-Z0-9-]{3,100}$` |
| Case Sensitivity | Case-insensitive | Case-insensitive |
| Empty Handling | Sets to NULL | Sets to NULL |
| Error Message | "GitHub handle already taken" | "LinkedIn account already taken" |

## Future Enhancements

Potential improvements for future PRs:

1. **LinkedIn Profile Verification:**
   - OAuth integration to verify LinkedIn account ownership
   - Display verified badge on profiles

2. **LinkedIn Profile Data:**
   - Fetch and display LinkedIn profile information
   - Show professional headline, company, etc.

3. **Search by LinkedIn:**
   - Add API endpoint to search profiles by LinkedIn account
   - Enable profile discovery via LinkedIn

4. **LinkedIn Profile Links:**
   - Generate clickable LinkedIn profile URLs
   - Format: `https://linkedin.com/in/{linkedin_account}`

5. **Batch Operations:**
   - Import LinkedIn accounts from CSV
   - Bulk validation and assignment

## Related Issues

- Fixes #163 (Add linkedin link to profile: BE)
- Related to #162 (Add linkedin account to user profiles - Full Stack)

## Notes

- All changes are backward compatible
- Existing profiles will have `linkedin_account` set to `NULL`
- No breaking changes to existing API endpoints
- Frontend integration required separately (issue #162)
