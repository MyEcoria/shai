static COMPRESSION_SUMMARY_PROMPT: &str = r#"Generate a comprehensive summary of this conversation, focusing on capturing every detail necessary to seamlessly continue the work.

Your summary must preserve:
- All user requests and instructions (both completed and pending)
- Technical decisions, code patterns, and architectural choices
- The progression of work and problem-solving approaches

Structure your summary as follows:

**Context**

1. **Conversation Overview**: Provide a high-level narrative of the entire discussion, capturing the flow from initial objectives through all major topics and pivots.

2. **Current Work Status**: Describe in detail the most recent task being addressed. Focus particularly on the latest messages to capture the immediate context before this summary request.

3. **Technical Details**: Document all relevant technical elements including:
   - Technologies, frameworks, and libraries used
   - Coding conventions and patterns established
   - Architectural decisions and design principles
   - Configuration settings and environment details

4. **Files and Code References**: List all files, code sections, or resources that were:
   - Examined or reviewed
   - Modified or updated
   - Created from scratch
   Prioritize the most recently touched items.

5. **Problem-Solving History**: Summarize:
   - Issues that were identified and resolved
   - Solutions that were implemented
   - Debugging approaches that were attempted
   - Any ongoing troubleshooting efforts

6. **Outstanding Work and Next Actions**: 
   - List all pending tasks explicitly requested by the user
   - Outline the planned next steps for incomplete work
   - Include verbatim quotes from recent messages showing the exact task in progress and where it was paused
   - Add relevant code snippets where they provide clarity

Output only the summary itself, with no preamble or meta-commentary.

Conversation to summarize:
{}"#;

pub fn get_compression_summary_prompt(conversation_text: &str) -> String {
    COMPRESSION_SUMMARY_PROMPT.replace("{}", conversation_text)
}