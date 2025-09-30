static COMPRESSION_SUMMARY_PROMPT: &str = r#"Compress this conversation by eliminating ONLY redundant information while preserving every unique piece of data needed to continue the work.

## ORIGINAL OBJECTIVE
Reproduce the FIRST user message VERBATIM - do not summarize or modify it:
"""
[Insert complete first user message here]
"""

## CONVERSATION FACTS
Extract and organize ALL unique information from the conversation. If something was mentioned multiple times, include it only once. If it's unique information, include it even if it seems minor.

### Technical Stack & Architecture
[Every technology, library, framework, pattern, or architectural decision mentioned - list each once with version if specified]

### Files & Code
For each file mentioned:
- `filepath`: [what it does] | [changes made] | [current state] | [remaining work]

Include code snippets where they contain decisions, patterns, or solutions that need to be preserved.

### User Requests & Instructions
List chronologically, one per line, using EXACT quotes:
1. "[exact user request]" → Status: [completed/in-progress/pending] → [deliverable or progress made]
2. "[exact user request]" → Status: [completed/in-progress/pending] → [deliverable or progress made]
[continue for all requests]

### Technical Decisions & Solutions
[Every problem solved, decision made, or approach chosen - include the reasoning if it was discussed]
- [Decision/Solution]: [context] → [implementation] → [outcome]

### Configuration & Environment
[Everything about setup, environment variables, dependencies, compilation flags, etc. - each item once]

### Coding Conventions & Patterns
[Any established patterns for naming, structure, error handling, testing, etc. that were agreed upon]

### Current State & Progress
**Most recent exchange**:
- Last user message: "[exact quote]"
- What was being done: [precise description]
- Exact stopping point: [file, function, line of code, or specific action]

**State of work**:
[For each active work item: what's done, what's in progress, what remains]

### Outstanding Work (In Order)
**Next immediate action**: [specific next step with file/function names]

**Remaining tasks**:
1. [task] - [any context needed]
2. [task] - [any context needed]
[ordered by priority or logical sequence]

**Deferred/Future**: [anything explicitly postponed or lower priority]

### Important Constraints & Notes
[User preferences, requirements, known bugs, warnings, gotchas - anything that affects how to proceed]

---

**Compression instructions**:
- Keep the FIRST user message completely intact
- Include information ONCE - if repeated in conversation, write it only once
- Use exact quotes for user requests - never paraphrase what the user asked for
- Include ALL unique technical details, decisions, and code patterns
- Preserve ALL code snippets that show solutions or patterns
- Be dense but complete - no fluff, but don't omit facts
- Organize logically but don't create redundant categories
- Most recent context is most critical - ensure it's captured precisely

Conversation to summarize:
{}"#;

pub fn get_compression_summary_prompt(conversation_text: &str) -> String {
    COMPRESSION_SUMMARY_PROMPT.replace("{}", conversation_text)
}