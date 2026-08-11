# `mcp/invalid-cursor` — Invalid pagination cursor

- **type**: `https://github.com/zircote/nsip/blob/main/docs/reference/errors/mcp/invalid-cursor.md`
- **status**: 400 · **exit_code**: 1 · **class**: caller
- **`suggested_fix` applicability**: `machine_applicable`
- **`retry_after`**: never

## When it occurs

A paginated MCP list request (`prompts/list`, `resources/list`,
`resources/templates/list`) was given a `cursor` that could not be decoded
as an offset, or whose offset was past the end of the result set. Cursors
are opaque tokens returned in a prior page's `nextCursor`; they must not be
synthesized or mutated. (JSON-RPC code: `invalid_params` / -32602.)

`tools/list` is not paginated -- it always returns the complete tool set in
a single response and never validates a `cursor`, so this error cannot occur
there. See [Pagination](../../../MCP.md#pagination) for the full contract.

## Recovery

Restart pagination without a cursor (begin from the first page), then follow
the `nextCursor` values verbatim. Deterministic — an agent may drop the bad
cursor and retry.

## Example

```json
{
  "type": "https://github.com/zircote/nsip/blob/main/docs/reference/errors/mcp/invalid-cursor.md",
  "title": "Invalid pagination cursor",
  "status": 400,
  "detail": "validation error: cursor could not be decoded as an offset",
  "instance": "urn:nsip:resources/list:5f6a7b8c-9d0e-1f2a-3b4c-5d6e7f8a9b0c",
  "exit_code": 1,
  "suggested_fix": "restart pagination without a cursor (begin from the first page)",
  "docs_url": "https://github.com/zircote/nsip/blob/main/docs/reference/errors/mcp/invalid-cursor.md"
}
```
