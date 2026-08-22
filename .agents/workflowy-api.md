# Nodes

A node is the fundamental unit of content in WorkFlowy. Each node
represents a single bullet point that can contain text, have child
nodes, and be organized hierarchically. Nodes can be expanded or
collapsed, completed, tagged, and moved around to create flexible
outlines and lists.

{% examples %}

```endpoints
POST   /api/nodes
POST   /api/nodes/:id
GET    /api/nodes/:id
GET    /api/nodes
DELETE /api/nodes/:id
```

{% /examples %}

---

## The Node object

## Attributes

- **`id`** *string*

  Unique identifier of the node.

- **`name`** *string*

  The text content of the node. This is the main bullet text that appears in your outline. Supports the following inline HTML tags:

  - `<b>text</b>`

    Bold

  - `<i>text</i>`

    Italic

  - `<s>text</s>`

    Strikethrough

  - `<code>text</code>`

    Inline code

  - `<a href="url">text</a>`

    Hyperlink

- **`data.layoutMode`** *string*

  Display mode of the node.

  - `"bullets"`

    Bullet point (default)

  - `"todo"`

    Todo item. Completion state is tracked by `completedAt`

  - `"h1"`

    Level 1 header

  - `"h2"`

    Level 2 header

  - `"h3"`

    Level 3 header

  - `"code-block"`

    Code block

  - `"quote-block"`

    Quote block

- **`note`** *string | null*

  Additional note content for the node. Notes appear below the main text and can contain extended descriptions or details.

- **`priority`** *number*

  Sort order of the node among its siblings. Lower values appear first.

- **`createdAt`** *number*

  Unix timestamp indicating when the node was created.

- **`modifiedAt`** *number*

  Unix timestamp indicating when the node was last modified.

- **`completedAt`** *number | null*

  Unix timestamp indicating when the node was completed. `null` if the node is not completed.

{% examples %}

```label
The Node object
```

```json
{
  "id": "6ed4b9ca-256c-bf2e-bd70-d8754237b505",
  "name": "This is a test outline for API examples",
  "note": null,
  "priority": 200,
  "data": {
    "layoutMode": "bullets"
  },
  "createdAt": 1753120779,
  "modifiedAt": 1753120850,
  "completedAt": null
}
```

{% /examples %}

---

## Create a node

### Parameters

- **`parent_id`** *string*

  Where to create the new node. Calendar nodes are created on demand if they don't exist yet.

  - `"6ed4b9ca-256c-bf2e-bd70-d8754237b505"`

    the `id` of any node you own

  - `"rd"`

    a shortcut key you've defined to point at a node

  - `"None"`

    the top level of your outline (root)

  - `"inbox"`

    your Inbox node, the built-in destination for quickly captured items at the top of your outline

  - `"calendar"`

    the root of your calendar

  - `"today"`

    the calendar node for today's date

  - `"tomorrow"`

    the calendar node for tomorrow's date

  - `"next_week"`

    the calendar node for the first day of next week, based on your week-start-day setting

  - `"YYYY"` (e.g. `"2026"`)

    a year node in the calendar

  - `"YYYY-MM"` (e.g. `"2026-01"`)

    a month node in the calendar

  - `"YYYY-MM-DD"` (e.g. `"2026-01-15"`)

    a day node in the calendar

