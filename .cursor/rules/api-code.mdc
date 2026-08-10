---
description: "API 规范 — RESTful 标准、版本管理、错误格式、幂等性、分页、速率限制"
globs: "src/api/**,docs/api/**"
alwaysApply: false
---
# API 规范

- 遵循 RESTful 标准
  - 使用正确的 HTTP 方法：GET 查询、POST 创建、PUT 整体更新、PATCH 部分更新、DELETE 删除
  - 使用正确的状态码：200 成功、201 创建、204 无内容、400 参数错误、401 未认证、403 无权限、404 不存在、409 冲突、422 校验失败、500 服务器错误
  - 禁止用 GET 执行写操作，禁止用 POST 执行删除
- API 必须版本管理
  - 路径版本：`/api/v1/resources`
  - 新版本不破坏旧版本契约
- 统一错误响应格式
  - 所有错误返回统一 JSON 结构：`{ code, message, details, requestId }`
- 幂等性设计
  - 写操作应支持幂等键（`Idempotency-Key` 头）
  - 重复请求返回相同结果，不产生副作用
- 请求与响应 Schema 验证
  - 使用 JSON Schema 或 Zod / Joi 校验入参
  - 禁止未经验证的数据进入业务层
- 分页标准统一
  - 使用 `cursor` 游标分页或 `page/pageSize` 偏移分页
  - 响应包含分页元信息：`{ items, nextCursor, hasMore }`
- 速率限制
  - 公开端点必须配置速率限制
  - 响应头包含 `X-RateLimit-Limit` / `X-RateLimit-Remaining` / `X-RateLimit-Reset`

## 示例

**正确**：
```http
POST /api/v1/orders HTTP/1.1
Content-Type: application/json
Idempotency-Key: 9b1d2f3a-...

Response: 201 Created
{
  "data": { "id": "ord_123", "status": "pending" },
  "requestId": "req_abc"
}
```

**错误**：
```http
POST /api/createOrder  (非 RESTful、无版本)
Response: 200 OK        (创建应返回 201)
{ "error": "something wrong" }  (非标准错误格式)
```