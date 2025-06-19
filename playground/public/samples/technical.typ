#set page(margin: (x: 2cm, y: 2.5cm))
#set text(font: "Source Sans Pro", size: 10pt)
#set heading(numbering: "1.1")

#align(center)[
  #text(size: 24pt, weight: "bold")[REST API Documentation]

  #v(0.5cm)

  #text(size: 14pt)[Version 2.1.0 • Updated May 2025]
]

#outline()

#pagebreak()

= Overview

This document provides comprehensive documentation for the User Management REST API. The API follows RESTful principles and returns JSON-formatted responses.

== Base URL
```
https://api.example.com/v2
```

== Authentication
All requests require API key authentication via the `Authorization` header:

```http
Authorization: Bearer YOUR_API_KEY
```

== Rate Limiting
- *Standard users:* 100 requests per minute
- *Premium users:* 1000 requests per minute
- *Enterprise users:* 10000 requests per minute

#rect(
  width: 100%,
  fill: rgb("#fff3cd"),
  inset: 10pt,
  stroke: (left: 4pt + rgb("#856404"))
)[
  *Note:* Rate limits are enforced per API key. Exceeding limits returns HTTP 429.
]

= User Endpoints

== GET /users

Retrieve a list of users with optional filtering and pagination.

=== Parameters

#table(
  columns: (auto, auto, auto, 1fr),
  stroke: 0.5pt,
  [*Name*], [*Type*], [*Required*], [*Description*],
  [page], [integer], [No], [Page number (default: 1)],
  [limit], [integer], [No], [Items per page (max: 100, default: 20)],
  [status], [string], [No], [Filter by status: active, inactive, pending],
  [search], [string], [No], [Search users by name or email]
)

=== Example Request

```bash
curl -X GET "https://api.example.com/v2/users?page=1&limit=50&status=active" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json"
```

=== Example Response

```json
{
  "data": [
    {
      "id": 12345,
      "email": "john.doe@example.com",
      "name": "John Doe",
      "status": "active",
      "created_at": "2024-01-15T10:30:00Z",
      "last_login": "2024-03-10T14:22:15Z"
    }
  ],
  "pagination": {
    "current_page": 1,
    "total_pages": 25,
    "total_items": 1247,
    "items_per_page": 50
  }
}
```

== POST /users

Create a new user account.

=== Request Body

#table(
  columns: (auto, auto, auto, 1fr),
  stroke: 0.5pt,
  [*Field*], [*Type*], [*Required*], [*Description*],
  [email], [string], [Yes], [Valid email address (unique)],
  [name], [string], [Yes], [Full name (2-100 characters)],
  [password], [string], [Yes], [Password (min 8 characters)],
  [role], [string], [No], [User role: admin, user (default: user)]
)

=== Example Request

```bash
curl -X POST "https://api.example.com/v2/users" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "new.user@example.com",
    "name": "New User",
    "password": "securePassword123",
    "role": "user"
  }'
```

=== Response Codes

#table(
  columns: (auto, 1fr),
  stroke: 0.5pt,
  [*Code*], [*Description*],
  [201], [User created successfully],
  [400], [Invalid input data],
  [409], [Email already exists],
  [429], [Rate limit exceeded]
)

= Error Handling

The API uses standard HTTP status codes and returns detailed error information:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input data",
    "details": [
      {
        "field": "email",
        "message": "Invalid email format"
      }
    ]
  }
}
```

== Common Error Codes

#table(
  columns: (auto, auto, 1fr),
  stroke: 0.5pt,
  [*HTTP*], [*Code*], [*Description*],
  [400], [VALIDATION_ERROR], [Request data failed validation],
  [401], [UNAUTHORIZED], [Invalid or missing API key],
  [403], [FORBIDDEN], [Insufficient permissions],
  [404], [NOT_FOUND], [Resource not found],
  [429], [RATE_LIMITED], [Too many requests],
  [500], [INTERNAL_ERROR], [Server error]
)

= SDKs and Libraries

Official SDKs are available for popular programming languages:

- *JavaScript/Node.js:* `npm install @company/api-client`
- *Python:* `pip install company-api-client`
- *PHP:* `composer require company/api-client`
- *Go:* `go get github.com/company/api-client-go`

#rect(
  width: 100%,
  fill: rgb("#d1ecf1"),
  inset: 10pt,
  stroke: (left: 4pt + rgb("#0c5460"))
)[
  *Need Help?* Contact our developer support team at dev-support@example.com or visit our community forum at forum.example.com.
]