- **`name`** *string* *required*

  The text content of the new node.

  **Multiline text** — when the `name` field contains multiple lines, the first line becomes the parent node and subsequent lines become child nodes. A single `\n` is joined into a space; use `\n\n` (double newline) to create separate children.

  **Inline styles** — markdown is parsed into HTML tags:

  | Markdown | HTML | Result |
        |---|---|---|
  | `**text**` | `<b>text</b>` | **bold** |
  | `*text*` | `<i>text</i>` | *italic* |
  | `~~text~~` | `<s>text</s>` | ~~strikethrough~~ |

  | `` `text` `` | `<code>text</code>` | inline code |
  | `[text](url)` | `<a href="url">text</a>` | hyperlink |
  | `[YYYY-MM-DD]` | — | date |
  | `[YYYY-MM-DD HH:MM]` | — | date with time (24-hour, user's timezone) |

  **Node layout** — markdown prefix sets the node's `layoutMode`:

  | Markdown | layoutMode | Result |
        |---|---|---|
  | `# text` | `"h1"` | level 1 header |
  | `## text` | `"h2"` | level 2 header |
  | `### text` | `"h3"` | level 3 header |
  | `- text` | `"bullets"` | bullet point (default) |
  | `- [ ] text` | `"todo"` | uncompleted todo |
  | `- [x] text` | `"todo"` | completed todo |

  | `` ```code``` `` | `"code-block"` | code block |
  | `> text` | `"quote-block"` | quote block |

- **`layoutMode`** *string*

  The display mode of the node. See the `name` field above for the markdown equivalents.

  - `"bullets"`

    Bullet point (default)

  - `"todo"`

    Todo item. Completion state is tracked by `completedAt`

  - `"h1"`

    Level 1 header

  - `"h2"`

    Level 2 header

  - `"h3"`

    Level 3 header

  - `"code-block"`

    Code block

  - `"quote-block"`

    Quote block

- **`note`** *string*

  Additional note content for the node. Notes appear below the main text.

- **`position`** *string*

  Where to place the new node. Options: `"top"` (default) or `"bottom"`.

{% examples %}

```endpoint
POST /api/v1/nodes
```

```curl
curl -X POST https://workflowy.com/api/v1/nodes \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <YOUR_API_KEY>" \
  -d '{
    "parent_id": "inbox",
    "name": "Hello API",
    "position": "top"
  }' | jq
```

```response
{
  "item_id": "5b401959-4740-4e1a-905a-62a961daa8c9"
}
```

{% /examples %}

---

## Update a node

Updates the specified node by setting the values of the parameters passed. Any parameters not provided will be left unchanged.

### Parameters

- **`id`** *string* *required*

  The identifier of the node to update.

- **`name`** *string*

  The text content of the node.

  **Inline styles** — markdown is parsed into HTML tags:

  | Markdown | HTML | Result |
        |---|---|---|
  | `**text**` | `<b>text</b>` | **bold** |
  | `*text*` | `<i>text</i>` | *italic* |
  | `~~text~~` | `<s>text</s>` | ~~strikethrough~~ |

  | `` `text` `` | `<code>text</code>` | inline code |
  | `[text](url)` | `<a href="url">text</a>` | hyperlink |
  | `[YYYY-MM-DD]` | — | date |
  | `[YYYY-MM-DD HH:MM]` | — | date with time (24-hour, user's timezone) |

  **Node layout** — markdown prefix sets the node's `layoutMode`:

  | Markdown | layoutMode | Result |
        |---|---|---|
  | `# text` | `"h1"` | level 1 header |
  | `## text` | `"h2"` | level 2 header |
  | `### text` | `"h3"` | level 3 header |
  | `- text` | `"bullets"` | bullet point (default) |
  | `- [ ] text` | `"todo"` | uncompleted todo |
  | `- [x] text` | `"todo"` | completed todo |

  | `` ```code``` `` | `"code-block"` | code block |
  | `> text` | `"quote-block"` | quote block |

- **`layoutMode`** *string*

  The display mode of the node. See the `name` field above for the markdown equivalents.

  - `"bullets"`

    Bullet point (default)

  - `"todo"`

    Todo item. Completion state is tracked by `completedAt`

  - `"h1"`

    Level 1 header

  - `"h2"`

    Level 2 header

  - `"h3"`

    Level 3 header

  - `"code-block"`

    Code block

  - `"quote-block"`

    Quote block

- **`note`** *string*

  The note content of the node.

{% examples %}

```endpoint
POST /api/v1/nodes/:id
```

```curl
curl -X POST https://workflowy.com/api/v1/nodes/:id \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <YOUR_API_KEY>" \
  -d '{
    "name": "Updated node title"
  }' | jq
```

```response
{
  "status": "ok"
}
```

{% /examples %}

---

## Retrieve a node

Retrieves the details of an existing node. Supply the unique node ID and WorkFlowy will return the corresponding node information.

### Parameters

- **`id`** *string* *required*

  The identifier of the node to retrieve.

{% examples %}

```endpoint
GET /api/v1/nodes/:id
```

```curl
curl -X GET https://workflowy.com/api/v1/nodes/:id \
  -H "Authorization: Bearer <YOUR_API_KEY>" | jq
```

```response
{
  "node": {
    "id": "6ed4b9ca-256c-bf2e-bd70-d8754237b505",
    "name": "This is a test outline for API examples",
    "note": null,
    "priority": 200,
    "data": {
      "layoutMode": "bullets"
    },
    "createdAt": 1753120779,
    "modifiedAt": 1753120850,
    "completedAt": null
  }
}
```

{% /examples %}

---

## List nodes

Returns a list of child nodes for a given parent. The nodes are returned unordered - you need to sort them yourself based on the `priority` field.

### Parameters

- **`parent_id`** *string*

  Whose children to list. Calendar keys return 404 if the node hasn't been created yet; use Create or Move to materialize it first.

  - `"6ed4b9ca-256c-bf2e-bd70-d8754237b505"`

    the `id` of any node you own.

  - `"rd"`

    a shortcut key you've defined to point at a node.

  - `"None"`

    the top level of your outline (root).

  - `"inbox"`

    your Inbox node, the built-in destination for quickly captured items at the top of your outline.

  - `"calendar"`

    the root of your calendar.

  - `"today"`

    the calendar node for today's date.

  - `"tomorrow"`

    the calendar node for tomorrow's date.

  - `"next_week"`

    the calendar node for the first day of next week, based on your week-start-day setting.

  - `"YYYY"` (e.g. `"2026"`)

    a year node in the calendar.

  - `"YYYY-MM"` (e.g. `"2026-01"`)

    a month node in the calendar.

  - `"YYYY-MM-DD"` (e.g. `"2026-01-15"`)

    a day node in the calendar.

{% examples %}

```endpoint
GET /api/v1/nodes
```

```curl
curl -G https://workflowy.com/api/v1/nodes \
  -H "Authorization: Bearer <YOUR_API_KEY>" \
  -d "parent_id=inbox" | jq
```

```response
{
  "nodes": [
    {
      "id": "ee1ac4c4-775e-1983-ae98-a8eeb92b1aca",
      "name": "Bullet A",
      "note": null,
      "priority": 100,
      "data": {
        "layoutMode": "bullets"
      },
      "createdAt": 1753120787,
      "modifiedAt": 1753120815,
      "completedAt": null
    }
  ]
}
```

{% /examples %}

---

## Delete a node

Permanently deletes a node. This cannot be undone.

### Parameters

- **`id`** *string* *required*

  The identifier of the node to delete.

{% examples %}

```endpoint
DELETE /api/v1/nodes/:id
```

```curl
curl -X DELETE https://workflowy.com/api/v1/nodes/:id \
  -H "Authorization: Bearer <YOUR_API_KEY>" | jq
```

```response
{
  "status": "ok"
}
```

{% /examples %}

---

## Move a node

### Parameters

- **`parent_id`** *string*

  The new parent for the node. Calendar nodes are created on demand if they don't exist yet.

  - `"6ed4b9ca-256c-bf2e-bd70-d8754237b505"`

    the `id` of any node you own.

  - `"rd"`

    a shortcut key you've defined to point at a node.

  - `"None"`

    the top level of your outline (root).

  - `"inbox"`

    your Inbox node, the built-in destination for quickly captured items at the top of your outline.

  - `"calendar"`

    the root of your calendar.

  - `"today"`

    the calendar node for today's date.

  - `"tomorrow"`

    the calendar node for tomorrow's date.

  - `"next_week"`

    the calendar node for the first day of next week, based on your week-start-day setting.

  - `"YYYY"` (e.g. `"2026"`)

    a year node in the calendar.

  - `"YYYY-MM"` (e.g. `"2026-01"`)

    a month node in the calendar.

  - `"YYYY-MM-DD"` (e.g. `"2026-01-15"`)

    a day node in the calendar.

- **`position`** *string*

  Where to place the node. Options: `"top"` (default) or `"bottom"`.

{% examples %}

```endpoint
POST /api/v1/nodes/:id/move
```

```curl
curl -X POST https://workflowy.com/api/v1/nodes/<NODE_ID>/move \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <YOUR_API_KEY>" \
  -d '{
    "parent_id": "inbox",
    "position": "top"
  }' | jq
```

```response
{
  "status": "ok"
}
```

{% /examples %}

---

## Complete a node

Marks a node as completed. This sets the completion timestamp and updates the node's status.

### Parameters

- **`id`** *string* *required*

  The identifier of the node to complete.

{% examples %}

```endpoint
POST /api/v1/nodes/:id/complete
```

```curl
curl -X POST https://workflowy.com/api/v1/nodes/:id/complete \
  -H "Authorization: Bearer <YOUR_API_KEY>" | jq
```

```response
{
  "status": "ok"
}
```

{% /examples %}

---

## Uncomplete a node

Marks a node as not completed. This removes the completion timestamp and updates the node's status.

### Parameters

- **`id`** *string* *required*

  The identifier of the node to uncomplete.

{% examples %}

```endpoint
POST /api/v1/nodes/:id/uncomplete
```

```curl
curl -X POST https://workflowy.com/api/v1/nodes/:id/uncomplete \
  -H "Authorization: Bearer <YOUR_API_KEY>" | jq
```

```response
{
  "status": "ok"
}
```

{% /examples %}

---

## Export all nodes

Returns all user's nodes as a flat list. Each node includes its `parent_id` field to reconstruct the hierarchy. The nodes are returned unordered - you need to build the tree structure yourself based on the `parent_id` and `priority` fields.

**Note:** This endpoint has a rate limit of 1 request per minute due to the potentially large response size.

{% examples %}

```endpoint
GET /api/v1/nodes-export
```

```curl
curl https://workflowy.com/api/v1/nodes-export \
  -H "Authorization: Bearer <YOUR_API_KEY>" | jq
```

```response
{
  "nodes": [
    {
      "id": "ee1ac4c4-775e-1983-ae98-a8eeb92b1aca",
      "name": "Top Level Item",
      "note": "This is a note",
      "parent_id": null,
      "priority": 100,
      "completed": false,
      "data": {
        "layoutMode": "bullets"
      },
      "createdAt": 1753120787,
      "modifiedAt": 1753120815,
      "completedAt": null
    },
    {
      "id": "ff2bd5d5-886f-2094-bf09-b9ffa93c2bdb",
      "name": "Child Item",
      "note": null,
      "parent_id": "ee1ac4c4-775e-1983-ae98-a8eeb92b1aca",
      "priority": 200,
      "completed": false,
      "data": {
        "layoutMode": "bullets"
      },
      "createdAt": 1753120820,
      "modifiedAt": 1753120830,
      "completedAt": null
    }
  ]
}
```

{% /examples %}

---

## Targets

Targets provide quick access to specific nodes in your outline. They include both system targets (like "inbox") and custom shortcuts you create (like "home").

Learn more about shortcuts in the [shortcuts documentation](https://workflowy.com/learn/shortcuts/).

{% examples %}

```endpoints
GET /api/v1/targets
```

{% /examples %}

---

## The Target object

### Attributes

- **`key`** *string*

  The unique identifier for this target (e.g., "home", "inbox", "today").

- **`type`** *string*

  The type of target:
  - `"shortcut"` - User-defined shortcuts.
  - `"system"` - System-managed locations like inbox. Always returned, even if the target node hasn't been created yet.

- **`name`** *string | null*

  The name of the node that this target points to. Returns `null` only for system targets when the target node hasn't been created yet.

{% examples %}

```label
User-defined shortcut
```

```json
{
  "key": "home",
  "type": "shortcut",
  "name": "My Home Page"
}
```

```label
System target (before node created)
```

```json
{
  "key": "inbox",
  "type": "system",
  "name": null
}
```

{% /examples %}

---

## List targets

Returns all available targets, including user-defined shortcuts (like "home") and system targets (like "inbox").

### Parameters

No parameters required.

{% examples %}

```endpoint
GET /api/v1/targets
```

```curl
curl https://workflowy.com/api/v1/targets \
  -H "Authorization: Bearer <YOUR_API_KEY>" | jq
```

```response
{
  "targets": [
    {
      "key": "home",
      "type": "shortcut",
      "name": "My Home Page"
    },
    {
      "key": "inbox",
      "type": "system",
      "name": "Inbox"
    }
  ]
}
```

{% /examples %}
