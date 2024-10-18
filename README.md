# scheduler_rs

## Overview

The Rust Task Scheduler is a simple library for managing and executing tasks concurrently in Rust. It allows you to control the maximum number of tasks that can run simultaneously, ensuring efficient resource usage in multi-threaded applications.

## Features

- **Concurrent Task Execution**: Specify the maximum number of tasks that can run at the same time.
- **Task Management**: Schedule tasks to be executed based on available bandwidth.
- **Thread Safety**: Utilizes `Arc` and `Mutex` to ensure safe access to shared state across multiple threads.


