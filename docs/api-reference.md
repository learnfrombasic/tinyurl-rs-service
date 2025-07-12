# API Reference

## Base URL
```
http://localhost:8080
```

## Authentication
Currently, no authentication is required. Future versions may include API key authentication.

## Content Type
All requests and responses use `application/json` content type unless otherwise specified.

## Error Responses

All errors follow a consistent format:

```json
{
  "error": "Error type",
  "message": "Human-readable error message",
  "code": 400
}
```

### Common HTTP Status Codes
- `200` - Success
- `201` - Created
- `301` - Moved Permanently (redirects)
- `400` - Bad Request
- `404` - Not Found
- `409` - Conflict (custom code already exists)
- `500` - Internal Server Error

## Endpoints

### 1. Create Short URL

**POST** `/shorten`

Creates a new shortened URL from a long URL.

#### Request Body
```json
{
  "url": "https://www.example.com/very/long/url/path",
  "custom_code": "my-link"  // Optional
}
```

#### Parameters
- `url` (string, required): The long URL to shorten. Must be a valid HTTP/HTTPS URL.
- `custom_code` (string, optional): Custom short code (1-20 characters, alphanumeric and hyphens only).

#### Response (201 Created)
```json
{
  "short_url": "http://localhost:8080/abc123",
  "long_url": "https://www.example.com/very/long/url/path",
  "short_code": "abc123",
  "qr_code": null
}
```

#### Example
```bash
curl -X POST http://localhost:8080/shorten \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://github.com/MinLee0210/tinyurl-rs",
    "custom_code": "github-repo"
  }'
```

### 2. Redirect to Original URL

**GET** `/{short_code}`

Redirects to the original long URL and increments the click counter.

#### Parameters
- `short_code` (string, path): The short code to resolve.

#### Response (301 Moved Permanently)
- **Headers**: `Location: <original_url>`
- **Body**: Empty

#### Example
```bash
curl -I http://localhost:8080/abc123
# Returns: HTTP/1.1 301 Moved Permanently
# Location: https://www.example.com/very/long/url/path
```

### 3. Get URL Statistics

**GET** `/stats/{short_code}`

Retrieves statistics and information about a shortened URL.

#### Parameters
- `short_code` (string, path): The short code to get stats for.

#### Response (200 OK)
```json
{
  "short_code": "abc123",
  "long_url": "https://www.example.com/very/long/url/path",
  "clicks": 42,
  "created_at": "2023-12-01T10:30:00Z",
  "updated_at": "2023-12-01T15:45:30Z"
}
```

#### Fields
- `clicks`: Total number of times the short URL has been accessed
- `created_at`: ISO 8601 timestamp when the URL was created
- `updated_at`: ISO 8601 timestamp when the URL was last accessed

### 4. Delete Short URL

**DELETE** `/{short_code}`

Permanently deletes a shortened URL.

#### Parameters
- `short_code` (string, path): The short code to delete.

#### Response (204 No Content)
- **Body**: Empty

#### Example
```bash
curl -X DELETE http://localhost:8080/abc123
# Returns: HTTP/1.1 204 No Content
```

### 5. Health Check

**GET** `/health`

Returns the service health status.

#### Response (200 OK)
```json
{
  "status": "healthy",
  "timestamp": "2023-12-01T10:30:00Z"
}
```

## Rate Limiting

Currently no rate limiting is implemented. For production use, consider implementing rate limiting at the reverse proxy level.

## Interactive Documentation

Visit `/swagger-ui/` for interactive API documentation where you can test endpoints directly in your browser.

## SDKs and Examples

### JavaScript/Node.js
```javascript
// Create short URL
const response = await fetch('http://localhost:8080/shorten', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    url: 'https://example.com',
    custom_code: 'my-link'
  })
});
const data = await response.json();
console.log(data.short_url);
```

### Python
```python
import requests

# Create short URL
response = requests.post('http://localhost:8080/shorten', json={
    'url': 'https://example.com',
    'custom_code': 'my-link'
})
data = response.json()
print(data['short_url'])
```

### cURL Examples
```bash
# Create short URL
curl -X POST http://localhost:8080/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'

# Get statistics
curl http://localhost:8080/stats/abc123

# Test redirect
curl -I http://localhost:8080/abc123
``` 