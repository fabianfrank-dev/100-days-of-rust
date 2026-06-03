# Day 5: State & Collections

## Projects
- **Inventory**: Built a simple inventory using a `Vec<String>` with add, remove, and print actions.
- **Mini Notepad**: Created notes and used actions to create, view, and delete them.

## Learnings
✅ Storing data in vectors
✅ Iterating with `.iter()` and `.enumerate()`
✅ Searching with `.find()`
✅ Removing items by index and with `.retain()`
✅ Cloning data when enum actions need owned values
✅ Modeling app behavior with enums and `match`

## Challenges
- **Finding before removing**: Tracked an index first, then removed the item after iteration.
- **Mutable state**: Practiced passing `&mut Vec<Note>` into methods that change stored data.
