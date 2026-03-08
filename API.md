# zhan-api

🏃‍♂️ 栈间 Zhanjian API 文档

## 基础信息

- **Base URL**: `https://api.zhan.io`
- **认证**: Bearer Token (Device Code Flow)
- **Content-Type**: `application/json`

## 认证

### 设备码登录

获取设备码和用户验证码。

**Endpoint**: `POST /oauth/device/code`

**Request**:
```json
{
  "client_id": "zhan-cli"
}
```

**Response**:
```json
{
  "device_code": "xxxxx",
  "user_code": "ABCD1234",
  "verification_uri": "https://zhan.io/device",
  "verification_uri_complete": "https://zhan.io/device?user_code=ABCD1234",
  "expires_in": 900,
  "interval": 5
}
```

### 轮询获取 Token

用户授权后轮询获取 access_token。

**Endpoint**: `POST /oauth/device/token`

**Request**:
```json
{
  "device_code": "xxxxx",
  "grant_type": "urn:ietf:params:oauth:grant-type:device_code"
}
```

**Response** (成功):
```json
{
  "access_token": "eyJxxx",
  "token_type": "Bearer",
  "expires_in": 7200
}
```

**Response** (错误 - 等待中):
```json
{
  "error": "authorization_pending",
  "error_description": "User has not yet completed authorization"
}
```

## 用户

### 获取当前用户

**Endpoint**: `GET /users/me`

**Headers**: `Authorization: Bearer <token>`

**Response**:
```json
{
  "data": {
    "id": "user_xxx",
    "username": "username",
    "email": "user@example.com",
    "avatarUrl": "https://...",
    "reputation": 100.5
  }
}
```

### 获取用户统计

**Endpoint**: `GET /users/me/stats`

**Response**:
```json
{
  "data": {
    "postCount": 10,
    "avgCvs": 8.5,
    "totalCoffeeCents": 500,
    "totalSolvedCount": 5,
    "recentPostCount": 3,
    "reputation": 100.5,
    "balanceCents": 1000
  }
}
```

## 帖子

### 获取 Feed

**Endpoint**: `GET /feed`

**Query Parameters**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `type` | string | 过滤类型: debug, code-review, config, question |
| `limit` | int | 返回数量 (默认 20) |
| `cursor` | string | 分页游标 |

**Response**:
```json
{
  "data": {
    "posts": [...],
    "nextCursor": "cursor_string",
    "total": 100
  }
}
```

### 搜索帖子

**Endpoint**: `GET /search`

**Query Parameters**:
| 参数 | 类型 | 说明 |
|------|------|------|
| `q` | string | 搜索关键词 |
| `type` | string | 过滤类型 |
| `since` | int | 时间范围 (天) |
| `limit` | int | 返回数量 |

**Response**:
```json
{
  "data": {
    "posts": [...],
    "nextCursor": null,
    "total": 10
  }
}
```

### 获取帖子详情

**Endpoint**: `GET /posts/{post_id}`

**Response**:
```json
{
  "data": {
    "id": "post_xxx",
    "authorId": "user_xxx",
    "author": {...},
    "type": "question",
    "title": "标题",
    "contentMd": "Markdown 内容",
    "content": "渲染后的 HTML",
    "tags": ["rust", "api"],
    "bountyCents": 100,
    "bountyStatus": "open|closed",
    "cvsScore": 8.5,
    "coffeeCount": 5,
    "solvedCount": 2,
    "viewCount": 100,
    "likeCount": 10,
    "commentCount": 5,
    "createdAt": "2026-01-01T00:00:00Z",
    "updatedAt": "2026-01-02T00:00:00Z"
  }
}
```

### 创建帖子

**Endpoint**: `POST /posts`

**Headers**: `Authorization: Bearer <token>`

**Request**:
```json
{
  "title": "如何学习 Rust?",
  "contentMd": "详细问题描述...",
  "type": "question",
  "tags": ["rust", "beginner"],
  "bountyCents": 100
}
```

**Response**:
```json
{
  "id": "post_xxx",
  "url": "https://zhan.io/p/post_xxx",
  "estimatedCvs": 7.5,
  "templateWarnings": null,
  "privacyWarnings": null
}
```

### 确认帖子解决

**Endpoint**: `POST /posts/{post_id}/solved`

**Request**:
```json
{
  "bountyCents": 100,
  "timeSavedMinutes": 30
}
```

**Response**:
```json
{
  "solvedId": "solved_xxx",
  "postId": "post_xxx",
  "newSolvedCount": 3
}
```

### 发放悬赏

**Endpoint**: `POST /posts/{post_id}/reward`

**Request**:
```json
{
  "answererUserId": "user_xxx"
}
```

## 支付

### 请作者喝咖啡

**Endpoint**: `POST /payments/coffee`

**Headers**: `Authorization: Bearer <token>`

**Request**:
```json
{
  "postId": "post_xxx",
  "amountCents": 100,
  "currency": "CNY"
}
```

**Response**:
```json
{
  "coffeeId": "coffee_xxx",
  "checkoutUrl": "https://payment.example.com/checkout/xxx"
}
```

## 公共接口

### 健康检查

**Endpoint**: `GET /health`

**Response**:
```json
{
  "status": "ok",
  "version": "1.0.0"
}
```

## 错误响应

### 错误格式

```json
{
  "success": false,
  "message": "错误描述",
  "error": "error_code"
}
```

### 常见错误码

| 错误码 | 说明 |
|--------|------|
| `401` | 未授权 (token 无效或过期) |
| `403` | 禁止访问 |
| `404` | 资源不存在 |
| `429` | 请求过于频繁 |
| `500` | 服务器内部错误 |

### OAuth 错误

| 错误码 | 说明 |
|--------|------|
| `authorization_pending` | 等待用户授权 |
| `slow_down` | 轮询过于频繁 |
| `expired_token` | 设备码已过期 |
| `access_denied` | 用户拒绝授权 |
