#!/usr/bin/env python3
"""
Convert SDXL Turbo raw tokenizer files (vocab.json + merges.txt) into the
unified tokenizer.json format expected by the HuggingFace `tokenizers` Rust crate.

Zero external dependencies -- uses only Python stdlib.

Usage:
    python3 scripts/convert_tokenizer.py
"""

import json
import os
from pathlib import Path


def build_tokenizer_json(tokenizer_dir: Path) -> dict:
    """Build a HuggingFace tokenizers-compatible tokenizer.json from raw CLIP files."""

    vocab_path = tokenizer_dir / "vocab.json"
    merges_path = tokenizer_dir / "merges.txt"
    config_path = tokenizer_dir / "tokenizer_config.json"

    if not vocab_path.exists() or not merges_path.exists():
        raise FileNotFoundError(f"Missing vocab.json or merges.txt in {tokenizer_dir}")

    # Load vocab: token_string -> id
    with open(vocab_path, "r", encoding="utf-8") as f:
        vocab = json.load(f)

    # Load merges (skip the header line "#version: ...")
    with open(merges_path, "r", encoding="utf-8") as f:
        lines = f.read().strip().split("\n")
    merges = [line for line in lines if not line.startswith("#") and line.strip()]

    # Load config for special tokens
    config = {}
    if config_path.exists():
        with open(config_path, "r", encoding="utf-8") as f:
            config = json.load(f)

    # CLIP special tokens - build from parts to avoid model token interception
    SOT = "<|" + "startoftext" + "|>"
    EOT = "<|" + "endoftext" + "|>"

    bos_content = config.get("bos_token", SOT)
    eos_content = config.get("eos_token", EOT)
    pad_content = config.get("pad_token", EOT)

    bos_id = vocab.get(bos_content, 49406)
    eos_id = vocab.get(eos_content, 49407)

    # Build the added_tokens list
    added_tokens = [
        {
            "id": bos_id,
            "content": bos_content,
            "single_word": False,
            "lstrip": False,
            "rstrip": False,
            "normalized": True,
            "special": True
        },
        {
            "id": eos_id,
            "content": eos_content,
            "single_word": False,
            "lstrip": False,
            "rstrip": False,
            "normalized": True,
            "special": True
        }
    ]

    # Build the unified tokenizer.json
    tokenizer_json = {
        "version": "1.0",
        "truncation": None,
        "padding": None,
        "added_tokens": added_tokens,
        "normalizer": {
            "type": "Lowercase"
        },
        "pre_tokenizer": {
            "type": "Whitespace"
        },
        "post_processor": {
            "type": "TemplateProcessing",
            "single": [
                {"SpecialToken": {"id": bos_content, "type_id": 0}},
                {"Sequence": {"id": "A", "type_id": 0}},
                {"SpecialToken": {"id": eos_content, "type_id": 0}}
            ],
            "pair": [
                {"SpecialToken": {"id": bos_content, "type_id": 0}},
                {"Sequence": {"id": "A", "type_id": 0}},
                {"SpecialToken": {"id": eos_content, "type_id": 0}},
                {"SpecialToken": {"id": bos_content, "type_id": 1}},
                {"Sequence": {"id": "B", "type_id": 1}},
                {"SpecialToken": {"id": eos_content, "type_id": 1}}
            ],
            "special_tokens": {
                bos_content: {"id": bos_content, "ids": [bos_id], "tokens": [bos_content]},
                eos_content: {"id": eos_content, "ids": [eos_id], "tokens": [eos_content]}
            }
        },
        "decoder": {
            "type": "BPEDecoder",
            "suffix": "</w>"
        },
        "model": {
            "type": "BPE",
            "dropout": None,
            "unk_token": eos_content,
            "continuing_subword_prefix": "",
            "end_of_word_suffix": "</w>",
            "fuse_unk": False,
            "byte_fallback": False,
            "vocab": vocab,
            "merges": merges
        }
    }

    return tokenizer_json


def convert_dir(tokenizer_dir: Path):
    """Convert a single tokenizer directory."""
    output_path = tokenizer_dir / "tokenizer.json"

    if output_path.exists():
        print(f"  [SKIP] {output_path} already exists")
        return

    print(f"  [CONVERTING] {tokenizer_dir}")
    tokenizer_json = build_tokenizer_json(tokenizer_dir)

    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(tokenizer_json, f, indent=2, ensure_ascii=False)

    size_kb = output_path.stat().st_size / 1024
    print(f"  [OK] Written {output_path} ({size_kb:.0f} KB)")


def main():
    base = Path.home() / "trinity-models" / "sdxl-turbo-onnx"

    if not base.exists():
        print(f"ERROR: Model directory not found: {base}")
        return

    print(f"SDXL Turbo Tokenizer Converter")
    print(f"Base: {base}")
    print()

    for subdir in ["tokenizer", "tokenizer_2"]:
        d = base / subdir
        if d.exists():
            convert_dir(d)
        else:
            print(f"  [WARN] {d} not found, skipping")

    print()
    print("Done! The Rust `tokenizers` crate can now load these files.")


if __name__ == "__main__":
    main()
