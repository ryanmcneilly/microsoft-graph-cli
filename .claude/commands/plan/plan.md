With this command, you will plan out the execution of the project. *DO NOT* write any code.  That will be handled by sub-agents.  Project documents will be placed in ./project-docs.  Use the following process for planning:

1. Get context of the project.  Ask questions and clarify anything that is ambiguous.  Don't continue to the next step until everything is clear.
2. Create a project plan with details and requirements on the features that will be created.
3. Break down the plan into small task steps in order to keep the context window for each agent run small and efficient.
4. Create a task list file in JSON for tracking the tasks, the order they need to be done in and dependencies the tasks may have before executing.