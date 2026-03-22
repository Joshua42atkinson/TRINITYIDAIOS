# trinity-sidecar-engineer

[![Rust](https://img.shields.io/badge/rust-stable-blue.svg)](https://rust-lang.org)
[![License: MIT](https://imgields.io/badge/license-MIT-blue.svg)](https://github.com/trinity-ai/trinity/blob/main/LICENSE)

## Overview

The **Engineer Sidecar** is a specialized component within the Trinity AI system, designed to execute software engineering tasks autonomously. It operates as one of the five roles in the Trinity Party: Engineer, Evaluator, Artist, Brakeman, and Pete.

### Trinity Party Concept

The Trinity Party represents a collaborative AI architecture where each role contributes uniquely:

- **Engineer**: Implements code solutions
- **Evaluator**: Reviews and validates work
- **Artist**: Handles creative and design tasks
- **Brakeman**: Ensures safety and compliance
- **Pete**: Coordinates and manages workflows

### Sword & Shield Dual-Model Architecture

The system employs a dual-model architecture:

- **Sword (REAP 25B)**: Fast, efficient code generation engine
- **Shield (Opus 27B)**: Strategic planning, analysis, and review capabilities

These models work in tandem during quest execution, with the Shield providing strategic direction and the Sword implementing solutions.

## Architecture