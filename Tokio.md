Tokio Manages Non-Blocking Threads by Swapping Current Process between Core Thread & Blocking Thread

Tokio is able to concurrently run many tasks on a few threads by repeatedly swapping the currently running task on each thread.
- However, this kind of swapping can only happen at .await points
- Thus, code that spends a long time without reaching an .await will prevent other tasks from running. 

This is where the Core & blocking threads address this issue.
The core threads are where all asynchronous code runs, and Tokio will by default spawn one for each CPU core.

You can use the environment variable TOKIO_WORKER_THREADS to override the default value.
