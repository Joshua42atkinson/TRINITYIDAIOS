#!/bin/bash
export PYTORCH_ALLOC_CONF="expandable_segments:True,garbage_collection_threshold:0.7,max_split_size_mb:128"
/opt/venv/bin/python /home/joshua/Workflow/desktop_trinity/trinity-genesis/scripts/quantize_gemma_E2B.py
