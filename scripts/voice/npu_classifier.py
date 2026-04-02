#!/usr/bin/env python3
"""
NPU Classifier Service - Always-on task router
Routes simple queries to NPU (fast), complex queries to sidecar (accurate)
"""

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import uvicorn
import os

app = FastAPI(title="Trinity NPU Classifier")

class QueryRequest(BaseModel):
    message: str
    max_tokens: int = 512

class ClassificationResponse(BaseModel):
    complexity: str  # "simple" or "complex"
    confidence: float
    route_to: str  # "npu" or "sidecar"
    reasoning: str

@app.post("/classify", response_model=ClassificationResponse)
async def classify_query(request: QueryRequest):
    """
    Classify query complexity and route appropriately.
    
    Simple (NPU): greetings, basic questions, short responses
    Complex (Sidecar): code generation, analysis, multi-step reasoning
    """
    message = request.message.lower()
    
    # Simple heuristics for now (replace with NPU model later)
    simple_patterns = [
        "hello", "hi", "hey", "thanks", "thank you",
        "what is", "who is", "when is", "where is",
        "yes", "no", "ok", "okay"
    ]
    
    complex_patterns = [
        "write", "create", "build", "implement", "fix",
        "analyze", "explain", "debug", "refactor",
        "generate", "design", "architect"
    ]
    
    # Check for simple patterns
    is_simple = any(pattern in message for pattern in simple_patterns)
    is_complex = any(pattern in message for pattern in complex_patterns)
    
    # Length heuristic
    word_count = len(message.split())
    if word_count < 10 and not is_complex:
        is_simple = True
    
    # Classify
    if is_complex:
        return ClassificationResponse(
            complexity="complex",
            confidence=0.85,
            route_to="sidecar",
            reasoning="Contains code/analysis keywords"
        )
    elif is_simple:
        return ClassificationResponse(
            complexity="simple",
            confidence=0.75,
            route_to="npu",
            reasoning="Short query or greeting pattern"
        )
    else:
        # Default to sidecar for safety
        return ClassificationResponse(
            complexity="medium",
            confidence=0.60,
            route_to="sidecar",
            reasoning="Uncertain - routing to sidecar for accuracy"
        )

@app.get("/health")
async def health():
    return {"status": "healthy", "service": "npu-classifier"}

if __name__ == "__main__":
    port = int(os.getenv("NPU_CLASSIFIER_PORT", "8099"))
    uvicorn.run(app, host="0.0.0.0", port=port)
