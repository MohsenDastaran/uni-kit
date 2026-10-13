---
title: MessageRole
description: The marker a conversation turn shows for the side it came from.
---

# MessageRole

`MessageRole` is which side of a conversation a turn came from, and the marker
that side shows. It is not the turn itself: it owns no body, header, timestamp,
or footer. The Slint authority, `message-role.slint`, declares one thing —
`export enum MessageRole { user, assistant }` — and is shared by `Message` and
`Bubble` rather than drawn, so every value the role decides is decided in the
two components that consume it. This port draws what those two derive from the
role alone, and nothing else:

- **The marker a turn shows.** `message.slint` gives both sides an avatar at
  `AvatarSize.medium`; a sent turn fills it with `primary`, takes
  `primary-foreground` initials, and borders it with the page background, while
  a received turn fills it with `muted` and takes `muted-foreground`. A sent
  turn that failed swaps both to `destructive`.
- **The side it sits on.** `Role::User` is the sent turn at the trailing edge;
  `Role::Assistant` is the received turn at the leading edge.
- **The initials it falls back to.** An empty caller string reads as `"YO"` for
  a sent turn and `"AL"` for a received one.

Two things are reproduced from the authority rather than corrected, because it
is the authority: a failed sent turn's initials are drawn in `destructive` —
the fill itself, not `destructive-foreground` — and a received turn stays
`muted` whatever `failed` says, since `message.slint` applies its tint only to
the sent side. The marker is a fixed 32-pixel circle, the `AvatarSize.medium`
both Slint consumers pick, with the avatar's one-pixel border and no shadow.

Because a role is a value, the marker cannot place itself. `Role::alignment()`
returns the edge the caller puts it at, and the application composes its own
`Message` row around that; the marker itself has no alignment property.

GPUI has no separate `MessageRole` component, so this page shows a Slint and
iced view only. A GPUI `Message` is placed with a `MessageAlignment`, and its
accessibility role is announced through `Message::role(...)`; the sent/received
marker described here belongs to the `Message` and `Bubble` surfaces.

## Import

## Usage

The marker is built from the role. `MessageRole` defaults `Role::Assistant`,
which is what `message.slint` defaults both `Message` and `Bubble` to, so a
received turn needs nothing but the role. The initials drawn are the role's
fallback when the caller names none.

The marker is a presentational element. It does not own the message body, the
sender, a timestamp, a delivery footer, or any action; compose those around it
with the existing `Message`, `Bubble`, and `Button` components.

### Custom Initials

The caller's own initials replace the fallback. The role still chooses the fill
and the colour the initials are drawn in, so a two-party transcript stays
legible with a real name behind each side.

`initials(...)` accepts any string. The empty string is read the same way
`message.slint` reads it — as no name at all — so `"".into()` shows the role's
fallback rather than an empty disc. `shown_initials()` returns what is actually
drawn.

### Failed State

A sent turn that failed to send is tinted destructively, which is the one state
the marker itself reports. A received turn is muted whatever `failed` says,
because `message.slint` uses its `tint` only on the sent side.

The destructive tint is a colour, not an announcement. Keep the failure in text
or another non-colour cue as well, and offer a retry action in the surrounding
`Message` row.

### Alignment

A marker cannot place itself, so the side each role sits on is a value the
application reads and applies. `Role::alignment()` returns `Alignment::End` for
a sent turn and `Alignment::Start` for a received one, and the marker is placed
inside a container aligned to that edge.

`Role::is_sent()` answers the same question as a boolean for layout decisions,
and `Role::default_initials()` returns the fallback without constructing a
marker.

## API reference

### `MessageRole`

| Method | Default | Purpose |
| --- | --- | --- |
| `new(role)` | `Role::Assistant` | Create the marker for a conversation side. |
| `initials(initials)` | the role's fallback | Set the initials to draw; the empty string keeps the fallback. |
| `failed(bool)` | `false` | Tint a sent turn destructively; no effect on a received turn. |
| `shown_initials()` | the fallback or the caller's string | Read the initials actually drawn. |

`MessageRole` converts into an `Element`, so it can be placed directly in a
layout.

### `Role`

| Method | Purpose |
| --- | --- |
| `is_sent()` | Whether this is the side that sent the turn. |
| `alignment()` | `End` for a sent turn, `Start` for a received one. |
| `default_initials()` | `"YO"` for a sent turn, `"AL"` for a received one. |
| `fill(tokens, failed)` | The marker's fill: `primary`, `destructive` when failed, or `muted`. |
| `ink(tokens, failed)` | The initials' colour: `primary-foreground`, or `destructive` when failed; `muted-foreground` on a received turn. |
| `marker_border(tokens)` | The marker's border, drawn in the page background. |

### `Style` and the marker's geometry

`Style::new(role).failed(failed).resolve(tokens)` returns the resolved
`container::Style` the marker draws: the fill, the one-pixel border, and the
`edge / 2` circle.

| Item | Value | Meaning |
| --- | --- | --- |
| `MARKER_EDGE` | `32.0` | The marker's diameter in pixels, the `AvatarSize.medium` of every turn. |
| `MARKER_BORDER_WIDTH` | `1.0` | The avatar's own border width. |

### Related components

- `Message` — the row that owns sender identity, metadata, content, and footer,
  and places a role's marker at its edge.
- `Bubble` — the content surface the role also selects a variant for.
- `Avatar` — the component `message.slint` takes the marker's
  `AvatarSize.medium` geometry from.
